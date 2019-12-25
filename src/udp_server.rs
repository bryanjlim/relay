use std::net::UdpSocket;

static SOCKET_1: &str = "127.0.0.1:1";
static SOCKET_2: &str = "127.0.0.1:2";

static SYSTEM_ADDRESS: &str = "127.0.0.1:8000";
static FRONT_END_ADDRESS: &str = "127.0.0.1:3001";

pub fn run() {
    // Starts loop to handle UDP packets
    let system_udp = UdpSocket::bind(SOCKET_1).expect("couldn't bind to system");
    let front_end_udp = UdpSocket::bind(SOCKET_2).expect("couldn't bind to front end");
    system_udp
        .connect(SYSTEM_ADDRESS)
        .expect("system connect function failed");
    front_end_udp
        .connect(FRONT_END_ADDRESS)
        .expect("front end connect function failed");
    // Buffer for data received from system
    let mut buf = [0; 10000];
    loop {
        // Relay data from system
        match system_udp.recv(&mut buf) {
            Ok(received) => {
                // Relay data to front end after receiving it
                // println!("Got {}", String::from_utf8_lossy(&buf[..received]));
                front_end_udp
                    .send(&buf[..received])
                    .expect("couldn't send data to front end");
            }
            Err(e) => println!("recv function failed: {:?}", e),
        }
    }
}
