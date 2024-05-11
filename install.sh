#! /bin/zsh

if command -v rustc &> /dev/null
then
  cargo run
  exit 1
fi
curl https://sh.rustup.rs -sSf | sh
