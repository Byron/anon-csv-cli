from guangie88/rustfmt-clippy:nightly

run cargo install hyperfine watchexec xsv

run apt-get update
run apt-get install -y valgrind

env PATH=$PATH:/root/.cargo/bin
