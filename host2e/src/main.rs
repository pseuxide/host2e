use std::fs::File;
use std::io::Write;

use host2bin;
mod arg_parser;
mod base64encoder;

#[tokio::main]
async fn main() {
    let args = arg_parser::parse();
    let imgbin = host2bin::get_image_as_bytes(&args.url).await;
    let result = match &args.encode_type[..] {
        // converts String into &str by slicing entire string
        "base64" => base64encoder::run(imgbin),
        _ => panic!("given type specifier is invalid."),
    };
    let mut file = File::create("host2e.txt").expect("Failed to create the output file.");
    file.write_all(result.as_bytes())
        .expect("Failed to write string to the file.");
}
