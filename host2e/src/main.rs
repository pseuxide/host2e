use host2bin;
mod arg_parser;
use base64::encode;

#[tokio::main]
async fn main() {
    let args = arg_parser::parse();
   host2bin::get_image_as_bytes(&args.url).await;
}
