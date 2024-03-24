use axum::routing::get;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use clap::Parser;
use crate::prelude::*;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = "localhost")]
    hostname: String,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Params {
    Smiles(String),
}

#[derive(Debug, Clone, Copy)]
struct Mimed<B> {
    mime: &'static str,
    body: B,
}
impl<B: IntoResponse> IntoResponse for Mimed<B> {
    fn into_response(self) -> Response {
        let (mut parts, body) = self.body.into_response();
        parts.headers.append("Content-Type", self.mime);
        Response::from_parts(parts, body)
    }
}

#[cfg(feature = "mol-bmp")]
async fn serve_png() {}
#[cfg(feature = "mol-svg")]
async fn serve_svg(params: Query<Params>) -> Result<Mimed, StatusCode> {
    match params {
        Params::Smiles(s) => Mimed {
            mime: "",
            body: fmtSmilesParser::new(&s).parse().map_err(|_| StatusCode::BAD_REQUEST)?
    }
}

#[tokio::main]
async fn main() -> Box<dyn std::error::Error> {
    let cli = Cli::parse();
    let app = axum::Router::new();

    #[cfg(feature = "mol-bmp")]
    let app = app.route("/mol.png", get(serve_png));

    #[cfg(feature = "mol-svg")]
    let app = app.route("/mol.svg", get(serve_svg));

    let listener = tokio::net::TcpListener::bind((cli.hostname, cli.port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
