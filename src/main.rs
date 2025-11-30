mod web;
mod encode;
mod decode;
use std::net::SocketAddr;
use web::start_server;
use decode::decode_img_to_string;
use encode::write_mes_to_file;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    write_mes_to_file("mesaj.emre".to_string(), "static/miyabiicon.png".to_string());
    decode_img_to_string("static/saklanmis_mesaj.png".to_string());

    let _ = start_server(SocketAddr::from(([127, 0, 0, 1], 1234))).await;
}
