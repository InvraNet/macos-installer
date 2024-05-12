#! /bin/zsh

if [[ -f "../get-and-install.sh" ]]; then
  rm -vf "../get-and-install.sh"
  echo 'Finished cleaning up.'
fi

if ! command -v cargo &> /dev/null; then
  curl https://sh.rustup.rs -sSf | sh
  exec zsh
fi

cargo build && cargo run