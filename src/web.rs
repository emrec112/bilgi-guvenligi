use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::{TokioIo, TokioTimer};
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

async fn serve_file(path: impl AsRef<Path>) -> Response<Full<Bytes>> {
    match File::open(path).await {
        Ok(mut file) => {
            let mut buf = vec![];
            if let Err(e) = file.read_to_end(&mut buf).await {
                Response::new(Full::new(Bytes::from(format!("Error reading file: {}", e))))
            } else {
                Response::builder()
                    .header(
                        "Content-Type",
                        infer::get(&buf)
                            .and_then(|t| Some(infer::Type::mime_type(&t)))
                            .unwrap_or_default(),
                    )
                    .body(Full::new(Bytes::from(buf)))
                    .unwrap()
            }
        }
        _ => Response::builder()
            .status(404)
            .body(Full::new(Bytes::from(format!("404 Not Found"))))
            .unwrap(),
    }
}

#[allow(unused_variables)]
async fn service(
    req: Request<impl hyper::body::Body>,
    remote_addr: SocketAddr,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let (parts, body) = req.into_parts();
    let path = parts.uri.path().trim_matches('/');

    println!("{}", path);
    match path.split_once('/') {
        Some(("static", sub_path)) => Ok(serve_file(format!("static/{}", sub_path)).await),

        None if path.is_empty() => Ok(serve_file("static/index.html".to_string()).await),

        Some(("api", sub_path)) => {
            todo!()
        }

        _ => Ok(Response::builder()
            .status(404)
            .header("Content-Type", "text/plain")
            .body(Full::new(Bytes::from("404 Not Found")))
            .unwrap()),
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
