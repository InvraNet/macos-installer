#! /bin/zsh

if [[ $(uname -m) == 'arm64' ]]; then
  curl -O https://cdxn.invra.net/gh/gh_2.49.1_macOS_arm64.tar.xz && unxz -vf gh_2.49.1_macOS_arm64.tar.xz && tar -xvf gh_2.49.1_macOS_arm64.tar
  exec gh_2.49.1_macOS_arm64/bin/gh repo clone InvraNet/macos-installer && cd ./macos-installer
  exit 1
else
  curl -O https://cdxn.invra.net/gh/gh_2.49.1_macOS_amd64.tar.xz && unxz -vf gh_2.49.1_macOS_amd64.tar.xz && tar -xvf gh_2.49.1_macOS_amd64.tar
  exec gh_2.49.1_macOS_amd64/bin/gh repo clone InvraNet/macos-installer && cd ./macos-installer
  exit 1
fi
then
if command -v cargo &> /dev/null
then
  cargo run
  exit 1
fi

curl https://sh.rustup.rs -sSf | sh
