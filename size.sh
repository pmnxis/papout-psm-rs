!/bin/sh
echo "Release Size"
cargo size --bin papout-psm-rs --release -- -A

echo ""
echo "Debug Size"
cargo size --bin papout-psm-rs -- -A