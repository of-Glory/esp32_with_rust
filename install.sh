curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

if [ $? -eq 0 ]; then
    cargo install ldproxy
    cargo install espup && espup install
    # ex) cargo espflash --example=blinky --release --monitor
    cargo install cargo-espflash
    # ex) espflash build/blinky
    cargo install espflash
    cargo install cargo-espmonitor
    cargo install espmonitor
else
    echo "failed cargo install. Please reinstall rust."
fi
