if [ "$(id -u)" != "0" ]; then
   echo "This script must be run as root" 1>&2
   exit 1
fi

cargo build --release

cp target/release/qpass /usr/bin/qpass
chmod +x /usr/bin/qpass

echo "qpass has been installed.🔥"   