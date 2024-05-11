use std::fs;

use std::process::{Command, exit};
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
struct Package {
    name: String,
    installer: String,
    friendly_name: Option<String>,
}

fn is_installed(package: &str, installer: &str, _friendly_name: &str) -> bool {
    match installer {
        "xcode-clt" => {
            if let Ok(output) = Command::new("xcode-select").arg("-p").output() {
                return output.status.success();
            } else {
                return false;
            }
        }           
        "brew" => {
            if package == "nodejs" {
                return Path::new("/usr/local/Cellar/node").exists();
            } else if _friendly_name == "non-cask" {
                return Path::new(&("/usr/local/Caskroom/".to_owned() + package)).exists();
            }
            else {
                return Path::new(&("/usr/local/Cellar/".to_owned() + package)).exists();
            }
        }
        "brewCask" | "installer" => {
            let lowercase_package = package.to_lowercase();
            if let Ok(entries) = fs::read_dir("/Applications") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(filename) = entry.file_name().into_string().ok() {
                            let filename = filename.to_lowercase();
                            if filename.contains(&lowercase_package) {
                                return true;
                            }
                        }
                    }
                }
            }
            return false;
        }
        "mas" => {
            let output = Command::new("mas").arg("list").output().unwrap();
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let id = line.split_whitespace().next().unwrap_or("");
                if id == package {
                    return true;
                }
            }
            return false;
        }
        _ => return false,
    }
}

fn install_pkg(installer: &str, pkg: &str, friendly_name: &str) {
    match installer {
        "fetch" => {
            match pkg {
                "homebrew" => {
                    if !Path::new("/usr/local/Homebrew").exists() {
                        println!("\x1B[92m     Installing\x1B[0m Homebrew...");
                        Command::new("sh")
                            .arg("-c")
                            .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)")
                            .status()
                            .unwrap_or_else(|_| {
                                eprintln!("Failed to install Homebrew");
                                exit(1);
                            });
                    } else {
                        println!("\x1B[33m\t[INFO]\x1B[0m {} is already installed.", pkg);
                    }
                }
                "xcode-clt" => {
                    if !is_installed("xcode-clt", "xcode-clt", "") {
                        println!("\x1B[92m     Installing\x1B[0m Xcode-CLT...");
                        Command::new("sh")
                            .arg("-c")
                            .arg("xcode-select --install")
                            .status()
                            .unwrap_or_else(|_| {
                                eprintln!("Failed to install Xcode-CLT");
                                exit(1);
                            });
                    } else {
                        println!("\x1B[33m\t[INFO]\x1B[0m {} is already installed.", pkg);
                    }
                }
                _ => {}
            }
        }
        "brew" | "brewCask" | "installer" | "mas" => {
            if is_installed(pkg, installer, friendly_name) {
                println!("\x1B[33m\t[INFO]\x1B[0m {} is already installed.", pkg);
            } else {
                match installer {
                    "brew" => {
                        println!("\x1B[92m     Installing\x1B[0m {}...", pkg);
                        Command::new("brew")
                            .arg("install")
                            .arg(pkg)
                            .status()
                            .unwrap_or_else(|_| {
                                eprintln!("Failed to install {}", pkg);
                                exit(1);
                            });
                    }
                    "brewCask" => {
                        println!("\x1B[92m     Installing\x1B[0m {}...", pkg);
                        Command::new("brew")
                            .arg("install")
                            .arg("--cask")
                            .arg(pkg)
                            .status()
                            .unwrap_or_else(|_| {
                                eprintln!("Failed to install {}", pkg);
                                exit(1);
                            });
                    }
                    "installer" => {
                        println!("\x1B[92m     Installing\x1B[0m {}...", pkg);
                        // Replace this line with the actual installation command using 'installer'
                    }
                    "mas" => {
                        println!("\x1B[92m     Installing\x1B[0m {}...", pkg);
                        Command::new("mas")
                            .arg("install")
                            .arg(pkg)
                            .status()
                            .unwrap_or_else(|_| {
                                eprintln!("Failed to install {}", pkg);
                                exit(1);
                            });
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}


fn parse_package_list(filename: &str) -> Result<(), String> {
    let content = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read {}: {}", filename, err))?;
    let packages: Vec<Package> = toml::from_str(&content)
        .map_err(|err| format!("Failed to parse {}: {}", filename, err))?;
    for package in packages {
        install_pkg(&package.installer, &package.name, &package.friendly_name.unwrap_or_default());
    }

    Ok(())
}

fn main() {
    println!("\x1B[36m===>\x1B[0m Installing dependencies ...");
    install_pkg("fetch", "xcode-clt", "");
    install_pkg("fetch", "homebrew", "");
    install_pkg("brew", "wget", "");
    println!("\n\x1B[36m===>\x1B[0m Installing Packages...");
    install_pkg("brew", "gsl", "");
    install_pkg("brew", "llvm", "");
    install_pkg("brew", "boost", "");
    install_pkg("brew", "libomp", "");
    install_pkg("brew", "jq", "");
    install_pkg("brew", "mas", "");
    install_pkg("brew", "gh", "");
    install_pkg("brew", "ifstat", "");
    install_pkg("brew", "skhd", "");
    install_pkg("brew", "sketchybar", "");
    install_pkg("brew", "borders", "");
    install_pkg("brew", "yabai", "");
    install_pkg("brew", "zsh-autosuggestions", "");
    install_pkg("brew", "zsh-fast-syntax-highlighting", "");
    install_pkg("brew", "lulu", "non-cask");
    install_pkg("brew", "btop", "");
    install_pkg("brew", "neovim", "");
    install_pkg("brew", "lazygit", "");
    install_pkg("brew", "dooit", "");
    install_pkg("brewCask", "kitty", "");
//    if let Err(err) = parse_package_list("packagelist.toml") {
//        eprintln!("{}", err);
//        exit(1);
//    }
}