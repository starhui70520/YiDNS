use std::mem::size_of;

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
        if status == 0 {
            let build_version = os_info.dwBuildNumber;
            match build_version {
                22000..=u32::MAX => "11".to_string(),         // Windows 11: build_version >= 22000
                15000..=21999 => "10".to_string(),            // Windows 10: build_version 在 15000 到 21999 之间
                9200..=14999 => "8".to_string(),              // Windows 8: build_version 在 9200 到 14999 之间
                7600..=9199 => "7".to_string(),               // Windows 7: build_version 在 7600 到 9199 之间
                6000..=7599 => "Vista".to_string(),           // Windows Vista: build_version 在 6000 到 7599 之间
                5000..=5999 => "XP".to_string(),              // Windows XP: build_version 在 5000 到 5999 之间
                _ => "Unknown".to_string(),                   // 如果不在上述范围内，返回 "Unknown"
            }
        } else {
            "Unknown".to_string()
        }
    }
}

pub fn os_type() -> String {
    if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macOS".to_string()
    } else if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn os_version() -> String {
    #[cfg(target_os = "windows")]
    return windows_version();
    
    #[cfg(target_os = "macos")]
    return macos_version();
    
    #[cfg(target_os = "linux")]
    return linux_version();
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return None;
}