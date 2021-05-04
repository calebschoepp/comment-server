use serde_json::Value;
use std::collections::BTreeMap;
use suborbital::http;
use suborbital::log;
use suborbital::req;
use suborbital::runnable::*;

struct CountComments {}

impl Runnable for CountComments {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let url = match req::state("url") {
            Some(url) => url,
            None => return Err(RunErr::new(500, "No url found in state")),
        };
        let platform = match req::state("platform") {
            Some(platform) => platform,
            None => return Err(RunErr::new(500, "No platform found in state")),
        };

        log::info(&format!("Counting comments at url: {}", url));
        let count = match platform.as_str() {
            "reddit" => count_reddit_comments(url),
            "hackernews" => count_hackernews_comments(url),
            _ => Err(RunErr::new(400, "Invalid platform")),
        }?;

        Ok(String::from(format!("{}", count)).as_bytes().to_vec())
    }
}

fn count_reddit_comments(url: String) -> Result<u64, RunErr> {
    // Build url to hit JSON endpoint
    let url = url + ".json";

    // Parse comment count from response
    let mut headers = BTreeMap::new();
    headers.insert("User-Agent", "comment-server");
    let bytes = match http::get(&url, Some(headers)) {
        Ok(bytes) => bytes,
        Err(err) => return Err(err),
    };
    let post: Value = match serde_json::from_slice(&bytes) {
        Ok(value) => value,
        Err(_) => return Err(RunErr::new(500, "Failed to serialize json")),
    };
    let comment_count = &post[0]["data"]["children"][0]["data"]["num_comments"];
    match comment_count {
        Value::Number(num) => Ok(num.as_u64().unwrap()),
        _ => Err(RunErr::new(500, "Field \"comment_count\" is not a number")),
    }
}

fn count_hackernews_comments(url: String) -> Result<u64, RunErr> {
    // Build url for Hacker News API
    let id = url.split("?id=").last();
    if let None = id {
        return Err(RunErr::new(400, "Invalid Hacker News URL"));
    }
    let url = format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json",
        id.unwrap()
    );

    // Parse comment count from response
    let bytes = match http::get(&url, None) {
        Ok(bytes) => bytes,
        Err(err) => return Err(err),
    };
    let post: Value = match serde_json::from_slice(&bytes) {
        Ok(value) => value,
        Err(_) => return Err(RunErr::new(500, "Failed to serialize json")),
    };
    let descendants = &post["descendants"];
    match descendants {
        Value::Number(num) => Ok(num.as_u64().unwrap()),
        _ => Err(RunErr::new(500, "Field \"descendants\" is not a number")),
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &CountComments = &CountComments {};

#[no_mangle]
pub extern "C" fn init() {
    use_runnable(RUNNABLE);
}
