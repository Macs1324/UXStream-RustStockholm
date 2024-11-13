use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use askama::Template;
use axum::{
    body::Body,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use thiserror::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let router = Router::new()
        .route("/", get(root))
        .route("/servers", get(server_list));

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");
}

#[derive(Template, Clone, Debug)]
#[template(path = "index.html")]
struct RootPage;

async fn root() -> Result<impl IntoResponse, WebuiError> {
    Ok(Html(
        RootPage.render().map_err(|e| WebuiError::RenderError(e))?,
    ))
}

#[derive(Error, Debug)]
enum WebuiError {
    #[error("Page not found")]
    NotFound,
    #[error("Failed to render template: {0}")]
    RenderError(askama::Error),
}

impl IntoResponse for WebuiError {
    fn into_response(self) -> axum::http::Response<Body> {
        let status = match self {
            WebuiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        Response::builder()
            .status(status)
            .body(Body::from(self.to_string()))
            .unwrap()
    }
}

async fn server_list() -> Result<impl IntoResponse, WebuiError> {
    let servers = vec![
        Server {
            name: Some("Server 1".to_string()),
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            status: ServerStatus::Available,
        },
        Server {
            name: Some("Server 2".to_string()),
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)),
            status: ServerStatus::Busy,
        },
        Server {
            name: None,
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 3)),
            status: ServerStatus::Offline,
        },
    ];

    Ok(Html(
        ServerListPage { servers }
            .render()
            .map_err(|e| WebuiError::RenderError(e))?,
    ))
}

#[derive(Template, Clone, Debug)]
#[template(path = "server_list.html")]
struct ServerListPage {
    pub servers: Vec<Server>,
}

#[derive(Clone, Debug)]
struct Server {
    pub name: Option<String>,
    pub ip: IpAddr,
    pub status: ServerStatus,
}

#[derive(Clone, Debug)]
enum ServerStatus {
    Available,
    Busy,
    Offline,
}
