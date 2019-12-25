use std::thread;

mod udp_server;
mod tcp_server;

fn main() {
    thread::spawn(|| {udp_server::run()});
    tcp_server::run();
}