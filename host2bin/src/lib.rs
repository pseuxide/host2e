use std::any::Any;

use bytes::Bytes;

pub async fn get_image_as_bytes(url: &String) {
    let response = reqwest::get(url).await.expect("The form of given url is invalid. Make sure it represent the form of https://foo.com/bar.jpg or png");
    println!("{:?}", response.type_id());
}
