use std::thread;
use std::time::Duration;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger, TrailingSlash::Trim};
use env_logger::{Env, Builder};

use logger_rust::*;
use crate::render_index::index;
use crate::form_process::process_form;
use crate::error_handler::{not_found, handle_error};

pub async fn create_server(server_ip: &str) -> std::io::Result<()> {
    log_info!("🚀 Trying to run on: \x1b[31m{}\x1b[0m", server_ip); // output server ip
    let server = match HttpServer::new(|| { // start the HTTP server
        App::new()  
            .wrap(Logger::default()) // logging
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::NormalizePath::new(Trim))
            .route("/", web::get().to(index)) // register routes and their handlers
            .route("/form", web::post().to(process_form))
//.service(web::resource("/upload").route(web::post().to(save_file))) <-- i'll add this bullshit somehow later maybe
//.service(web::resource("/data").route(web::get().to(get_data))) <-- i'll add this bullshit somehow later maybe
// i'll add a db connection here later too but idk why then this server called "basic" lol
            .app_data(web::Data::new(handle_error)) // register the error handler
            .default_service(
                actix_web::web::route().to(not_found)
         ) // default gateway for bad request -> like 404
    })
    
    .bind(server_ip) {
        // for ok
        Ok(server) => { // if ok
            log_warn!("📢 \x1B[1m\x1b[32mListening on: \x1b[31mhttp://{}\x1b[0m", server_ip); // print the server IP address after the server starts
            log_info!("✅ \x1B[1m\x1B[4mOk bro now i'm gonna run ur site\x1b[0m");
            server
        }
        // for errors
        Err(e) => { // if NOT ok
            log_error!("!!! FAILED TO BIND A SERVER !!!\n\x1b[33mIP: \x1b[31m'{}'\n\x1b[33m  |\n\x1b[33m  v\n\x1b[33mERROR_CODE: \x1b[31m{}\x1b[0m", server_ip, e);
            thread::sleep(Duration::from_secs(10)); 
            return Err(e);
        }
    };

    server.run().await?; // run the server

    Ok(())
}


pub fn init_logger() { // initialize the logger
    Builder::from_env(Env::default().default_filter_or("info")).init();
}

// colors: 
// \x1b[32m - green
// \x1b[31m - red
// \x1B[4m - underline 
// \x1B[1m - bold
// \x1b[0m - reset