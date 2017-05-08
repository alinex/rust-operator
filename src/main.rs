//!
//! # Operator
//!
//! An application to manage data structures through a web interface.
//! It is my first bigger application in rust and as this is used to learn more about ruat and
//! it's eco system.
//!
//! Find the source code at [`GutHub`](https://github.com/alinex/rust-operator).
//!
//! **Currently under heavy development!**
//!
//! ## Main Packages
//!
//! Mainly the following crates are used:
//!
//! - [clap](../clap/index.html) - CLI argument parsing
//! - [webserver](../webserver/index.html) - webserver based on iron

#[macro_use]
extern crate clap;
extern crate webserver;
extern crate ansi_term;

#[cfg(test)]
extern crate gag;

use clap::{App, Arg};

use std::process;
use std::io::prelude::*;

// Startup of the binary call.
fn main() {
    logo();

    // initialization
    let mut stderr = std::io::stderr();
    // method to check for unsigned integer for arguments
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
    fn is_u32(v: String) -> Result<(), String> {
        if v.parse::<u32>().is_ok() {
            return Ok(());
        }
        Err(format!("{} isn't a positive integer number", &*v))
    }

    // CLI setup
    let arg = App::new("IT Operator")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file")
                 .takes_value(true))
        .arg(Arg::with_name("port")
                 .short("p")
                 .long("port")
                 .value_name("NUM")
                 .help("Sets the port to start the server")
                 .validator(is_u32)
                 .takes_value(true))
        .get_matches();

    //    println!("Matches: {}", &matches);

    // Gets a value for config if supplied by user, or defaults to "operator.yaml"
    let file = arg.value_of("config").unwrap_or("operator.yaml");
    println!("Value for config: {}", file);

    let mut config = webserver::Config::new();
    match arg.value_of("port") {
        Some(i) => {
            config.port = i.parse::<u32>().unwrap();
            ()
        }
        _ => (),
    }

    // start webserver
    if let Err(e) = webserver::run(config) {
        writeln!(&mut stderr, "Application error: {}", e).expect("Could not write to stderr");
        process::exit(1);
    }

    // add applications to webserver

}

// output logo
fn logo() {
    use ansi_term::Colour::{Cyan, Yellow};

    println!("                            {}",
             Yellow.bold().paint(" __   ____     __"));
    println!("             {}  {}   {}",
             Cyan.paint("######  #####"),
             Yellow.bold().paint("|  | |    \\   |  |"),
             Cyan.paint("########### #####       #####"));
    println!("            {}  {}  {}",
             Cyan.paint("######## #####"),
             Yellow.bold().paint("|  | |     \\  |  |"),
             Cyan.paint("############  #####     #####"));
    println!("           {}  {}  {}",
             Cyan.paint("######### #####"),
             Yellow.bold().paint("|  | |  |\\  \\ |  |"),
             Cyan.paint("#####          #####   #####"));
    println!("          {}  {}  {}",
             Cyan.paint("########## #####"),
             Yellow.bold().paint("|  | |  | \\  \\|  |"),
             Cyan.paint("#####           ##### #####"));
    println!("         {}  {}  {}",
             Cyan.paint("##### ##### #####"),
             Yellow.bold().paint("|  | |  |_ \\     |"),
             Cyan.paint("############     #########"));

    println!("        {}  {}  {}",
             Cyan.paint("#####  ##### #####"),
             Yellow.bold().paint("|  | |    \\ \\    |"),
             Cyan.paint("############     #########"));
    println!("       {}  {}  {}",
             Cyan.paint("#####   ##### #####"),
             Yellow.bold().paint("|__| |_____\\ \\___|"),
             Cyan.paint("#####           ##### #####"));
    println!("{}",
             Cyan.paint("      #####    ##### #####                      #####          #####   \
                         #####"));
    println!("{}",
             Cyan.paint("     ##### ######### ########################## ############  #####     \
                         #####"));
    println!("{}",
             Cyan.paint("    ##### ##########  ########################   ########### #####       \
                         #####"));
    let line = Cyan.paint("    ___________________________________________________________________________");
    println!("{}", line);
    println!();
    println!("{}",
             Yellow
                 .bold()
                 .paint("                              O P E R A T O R"));
    println!("{}", line);
    println!();
}



#[cfg(test)]
mod tests {
    use super::logo;

    #[test]
    fn output_logo() {
        use std::io::Read;
        use gag::BufferRedirect;

        let mut buf = BufferRedirect::stdout().unwrap();

        logo();

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();

        assert!(output.contains("O P E R A T O R"));
    }
}
