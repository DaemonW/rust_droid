use reqwest::blocking::Client;
use reqwest::blocking::Response;
// use reqw;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn say_hello() -> String {
    let client = Client::new();
    let resp: Response = match client.get("http://httpbin.org/").send() {
        Ok(r) => r,
        Err(e) => panic!("request error: {}", e),
    };
    let status = resp.status();
    if !matches!(reqwest::StatusCode::OK, status) {
        return String::from("request error!");
    }
    let content = match resp.text() {
        Ok(v) => String::from("request success"),
        Err(e) => String::from("request error"),
    };
    content
}
