#! /bin/zsh

if [[ $(uname -m) == 'arm64' ]]; then
  curl -O https://cdxn.invra.net/macos/automation/deps/git-intel-uni.pkg && \
  sudo installer -pkg git-intel-uni.pkg -target /
  git clone https://github.com/InvraNet/macos-installer 
  echo 'Cleaning up before we continue...'
  rm git-intel-uni.pkg
  cd ./macos-installer
else
  curl -O https://cdxn.invra.net/macos/automation/deps/git-intel-uni.pkg && \
  sudo installer -pkg git-intel-uni.pkg -target /
  git clone https://github.com/InvraNet/macos-installer 
  echo 'Cleaning up before we continue...'
  rm git-intel-uni.pkg
  cd ./macos-installer
fi

sh install.sh