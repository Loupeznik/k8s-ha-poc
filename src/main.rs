use std::net::SocketAddr;

use http_body_util::{Empty, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use hyper::{header, Method, Request, Response, StatusCode};
use http_body_util::{combinators::BoxBody, BodyExt};
use sys_info;
use local_ip_address::local_ip;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

#[derive(Debug, serde::Serialize)]
struct ResponseBody {
    hostname: String,
    datetime: String,
    internal_ip_addr: String,
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}



async fn get_response() -> Result<Response<BoxBody<Bytes, hyper::Error>>> {
    let hostname = sys_info::hostname().unwrap_or_else(|_| "Unknown".to_string());
    let datetime = chrono::Local::now().to_string();
    let internal_ip_addr = local_ip().map_or_else(|_| "Unknown".to_string(), |ip| ip.to_string());

    let response = ResponseBody {
        hostname,
        datetime,
        internal_ip_addr,
    };
    let res = match serde_json::to_string(&response) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(full(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(full(StatusCode::INTERNAL_SERVER_ERROR.as_str()))
            .unwrap(),
    };
    Ok(res)
}

async fn routes(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => get_response().await,


        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}



#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([0,0,0,0], 3000));

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(routes))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}