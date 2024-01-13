use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use env_logger::{self, Env};
use log::info;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "backend", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "dist")]
    static_dir: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::parse();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    info!("Server started at http://{}", sock_addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(|app| {
                app.service(hello)
                    .service(echo)
                    .route("/hey", web::get().to(manual_hello));
            })
    })
    .bind(sock_addr)?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
