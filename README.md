rust-socket-activation
=========================

Example of using socket activation with rust and tokio. You can use systemd or (socketmaster)[https://github.com/zimbatm/socketmaster]

## Usage with systemd

```bash
make build
make install-systemd
make start
```

## Usage with socketmaster

```bash
make build
make socketmaster
```

## Makefile

| command | description |
| --------|-------------|
| make build | Builds the application |
| make install-systemd | Installs the systemd socket and service |
| make uninstall-systemd | Uninstalls the systemd socket and service |
| make start-systemd | Starts the systemd socket and service |
| make stop-systemd | Stops the systemd socket and service |
| make status-systemd | Gets the service status |
| make connect | Connects to application via netcat |
| make socketmaster | Runs the application under socketmaster |


## Tested with

rustc 1.22.0-nightly (14039a42a 2017-09-22)

## TODO

Add signal handling with tokio-signal
