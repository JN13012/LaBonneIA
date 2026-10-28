use std::{env, error::Error, net::Ipv4Addr, process::ExitCode};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("LaBonneIA API: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let port = match env::var("API_PORT") {
        Ok(value) => value.parse::<u16>()?,
        Err(env::VarError::NotPresent) => 3000,
        Err(error) => return Err(error.into()),
    };
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port)).await?;
    println!("LaBonneIA API: http://{}", listener.local_addr()?);
    axum::serve(listener, labonneia_api::app())
        .with_graceful_shutdown(shutdown())
        .await?;
    Ok(())
}

async fn shutdown() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        eprintln!("LaBonneIA API: impossible d’écouter Ctrl+C: {error}");
    }
}
