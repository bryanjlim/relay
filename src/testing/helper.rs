use std::io::prelude::*;
use std::net::TcpListener;
use std::net::UdpSocket;
use std::thread;

fn main() {
    // thread::spawn(|| {send_packets()});
    // listen_udp();

    // listen_tcp();
}

fn listen_tcp() {
    // Listens for TCP Packets
    let res = TcpListener::bind("127.0.0.1:8000");
    match res {
        Ok(listener) => {
            // accept connections and process them serially
            for connection in listener.incoming() {
                match connection {
                    Ok(mut stream) => {
                        let mut buffer = [0; 10000];
                        stream.read(&mut buffer).expect("Error reading buffer");
                        println!("Body: {}", String::from_utf8_lossy(&buffer));
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}

fn listen_udp() {
    // Listens for UDP Packets
    let in_socket = UdpSocket::bind("127.0.0.1:3001").expect("couldn't bind to in address");
    in_socket
        .connect("127.0.0.1:2")
        .expect("connect function failed");
    let mut buf = [0; 10];
    match in_socket.recv(&mut buf) {
        Ok(received) => println!("UDP packet received: {}", String::from_utf8_lossy(&buf[..received])),
        Err(e) => println!("recv function failed: {:?}", e),
    }
}

fn send_packets() {
    // Sends UDP Packet
    let out_socket = UdpSocket::bind("127.0.0.1:8000").expect("couldn't bind to out address");
    out_socket
            .connect("127.0.0.1:1")
            .expect("connect function failed");
    out_socket
        .send("Test".as_bytes())
        .expect("Error sending UDP packet");
}
