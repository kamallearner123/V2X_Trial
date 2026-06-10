mod asn1_ffi;

use asn1_ffi::{asn1_decode, asn1_encode, BSMData};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_network_layer(mut stream: TcpStream) {
    println!("[V2X Stack] Connected to Network Layer.");
    let mut buffer = [0u8; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("[V2X Stack] Network Layer disconnected.");
                break;
            }
            Ok(n) => {
                // Decode the received ASN.1 byte stream (dummy) into BSMData
                let mut bsm_data = BSMData {
                    vehicle_id: 0,
                    latitude: 0,
                    longitude: 0,
                    speed: 0,
                    heading: 0,
                };

                let res = unsafe {
                    asn1_decode(
                        buffer.as_ptr(),
                        n as libc::size_t,
                        &mut bsm_data as *mut BSMData,
                    )
                };

                if res == 0 {
                    println!(
                        "[V2X Stack] Decoded Incoming Network BSM: Vehicle ID: {}, Speed: {}",
                        bsm_data.vehicle_id, bsm_data.speed
                    );
                } else {
                    println!("[V2X Stack] Failed to decode incoming network ASN.1 frame.");
                }
            }
            Err(e) => {
                println!("[V2X Stack] Network read error: {}", e);
                break;
            }
        }
    }
}

fn handle_can_interface(mut can_stream: TcpStream) {
    println!("[V2X Stack] Connected to CAN Interface.");
    let mut buffer = [0u8; 1024];

    // Try to connect to Security Service to forward encoded packets
    let mut sec_stream = loop {
        match TcpStream::connect("127.0.0.1:8082") {
            Ok(s) => break s,
            Err(_) => {
                println!("[V2X Stack] Waiting for Security Service...");
                thread::sleep(Duration::from_secs(2));
            }
        }
    };

    println!("[V2X Stack] Connected to Security Service.");

    loop {
        match can_stream.read(&mut buffer) {
            Ok(0) => {
                println!("[V2X Stack] CAN Interface disconnected.");
                break;
            }
            Ok(n) => {
                // Here the CAN interface sends raw BSMData struct for simplicity
                if n == std::mem::size_of::<BSMData>() {
                    let can_data: BSMData = unsafe { std::ptr::read(buffer.as_ptr() as *const _) };
                    println!(
                        "[V2X Stack] Read local CAN data: Speed {}, Encoding...",
                        can_data.speed
                    );

                    // Encode into ASN.1 using C library
                    let mut encoded_buffer = [0u8; 1024];
                    let mut encoded_len: libc::size_t = 0;

                    let res = unsafe {
                        asn1_encode(
                            &can_data as *const BSMData,
                            encoded_buffer.as_mut_ptr(),
                            &mut encoded_len as *mut libc::size_t,
                        )
                    };

                    if res == 0 {
                        println!("[V2X Stack] Successfully encoded to ASN.1 ({} bytes). Sending to Security Service.", encoded_len);
                        if let Err(e) = sec_stream.write_all(&encoded_buffer[0..encoded_len as usize]) {
                            println!("[V2X Stack] Failed to send to Security Service: {}", e);
                        }
                    } else {
                        println!("[V2X Stack] Failed to ASN.1 encode CAN data.");
                    }
                }
            }
            Err(e) => {
                println!("[V2X Stack] CAN read error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    println!("[V2X Stack] Starting...");

    // Spawn a thread to listen for Network Layer (TCP 8080)
    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind 8080");
        println!("[V2X Stack] Listening on 127.0.0.1:8080 for Network Layer");
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                thread::spawn(move || handle_network_layer(stream));
            }
        }
    });

    // Spawn a thread to listen for CAN Interface (TCP 8081)
    let can_listener = TcpListener::bind("127.0.0.1:8081").expect("Could not bind 8081");
    println!("[V2X Stack] Listening on 127.0.0.1:8081 for CAN Interface");
    for stream in can_listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || handle_can_interface(stream));
        }
    }
}
