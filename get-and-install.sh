#! /bin/zsh

if [[ $(uname -m) == 'arm64' ]]; then
  curl -O https://cdxn.invra.net/gh/gh_2.49.1_macOS_arm64.tar.xz && tar -xvf gh_2.49.1_macOS_arm64.tar && \
  gh_2.49.1_macOS_arm64/bin/gh repo clone InvraNet/macos-installer 
  echo 'Cleaning up before we continue...'
  rm gh_2.49.1_macOS_arm64.tar.xz
  rm -rf gh_2.49.1_macOS_arm64
  cd ./macos-installer
else
  curl -O https://cdxn.invra.net/gh/gh_2.49.1_macOS_amd64.tar.xz && tar -xvf gh_2.49.1_macOS_amd64.tar && \
  gh_2.49.1_macOS_amd64/bin/gh repo clone InvraNet/macos-installer 
  echo 'Cleaning up before we continue...'
  rm gh_2.49.1_macOS_amd64.tar.xz
  rm -rf gh_2.49.1_macOS_amd64
  cd ./macos-installer
fi

sh install.sh