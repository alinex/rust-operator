#[macro_use]
extern crate log;
extern crate env_logger;

use std::error::Error;

pub fn run() -> Result<(), Box<Error>> {
    env_logger::init().unwrap();

    debug!("Start the webserver...");


    Ok(())
}
