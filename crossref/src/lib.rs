mod message;

pub use message::Work;
use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

pub fn work_with_client(doi: impl AsRef<[u8]>, client: reqwest::Client) -> Work {
    let doi = percent_encode(doi.as_ref(), DEFAULT_ENCODE_SET);
    client.get(format!("https://api.crossref.org/works/{}", doi)).unwrap().json().unwrap()
}