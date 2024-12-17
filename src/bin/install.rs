use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::str;

fn get_macos_version() -> Option<String> {
    if let Ok(file) = File::open("/System/Library/CoreServices/SystemVersion.plist") {
        let reader = BufReader::new(file);

        // Iterate over each line in the file
        for (index, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                // Look for line 14
                if index == 13 {
                    let version_line = line.trim_start(); // Remove leading whitespace
                    if version_line.starts_with("<string>") && version_line.ends_with("</string>") {
                        let version = version_line.trim_start_matches("<string>").trim_end_matches("</string>").trim();
                        return Some(version.to_string());
                    }
                }
            }
        }
    }
    None
}

fn split_version(version: &str) -> (u32, u32, u32) {
    let mut parts = version.split('.').map(|s| s.parse::<u32>().unwrap_or(0));
    let major = parts.next().unwrap_or(0);
    let minor = parts.next().unwrap_or(0);
    let fix = parts.next().unwrap_or(0);
    (major, minor, fix)
}

fn get_version_name(major: u32, minor: u32) -> &'static str {
    match major {
        10 => match minor {
            0 => "Cheetah",
            1 => "Puma",
            2 => "Jaguar",
            3 => "Panther",
            4 => "Tiger",
            5 => "Leopard",
            6 => "Snow Leopard",
            7 => "Lion",
            8 => "Mountain Lion",
            9 => "Mavericks",
            10 => "Yosemite",
            11 => "El Capitan",
            12 => "Sierra",
            13 => "High Sierra",
            14 => "Mojave",
            15 => "Catalina",
            _ => "Unknown",
        },
        11 => "Big Sur",
        12 => "Monterey",
        13 => "Ventura",
        14 => "Sonoma",
        15 => "Sequoia"
        _ => "Unknown",
    }
}


fn main() {
    if let Some(mac_os_version) = get_macos_version() {
        let (major, minor, _) = split_version(&mac_os_version);
        let version_name = get_version_name(major, minor);
        println!("\x1B[32m[DEBUG]\x1B[0m macOS Version: {} ({})", version_name, mac_os_version);
    } else {
        eprintln!("Failed to get the macOS version.");
        return;
    }

    let output = Command::new("uname").arg("-m").output();
    if let Ok(output) = output {
        let arch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("\x1B[32m[DEBUG]\x1B[0m Architecture: {}", arch);

        match arch.as_str() {
            "x86_64" => {
                println!("\x1B[36m==>\x1B[0m Downloading a temporary git.");
                Command::new("curl").args(&["-O", "https://cdndwnld.invra.net/macos/automation/deps/git-x86_64.tar.gz"]).status().unwrap();
                Command::new("tar").args(&["-xvf", "git-x86_64.tar.gz"]).status().unwrap();

                let pwd_output = Command::new("pwd").output().expect("Failed to execute command");
                let pwd_string = String::from_utf8(pwd_output.stdout).expect("Failed to convert to String").trim().to_string();
                println!("Current working directory: {}", pwd_string);

                let git_clone_status = Command::new("git").args(&["clone", "https://github.com/InvraNet/macos-installer"]).status();
                if let Err(err) = git_clone_status {
                    eprintln!("Failed to clone repository: {}", err);
                    return;
                }

                let cd_status = Command::new("sh").args(&["-c", &format!("cd {}/macos-installer", pwd_string)]).status();
                if let Err(err) = cd_status {
                    eprintln!("Failed to change directory: {}", err);
                    return;
                }
            }
            "arm64" => {
                Command::new("curl").args(&["-O", "https://cdndwnld.invra.net/macos/automation/deps/git.tar.gz"]).status().unwrap();
            }
            _ => {
                println!("We have run into an issue! The architecture '{}' is not supported. The only architectures to be supported by macOS or casually supported is x86_64, or arm64.", arch);
            }
        }
    } else {
        eprintln!("Failed to get architecture information.");
    }
}
