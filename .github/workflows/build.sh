if [ "$RUNNER_OS" == "Linux" ]
then
 sudo apt install -y libdbus-1-dev pkg-config
fi

cargo build --release --verbose -j 2