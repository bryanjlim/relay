use std::convert::Infallible;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;

use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

static SYSTEM_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
pub async fn run() {
    // Starts server to handle HTTP commands
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
// Handles HTTP request and relays body via TCP
async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let stream_result = TcpStream::connect(SYSTEM_ADDRESS);
    let mut response = Response::new(Body::empty());
    match stream_result {
        Ok(mut stream) => {
            match (req.method(), req.uri().path()) {
                (&Method::POST, "/") => {
                    let bytes = body::to_bytes(req.into_body()).await;
                    match bytes {
                        Ok(buf) => {
                            // println!("Body: \n{}", String::from_utf8_lossy(&buf));
                            stream
                                .write(&buf[..])
                                .expect("ERROR: Could not write buffer to TCP");
                        }
                        Err(e) => println!("Error reading body: {}", e),
                    }
                }
                _ => {
                    *response.status_mut() = StatusCode::NOT_FOUND;
                }
            };
        }
        Err(e) => println!("TCP Connection ERROR: {}", e),
    }
    Ok(response)
}
