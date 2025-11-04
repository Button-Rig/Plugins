set -Eeuxo pipefail

if [ -z "$1" ]; then
	echo "Error: No input provided." >&2
	exit 1
fi

windows() {
    rustup target add x86_64-pc-windows-msvc
    cargo build --release --target x86_64-pc-windows-msvc
    cp ./target/x86_64-pc-windows-msvc/release/handler-cli ../plugin-src/static/handler-cli-x86_64-windows.exe
}

linux() {
    rustup target add x86_64-unknown-linux-gnu
    cargo build --release --target x86_64-unknown-linux-gnu
    cp ./target/x86_64-unknown-linux-gnu/release/handler-cli ../plugin-src/static/handler-cli-x86_64-linux
}

mac() {
    rustup target add aarch64-apple-darwin
    cargo build --release --target aarch64-apple-darwin
    cp ./target/aarch64-apple-darwin/release/handler-cli ../plugin-src/static/handler-cli-arm-mac
}

case "$1" in
"windows")
	windows
	;;
"linux")
    linux
	;;
"mac")
	mac
	;;
"all")
    windows
    linux
    mac
	;;
*)
	echo "Error: Invalid input. Expected 'windows', 'linux', 'mac' or 'all'" >&2
	exit 1
	;;
esac
