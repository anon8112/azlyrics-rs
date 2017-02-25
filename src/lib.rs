extern crate hyper;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Attr};

use hyper::client::Client;
use hyper::client::Response;
use hyper::Error;

use std::io::Read;

pub fn search(param: &str) -> Vec<String> {
    let url = format!("http://search.azlyrics.com/search.php?q={}", param);
    let res = request(&url).unwrap();
    let doc = Document::from(res.as_str());
    let mut links: Vec<String> = Vec::new();
    for node in doc.find(Class("visitedlyr")).iter() {
        let a = node.find(Name("a")).first().unwrap();
        let link = a.attr("href").unwrap();
        links.push(link.into());
    }
    links
}

pub fn get_lyrics(url: &str) -> Result<String, String> {
    if !url.contains("lyrics") { return Err("Url must be of a song not an artist".into()) };
    let res = request(url).unwrap();
    let doc = Document::from(res.as_str());
    let mut count = 0;
    for node in doc.find(Name("div")).iter() {
        if count == 21 {
            return Ok(node.text())
        }
        count += 1;
    }
    Err("no lyrics found".into())
}

pub fn request(url: &str) -> Result<String, Error> {
    let client = Client::new();
    let mut res = try!(client.get(url).send());
    let mut s = String::new();
    let _ = res.read_to_string(&mut s);
    Ok(s)
}
