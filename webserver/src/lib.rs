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

use std::error::Error;
use iron::prelude::*;
use iron::status;

/// Start the webserver. The method will not return normally while the server is running endlessly.
pub fn run() -> Result<(), Box<Error>> {
    env_logger::init().unwrap();

    // take all requests
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        debug!("GET hello-world");
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    debug!("Start the webserver...");
    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");

    Ok(())
}
