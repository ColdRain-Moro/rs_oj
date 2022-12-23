#![feature(let_chains)]
#![feature(option_result_contains)]

use std::{fs, sync::Mutex};
use std::cell::Cell;
use std::sync::Arc;

use actix_web::{middleware::Logger, App, HttpServer, web};
use clap::{command, arg};
use lazy_static::lazy_static;
use model::{config::Config, Job};

mod controller;
mod model;
mod err;

lazy_static! {
    static ref JOB_LIST: Arc<Mutex<Vec<Job>>> = Arc::new(Mutex::new(Vec::new()));
    static ref ATOMIC_ID: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
}

#[derive(Debug, Clone)]
pub struct AppState {
    config: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = command!()
      .arg(arg!(--config <VALUE> "the config.json path"))
      .get_matches();
    let path: &str = matches.get_one::<String>("config").expect("config.json path not found");
    let config: Config = serde_json::from_str(
      &fs::read_to_string(path).expect("read config.json failed")
    )?;
    let server = config.server.clone();
    // 上把互斥锁 允许并发访问，但要小心死锁
    let mutex = Mutex::new(AppState { config });
    let app_state = web::Data::new(mutex);
    HttpServer::new(move || {
        App::new()
            // 日志中间件
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(controller::jobs::post_job)
            .service(controller::jobs::get_jobs)
            .service(controller::jobs::get_job_by_id)

    })
    .bind((server.bind_address, server.bind_port))?
    .run()
    .await
}
