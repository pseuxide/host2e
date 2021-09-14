use bytes::Bytes;
//use reqwest::header::CONTENT_TYPE;

pub async fn get_image_as_bytes(url: &String) -> Bytes {
    let response = reqwest::get(url).await.expect("The form of given url is invalid. Make sure it represent the form of https://foo.com/bar.jpg or png");
    //response.headers().get(CONTENT_TYPE)
    response
        .bytes()
        .await
        .expect("Failed to convert response to bytes")
}
