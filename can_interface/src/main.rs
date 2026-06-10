use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct BSMData {
    vehicle_id: u32,
    latitude: i32,
    longitude: i32,
    speed: u16,
    heading: i16,
}

fn main() {
    println!("[CAN Interface] Starting dummy CAN reader...");

    let mut stream = loop {
        match TcpStream::connect("127.0.0.1:8081") {
            Ok(s) => break s,
            Err(_) => {
                println!("[CAN Interface] Connection Failed. Retrying in 2 seconds...");
                thread::sleep(Duration::from_secs(2));
            }
        }
    };

    println!("[CAN Interface] Connected to V2X Stack at 127.0.0.1:8081");

    let mut can_data = BSMData {
        vehicle_id: 2002, // Different vehicle ID to represent ego vehicle
        latitude: 340522000,
        longitude: -1182437000,
        speed: 65,
        heading: 180,
    };

    loop {
        can_data.speed = (can_data.speed + 2) % 120;
        can_data.latitude += 200;

        // Serialize to bytes (unsafe but works for a demo between same arch)
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                (&can_data as *const BSMData) as *const u8,
                std::mem::size_of::<BSMData>(),
            )
        };

        if let Err(e) = stream.write_all(bytes) {
            println!("[CAN Interface] Failed to send CAN data: {}", e);
            break;
        }

        println!("[CAN Interface] Read CAN bus -> Speed: {}, Heading: {}", can_data.speed, can_data.heading);

        thread::sleep(Duration::from_secs(1));
    }
}
