use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub struct IpcServer {
    rx: Receiver<String>,
}

impl IpcServer {
    pub fn new() -> Option<Self> {
        let socket_path = Self::socket_path()?;

        let _ = fs::remove_file(&socket_path);

        let listener = UnixListener::bind(&socket_path).ok()?;

        let (tx, rx) = channel();

        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let mut buffer = [0u8; 1024];
                    if let Ok(n) = stream.read(&mut buffer) {
                        if let Ok(msg) = String::from_utf8(buffer[..n].to_vec()) {
                            let _ = tx.send(msg.trim().to_string());
                        }
                    }
                }
            }
        });

        Some(IpcServer { rx })
    }

    pub fn check_message(&self) -> Option<String> {
        self.rx.try_recv().ok()
    }

    fn socket_path() -> Option<PathBuf> {
        Some(PathBuf::from("/tmp/memwatch.sock"))
    }
}

pub fn send_toggle_command() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = PathBuf::from("/tmp/memwatch.sock");

    let mut stream = UnixStream::connect(&socket_path)?;
    stream.write_all(b"toggle")?;

    Ok(())
}
