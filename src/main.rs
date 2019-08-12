use actix_web::{HttpServer, App, web, Result,middleware::Logger};
use actix_files as fs;

fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("index.html")?)
}

fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix_rt::System::new("Serv");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(fs::Files::new("/", "/static").show_files_listing())
    })
    .bind("0.0.0.0:8087")
    .unwrap()
    .start();
    println!("Starting HttpServer: 0.0.0.0:8087");

    sys.run()
}
