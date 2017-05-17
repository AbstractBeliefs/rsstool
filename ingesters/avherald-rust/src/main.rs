extern crate hyper;
extern crate hyper_native_tls;
extern crate hyper_socks;
extern crate select;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper::header::{UserAgent, AcceptEncoding, Encoding, qitem};
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

use select::document::Document;
use select::predicate::{Attr, Name, And};

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);

    let client = Client::with_connector(connector);

    let mut resp = client
        .get("https://avherald.com/")
        .header(UserAgent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/57.0.2987.98 Safari/537.36".to_string()))
        .header(AcceptEncoding(vec![qitem(Encoding::Gzip),qitem(Encoding::Deflate),]))
        .send()
        .unwrap();
    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();

    let dom = Document::from(body.as_str());
    let article_root = dom.find(Attr("id", "ad1cell")).nth(0).expect("Couldn't find the article list");

    for node in article_root.find(And(Name("td"), Attr("align", "center"))) {
        // Find the severity
        let severity = node.find(Name("img")).nth(0)
            .and_then(|img| img.attr("alt"))
            .unwrap_or("Unknown");

        // Find the title
        let title = node.next()
            .and_then(|contentcell| Some(contentcell.text()))
            .unwrap_or("Unknown".to_string());

        // Find the link
        let link = node.next()
            .and_then(|contentcell| contentcell.find(Name("a")).nth(0)
                .and_then(|link| link.attr("href"))
                .and_then(|addr| Some("https://avherald.com".to_string() + addr)))
            .unwrap_or("Unknown".to_string());

        println!("[{}] {}: {}", severity, title, link);
    }
}
