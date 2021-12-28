extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("rsz")
        .version("1.0")
        .author("Luca Cavallin <nillavac@gmail.com>")
        .about("Resize an image by the specified ratio")
        .arg(Arg::with_name("ratio")
            .short("-r")
            .long("ratio")
            .value_name("RATIO")
            .help("Sets the resize ratio for the image")
            .default_value("0.5")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let ratio = matches.value_of("ratio").unwrap_or_default();
    println!("Value for ratio: {}", ratio);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // more program logic goes here...
}
