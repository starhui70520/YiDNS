use std::mem::size_of;

use std::fs::File;
use std::io::{self, BufReader, Lines};
use std::collections::HashMap;

#[cfg(target_os = "windows")]
use winapi::um::winnt::RTL_OSVERSIONINFOW;

#[cfg(target_os = "windows")]
#[link(name = "ntdll")]
unsafe extern "system" {
    fn RtlGetVersion(lpVersionInformation: *mut RTL_OSVERSIONINFOW) -> i32;
}

#[cfg(target_os = "windows")]
pub fn windows_version() -> String {
    unsafe {
        let mut os_info: RTL_OSVERSIONINFOW = std::mem::zeroed();
        os_info.dwOSVersionInfoSize = size_of::<RTL_OSVERSIONINFOW>() as u32;

        let status = RtlGetVersion(&mut os_info);
        if status != 0 {
            return "Unknown".to_string();
        }

        let build_version = os_info.dwBuildNumber;
        match build_version {
            22000..=u32::MAX => "11".to_string() + "." + &build_version.to_string(),         // Windows 11: build_version >= 22000
            15000..=21999 => "10".to_string() + "." + &build_version.to_string(),            // Windows 10: build_version 在 15000 到 21999 之间
            9200..=14999 => "8".to_string() + "." + &build_version.to_string(),              // Windows 8: build_version 在 9200 到 14999 之间
            7600..=9199 => "7".to_string() + "." + &build_version.to_string(),               // Windows 7: build_version 在 7600 到 9199 之间
            6000..=7599 => "Vista".to_string() + "." + &build_version.to_string(),           // Windows Vista: build_version 在 6000 到 7599 之间
            5000..=5999 => "XP".to_string() + "." + &build_version.to_string(),              // Windows XP: build_version 在 5000 到 5999 之间
            _ => "Unknown".to_string() + "." + &build_version.to_string(),                   // 如果不在上述范围内，返回 "Unknown"
        }
    }
}

#[cfg(target_os = "linux")]
fn read_os_release(file_path: &str, flag: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        
        // 检查是否匹配要找的字段
        if !line.starts_with(&format!("{}=", flag)) {
            continue;
        }

        if let Some(value) = line.split('=').nth(1) {
            return Ok(value.trim_matches('"').to_string());
        }
    }
    
    Ok(String::new())
}

#[cfg(target_os = "linux")]
fn linux_id() -> String {
    match read_os_release("/etc/os-release", "ID") {
        Ok(id) => id,
        Err(_) => "unknown".to_string(),
    }
}

#[cfg(target_os = "linux")]
pub fn linux_version() -> String {
    match read_os_release("/etc/os-release", "VERSION_ID") {
        Ok(version) => version,
        Err(_) => "unknown".to_string(),
    }
}

#[cfg(target_os = "macos")]
pub fn macos_version() -> String {
    use std::process::Command;

    match Command::new("sw_vers")
        .arg("-productVersion")
        .output()
    {
        Ok(output) => {
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string()
        }
        Err(_) => "unknown".to_string(),
    }
}

pub fn os_type() -> String {
    #[cfg(target_os = "windows")]
    return "Windows".to_string();

    #[cfg(target_os = "macos")]
    return "macOS".to_string();

    #[cfg(target_os = "linux")]
    return linux_id();

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "Unknown".to_string();
}

pub fn os_version() -> String {
    #[cfg(target_os = "windows")]
    return windows_version();
    
    #[cfg(target_os = "macos")]
    return macos_version();
    
    #[cfg(target_os = "linux")]
    return linux_version();
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "Unknown".to_string();
}