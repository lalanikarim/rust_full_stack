use axum::body::{boxed, Body, BoxBody, HttpBody};
use axum::response::Response;
use axum::routing::{get, get_service, IntoMakeService};
use axum::{Router, Server};
use hyper::server::conn::AddrIncoming;

use hyper::{Request, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs;
use tower_http::services::ServeDir;

use super::AppState;

pub struct AxumApp;

impl AxumApp {
    pub fn create(
        routes: Router<AppState>,
        state: AppState,
        listener_addr: &str,
    ) -> Server<AddrIncoming, IntoMakeService<Router>> {
        let addr: SocketAddr = listener_addr.parse().expect("Unable to parse IP Address");
        let service_error_function = |error: Infallible| async move {
            println!("service error!");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal server error: {error}"),
            )
        };
        let spa_service = get(move |req: Request<Body>| async {
            let dist_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dist");
            let spa_index = "./dist/index.html";
            if let Ok(index_content) = fs::read_to_string(spa_index).await {
                match ServeDir::new(dist_dir).try_call(req).await {
                    Ok(resp) => match resp.status() {
                        StatusCode::NOT_FOUND => Response::builder()
                            .status(StatusCode::OK)
                            .body(boxed(Body::from(index_content)))
                            .unwrap(),
                        _ => resp.map(boxed),
                    },
                    Err(err) => Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(boxed(Body::from(format!(
                            "index not found: {}",
                            err.to_string()
                        ))))
                        .unwrap(),
                }
            } else {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(boxed(Body::from("index not found").boxed()))
                    .unwrap()
            }
        });
        let app: Router = Router::new()
            .nest("/api", routes)
            .with_state(state)
            .fallback({
                let dist_dir = "./dist".to_string();
                get_service(ServeDir::new(dist_dir).append_index_html_on_directories(true))
                    .handle_error(service_error_function)
            })
            .fallback(spa_service.handle_error(service_error_function));
        let server = axum::Server::bind(&addr).serve(app.into_make_service());
        server
    }
}
