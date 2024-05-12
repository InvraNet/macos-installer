#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdio>
#include <algorithm>
#include <sys/utsname.h>

std::string getMacOSVersion() {
    std::string version;
    std::ifstream plistFile("/System/Library/CoreServices/SystemVersion.plist");
    std::string line;

    if (plistFile.is_open()) {
        while (std::getline(plistFile, line)) {
            size_t keyPos = line.find("<key>ProductVersion</key>");
            if (keyPos != std::string::npos) {
                // Found the line with ProductVersion key
                std::getline(plistFile, line); // Read the next line containing the version
                size_t stringStartPos = line.find("<string>");
                size_t stringEndPos = line.find("</string>");
                if (stringStartPos != std::string::npos && stringEndPos != std::string::npos) {
                    // Extract the version substring between <string> tags
                    version = line.substr(stringStartPos + 8, stringEndPos - stringStartPos - 8);
                    break;
                }
            }
        }
        plistFile.close();
    } else {
        std::cerr << "Failed to open SystemVersion.plist." << std::endl;
    }

    return version;
}

void splitVersion(const std::string& version, int& major, int& minor, int& fix) {
    std::istringstream iss(version);
    std::string token;
    
    if (std::getline(iss, token, '.')) {
        major = std::stoi(token);
    }
    if (std::getline(iss, token, '.')) {
        minor = std::stoi(token);
    }
    if (std::getline(iss, token, '.')) {
        fix = std::stoi(token);
    }
}

std::string getVersionName(int major, int minor) {
    switch (major) {
        case 10:
            switch (minor) {
                case 0: return "Cheetah";
                case 1: return "Puma";
                case 2: return "Jaguar";
                case 3: return "Panther";
                case 4: return "Tiger";
                case 5: return "Leopard";
                case 6: return "Snow Leopard";
                case 7: return "Lion";
                case 8: return "Mountain Lion";
                case 9: return "Mavericks";
                case 10: return "Yosemite";
                case 11: return "El Capitan";
                case 12: return "Sierra";
                case 13: return "High Sierra";
                case 14: return "Mojave";
                case 15: return "Catalina";
                default: return "Unknown";
            }
        case 11: return "Big Sur";
        case 12: return "Monterey";
        case 13: return "Ventura";
        case 14: return "Sonoma";
        default: return "Unknown";
    }
}

int main() {
    std::string macOSVersion = getMacOSVersion();
    int major = 0, minor = 0, fix = 0;
    splitVersion(macOSVersion, major, minor, fix);
    if (!macOSVersion.empty()) {
        std::string versionName = getVersionName(major, minor);
        std::cout << "\x1B[32m[DEBUG]\x1B[0m macOS Version: " << versionName << " (" << macOSVersion << ")" << std::endl;
    } else {
        std::cerr << "Failed to get the macOS version." << std::endl;
    }

    struct utsname sysInfo;
if (uname(&sysInfo) != -1) {
    std::string arch = sysInfo.machine;
    std::cout << "\x1B[32m[DEBUG]\x1B[0m Architecture: " << arch << std::endl;
    if (arch == "x86_64") {
        std::cout << major;
    } else if (arch == "arm64") {
        
    } else if ((major == 10 && minor <= 15)) {
        std::cout << "Your macOS version is not supported anymore. And also is a security vulnerability. Please think of upgrading if you are able to." << std::endl;
        return 0;
    } else {
        std::cout << "We have run into an issue! The architecture '" << arch << "' is not supported. The only architectures to be supported by macOS or casually supported is x86_64, or arm64." << std::endl;
    }
    } else {
    std::cerr << "Failed to get architecture information." << std::endl;
    }


    return 0;
}
