//! Structure for getting metadata from crossref.org and decoding it.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CrossRefMessage<'a, Msg> {
    #[serde(borrow)]
    status: &'a str,
    #[serde(rename = "message-type", borrow)]
    messageType: &'a str,
    #[serde(rename = "message-version", borrow)]
    messageVersion: &'a str,
    message: Msg,
}

pub fn find_by_doi(doi: &str, client: reqwest::Client) -> () {
    ()
}
