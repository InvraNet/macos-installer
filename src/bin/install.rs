use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command; // Import Command for process execution

fn get_macos_version() -> Option<String> {
    if let Ok(file) = File::open("/System/Library/CoreServices/SystemVersion.plist") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            if line.contains("<key>ProductVersion</key>") {
                if let Some(version) = line.strip_prefix("<string>") {
                    if let Some(version) = version.strip_suffix("</string>") {
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
    }

    // Get system architecture using the 'uname' command
    let output = Command::new("uname").arg("-m").output();
    if let Ok(output) = output {
        let arch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("\x1B[32m[DEBUG]\x1B[0m Architecture: {}", arch);

        match arch.as_str() {
            "x86_64" => {
                println!("\x1B[36m==>\x1B[0m Downloading a temporary git.");
                Command::new("curl").args(&["-O", "https://cdndwnld.invra.net/macos/automation/deps/git.tar.gz"]).status().unwrap();
                Command::new("tar").args(&["-xvf", "git.tar.gz"]).status().unwrap();
                println!("\x1B[36m==>\x1B[0m Compiling git.");
                Command::new("sh").arg("-c").arg("cd git-2.35.1 && make").status().unwrap();
                Command::new("sh").arg("-c").arg("cd ../ && mv git-2.35.1 git-temp").status().unwrap();
                Command::new("sh").arg("-c").arg("git-temp/git clone ").status().unwrap();
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
