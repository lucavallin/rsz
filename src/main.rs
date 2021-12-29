extern crate clap;
extern crate image;

use clap::{Arg, App};
use image::GenericImageView;
use image::imageops::FilterType;

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

    let ratio: f32 = matches.value_of("ratio").unwrap().parse().unwrap();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let image_path = matches.value_of("INPUT").unwrap();

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(image_path).unwrap();
    let (width, height) = img.dimensions();

    let resized = img.resize(width * ratio as u32, height * ratio as u32, FilterType::Nearest);

    // Write the contents of this image to the Writer in PNG format.
    resized.save("test.png").unwrap();
}
