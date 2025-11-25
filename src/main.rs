mod system;
mod config;


fn main() {
    let config_path = "config.ini";
    match config::read_config(config_path) {
        Ok(config) => {
            println!("配置文件内容:");
            for (key, value) in config {
                println!("{} = {}", key, value);
            }
        }
        Err(e) => {
            eprintln!("读取配置文件时出错: {}", e);
        }
    }
}
