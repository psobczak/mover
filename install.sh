#!/bin/bash
cargo build --release && cp ./target/release/mover ~/.local/bin/mover
echo "@reboot ~/.local/bin/mover" >> file; crontab file; rm file;