# InvraNet's macOS Automated Install
This will automate everything needed to install to make macOS a usable OS.

## What is to install
* Web browser
* Network concious tools
* Xcode Command Line Tools
* Brew
* Kitty
* Git & Github-CLI
* My Dotfiles. (Work in progress, does not have my dotfiles as automation yet.)

## Usage
Please note, this tool is still in the works. There will be a config file system added later, so this usage notation may be completley different.
### Slower setup (setup a Mac you will use)
```sh
cargo build
cargo run
```
This will run the tool as usual.

```sh
sh install.sh
```
This will run a script which will install Rust if it is not installed, then run the tool.
### You can't use many tools before automation
```sh
/bin/bash -c "$(curl -fsSL https://cdndwnld.invra.net/macos/automation/get-and-install.sh)"
```
This command will grab the deployment script and will automatically execute it. The following command is how you can inspect the file you are downloading.

```sh
curl https://cdndwnld.invra.net/macos/automation/get-and-install.sh
```
Inspect the code of what you are running with this.

#### Chucking this file on a USB for mass deployment.
```sh
curl -O https://cdndwnld.invra.net/macos/automation/get-and-install.sh
mv get-and-install.sh /your/mount/location
```
Two commands which will download the automated script and move to your specified location.

This tool is:
Managed and maintained by [InvraNet](https://invra.net).