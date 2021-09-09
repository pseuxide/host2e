use host2bin;
mod arg_parser;

fn main() {
    let args = arg_parser::parse();
    host2bin::scrape();
}
