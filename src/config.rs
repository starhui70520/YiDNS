use std::fs;
use std::path::Path;    
use std::io::Result;
use std::collections::HashMap;

pub fn check_config(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);

    // 检查文件是否存在
    if path.exists() {
        println!("文件正常: {}", file_path);
        return Ok(());
    } 

    init_config(file_path)?;
    Ok(())
}

pub fn init_config(file_path: &str) -> Result<()> {
    // INI 格式的默认内容
    let default_content = "
        [settings]\n
        domain = example.local # custom domain, e.g., example.local\n
        deviceid = 0 # custom device ID\n
        groupid = 0 # custom group ID\n";

    // 写入文件内容
    fs::write(file_path, default_content)?;
    println!("文件已创建: {}", file_path);
    Ok(())
}

pub fn read_config(file_path: &str) -> Result<HashMap<String, String>> {
    check_config(file_path)?;
    let content = fs::read_to_string(file_path)?;
    let mut config_map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        // 跳过注释和空行
        if line.starts_with('#') || line.is_empty() || line.starts_with('[') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            // 移除值中的注释
            let value = value.split('#').next().unwrap_or("").trim();
            config_map.insert(key.trim().to_string(), value.to_string());
        }
    }
    Ok(config_map)
}
