use clap::Parser;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use std::path::PathBuf as FsPathBuf;
use server::SSTServer;

pub mod server;
pub mod database;
pub mod data;

pub mod proto_service {
    tonic::include_proto!("service");
}


#[static_init::dynamic]
pub(crate) static OPTS: Opts = {
    dotenv::dotenv().ok();
    Opts::parse()
};

#[derive(Debug, Parser)]
struct Opts {

    #[clap(long, default_value = "9090", env = "SST_SERVER_PORT")]
    port: u16,

    #[clap(long, default_value = "test_resources/key.pem", env = "SST_SERVER_PRIVATE_KEY_FILE")]
    private_key_file: FsPathBuf,

    #[clap(long, default_value = "test_resources/cert.pem", env = "SST_SERVER_CERTIFICATE_FILE")]
    certificate_file: FsPathBuf,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting SST server, with options:\n{:#?}", *OPTS);

    let cert = tokio::fs::read(&OPTS.certificate_file).await?;
    let key = tokio::fs::read(&OPTS.private_key_file).await?;
    
    let identity = Identity::from_pem(cert, key);

    let server = SSTServer::default();

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(proto_service::todo_list_server::TodoListServer::new(server))
        .serve(std::net::SocketAddr::new("0.0.0.0".parse()?, OPTS.port))
        .await?;

    Ok(())
}
