use axum::routing::{get_service, IntoMakeService};
use axum::{Router, Server};
use hyper::server::conn::AddrIncoming;

use hyper::StatusCode;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;

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
        let dist_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dist");
        let service_error_function = |error: Infallible| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal server error: {error}"),
            )
        };
        let app: Router = Router::new()
            .nest("/api", routes)
            .with_state(state)
            .fallback(
                get_service(ServeDir::new(dist_dir).append_index_html_on_directories(true))
                    .handle_error(service_error_function),
            );
        let server = axum::Server::bind(&addr).serve(app.into_make_service());
        server
    }
}
