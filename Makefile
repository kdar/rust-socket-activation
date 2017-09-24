build:
	cargo build

install-systemd:
	cat etc/rust-socket-activation.service |sed "s+EXECSTART+$(PWD)/target/debug/rust-socket-activation+" > /tmp/rust-socket-activation.service
	sudo cp /tmp/rust-socket-activation.service /etc/systemd/system
	sudo cp etc/rust-socket-activation.socket /etc/systemd/system
	sudo systemctl daemon-reload

uninstall-systemd: stop
	sudo rm /etc/systemd/system/rust-socket-activation.service
	sudo rm /etc/systemd/system/rust-socket-activation.socket

start-systemd: stop-systemd
	sudo systemctl start rust-socket-activation.socket
	sudo systemctl start rust-socket-activation.service

stop-systemd:
	sudo systemctl stop rust-socket-activation.socket
	sudo systemctl stop rust-socket-activation.service

status-systemd:
	sudo systemctl status rust-socket-activation.service --no-pager

connect:
	nc -v localhost 8080

socketmaster:
	socketmaster -listen=tcp4://localhost:8080 -command=./target/debug/rust-socket-activation -start=1000