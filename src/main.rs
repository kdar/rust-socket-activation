#![feature(conservative_impl_trait)]

extern crate env_logger;
extern crate futures;
extern crate systemd;
extern crate tokio_core;
extern crate tokio_io;

use std::env;
use std::net::SocketAddr;
use std::error::Error;
use std::process::exit;
use std::collections::HashMap;

use futures::{Future, Stream};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_io::io::copy;
use tokio_io::AsyncRead;
use systemd::daemon;

fn get_socket_fd() -> Option<i32> {
  match daemon::listen_fds(false) {
    Ok(0) => if let Ok(v) = env::var("EINHORN_FDS") {
      v.parse().ok()
    } else {
      None
    },
    Ok(1) => Some(daemon::LISTEN_FDS_START),
    Ok(n) => {
      println!("error: expected only 1 FD from systemd, got: {}", n);
      exit(-1);
    }
    Err(e) => {
      println!("{}", e);
      exit(-1);
    }
  }
}

fn run() -> Result<(), impl Error> {
  env_logger::init().unwrap();
  let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
  let addr = addr.parse::<SocketAddr>().unwrap();

  let mut core = Core::new()?;
  let handle = core.handle();
  let listener = match get_socket_fd() {
    None => {
      let l = TcpListener::bind(&addr, &core.handle())?;
      let addr = l.local_addr()?;
      println!("Listening for connections on {}", addr);
      l
    }
    Some(fd) => {
      let fd_listener = daemon::tcp_listener(fd)?;
      let l = TcpListener::from_listener(fd_listener, &addr, &handle)?;
      println!(
        "Listening for connections on FD({})",
        daemon::LISTEN_FDS_START,
      );
      let mut m = HashMap::new();
      m.insert("READY", "1");
      daemon::notify(false, m)?;
      l
    }
  };

  let server = listener.incoming().for_each(move |(socket, addr)| {
    let (reader, writer) = socket.split();
    let amt = copy(reader, writer);

    let msg = amt.then(move |result| {
      match result {
        Ok((amt, _, _)) => println!("wrote {} bytes to {}", amt, addr),
        Err(e) => println!("error on {}: {}", addr, e),
      }

      Ok(())
    });

    handle.spawn(msg);

    Ok(())
  });

  core.run(server)
}

fn main() {
  match run() {
    Ok(_) => (),
    Err(e) => println!("{}", e),
  };
}
