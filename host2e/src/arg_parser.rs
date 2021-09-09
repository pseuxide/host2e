extern crate clap;
use clap::{App, Arg};

pub struct Arguments {
    pub url: String,
    pub encode_type: String,
}

pub fn parse() -> Arguments {
    let matches = App::new("host2e")
        .version("0.1.0")
        .author("s3pt3mb3r")
        .about("get binary on the internet and encode it to whatever way you prefer.")
        .arg(Arg::with_name("url").required(true).index(1))
        .arg(
            Arg::with_name("encode_type")
                .short("t")
                .long("encode_type")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap().into();
    let encode_type = matches.value_of("encode_type").unwrap().into();

    Arguments { url, encode_type }
}
