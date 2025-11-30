use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::{TokioIo, TokioTimer};
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[allow(unused_variables)]
async fn service(
    req: Request<impl hyper::body::Body>,
    remote_addr: SocketAddr,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let (parts, body) = req.into_parts();
    let path = parts.uri.path().trim_matches('/');

    println!("{}", path);

    if let Some(("static", path)) = path.split_once("/") {
        Ok(Response::builder()
            .body(Full::new(Bytes::from({
                let mut buf = vec![];
                let a = File::open(format!("static/{path}"))
                    .await
                    .unwrap()
                    .read_to_end(&mut buf).await;
                // println!("{:?}", buf);
                buf
            })))
            .unwrap())
    } else if path.is_empty() {
        todo!()
    } else {
        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "text/plain")
            .body(Full::new(Bytes::from("404 Not Found")))
            .unwrap())
    }
}

pub async fn start_server(addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (tcp, remote_addr) = listener.accept().await?;

        let io = TokioIo::new(tcp);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(
                    io,
                    service_fn(|req| {
                        return service(req, remote_addr);
                    }),
                )
                .await
            {
                eprintln!("Error serving connection: {err}");
            }
        });
    }
}
