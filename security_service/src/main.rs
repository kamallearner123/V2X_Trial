use std::io::Read;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").expect("Could not bind to port 8082");
    println!("[Security Service] Listening on 127.0.0.1:8082 for incoming data to secure...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("[Security Service] V2X Stack connected.");
                thread::spawn(move || {
                    let mut buffer = [0; 1024];
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(0) => {
                                println!("[Security Service] V2X Stack disconnected.");
                                break;
                            }
                            Ok(n) => {
                                println!("[Security Service] Received {} bytes to sign/encrypt.", n);
                                // Dummy: We don't actually encrypt here, just pretend
                            }
                            Err(e) => {
                                println!("[Security Service] Error reading from stream: {}", e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                println!("[Security Service] Connection failed: {}", e);
            }
        }
    }
}
