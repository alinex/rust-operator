//!
//! # Webserver
//!
//! This library wraps the iron webserver to be used in my project.
//!
//! ## Usage
//!
//! ```
//! // start webserver
//! if let Err(e) = webserver::run() {
//!     writeln!(&mut stderr, "Application error: {}", e).expect("Could not write to stderr");
//!     process::exit(1);
//! }
//! ```

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate iron;
extern crate router;

use std::error::Error;
use std::fmt;
use iron::prelude::*;
use iron::status;
use router::Router;

/// The configuration structure is used to setup the server tu run.
pub struct Config {
    /// Server port to bind to.
    ///
    /// The default value is `3000` but keep in mind that you only be able to bind to ports lower
    /// than 1024 as `root`.
    pub port: u32,
}
impl Config {
    /// Get the default configuration.
    pub fn new() -> Config {
        Config { port: 3000 }
    }
}
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server Setup: localhost:{}", self.port)
    }
}

/// Start the webserver. The method will not return normally while the server is running endlessly.
pub fn run(config: Config) -> Result<(), Box<Error>> {
    env_logger::init().unwrap();

    // setup routing
    let mut router = Router::new();
    router.get("/", hello_world, "index");
    // take all requests
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        debug!("GET hello-world");
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    info!("Start the webserver...");
    debug!("{}", &config);
    let server = "localhost:".to_string() + &config.port.to_string();
    Iron::new(router).http(server).unwrap();

    Ok(())
}
