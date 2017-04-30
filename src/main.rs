//!
//! # IT Systems Operator
//!
//! An application to manage complex IT structures easily.
//!
//! **Currently under heavy development!**

#[macro_use(crate_version, crate_authors, crate_description)]
extern crate clap;
extern crate webserver;
extern crate ansi_term;

use clap::{Arg, App};

use std::process;
use std::io::prelude::*;

fn main() {
    logo();

    // initialization
    let mut stderr = std::io::stderr();
    fn is_u32(v: String) -> Result<(), String> {
        if v.parse::<u32>().is_ok() {
            return Ok(());
        }
        Err(format!("{} isn't a positive integer number", &*v))
    }

    // CLI setup
    let matches = App::new("IT Operator")
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
    let config = matches.value_of("config").unwrap_or("operator.yaml");
    println!("Value for config: {}", config);

    // start webserver
    if let Err(e) = webserver::run() {
        writeln!(&mut stderr, "Application error: {}", e).expect("Could not write to stderr");
        process::exit(1);
    }

}

// output logo
fn logo() {
    use ansi_term::Style;
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
             Cyan.paint("      #####    ##### #####                      #####          #####   #####"));
    println!("{}",
             Cyan.paint("     ##### ######### ########################## ############  #####     #####"));
    println!("{}",
             Cyan.paint("    ##### ##########  ########################   ########### #####       #####"));
    let line = Cyan.paint("    ___________________________________________________________________________");
    println!("{}", line);
    println!();
    println!("{}",
             Yellow
                 .bold()
                 .paint("                           I T   O P E R A T O R"));
    println!("{}", line);
    println!();
}


pub struct Config {
    pub port: u32,
}
