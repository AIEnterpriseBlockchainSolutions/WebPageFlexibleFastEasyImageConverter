pub mod ai_option;
pub mod factory_pool;
pub mod hashmap;
pub mod hashsets;
pub mod idintifier;

use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::time::Instant;

#[tokio::main]
async fn main() {
    // initlizes the logger
    let stdout = ConsoleAppender::builder().build();
    // builds the Config fro the logger
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    // start the logger
    let _handle = log4rs::init_config(config).unwrap();
    // start inilizing
    info!("Start initilizing");
    let start = Instant::now();
    factory_pool::init::init().await;
    let duration = start.elapsed();
    info!("Initilized in {:?} seconds", duration);
    // finished initlizing
}
