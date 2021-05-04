// use std::collections::BTreeMap;
use serde_json::Value;
use suborbital::http;
use suborbital::req;
use suborbital::runnable::*;

struct CountComments {}

impl Runnable for CountComments {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let url = match req::state("url") {
            Some(url) => url,
            None => return Err(RunErr::new(1, "No url found in state")), // TODO return more meaningful error code
        };
        let platform = match req::state("platform") {
            Some(platform) => platform,
            None => return Err(RunErr::new(1, "No platform found in state")), // TODO return more meaningful error code
        };

        let count = match platform.as_str() {
            "reddit" => count_reddit_comments(url),
            "hackernews" => count_hackernews_comments(url),
            _ => Err(RunErr::new(1, "Invalid platform")), // TODO return more meaningful error code
        }?;

        Ok(String::from(format!("{}", count)).as_bytes().to_vec())
    }
}

fn count_reddit_comments(_: String) -> Result<u64, RunErr> {
    Ok(0)
    // let headers = BTreeMap::new();
    // headers.insert("User-Agent", "comment-server");
}

fn count_hackernews_comments(url: String) -> Result<u64, RunErr> {
    // Build url for Hacker News API
    let id = url.split("?id=").last();
    if let None = id {
        return Err(RunErr::new(1, "Invalid Hacker News URL")); // TODO return more meaningful error code
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
        Err(_) => return Err(RunErr::new(1, "Failed to serialize json")), // TODO return more meaningful error code
    };
    let descendants = &post["descendants"];
    match descendants {
        Value::Number(num) => Ok(num.as_u64().unwrap()),
        _ => Err(RunErr::new(1, "Field \"descendants\" is not a number")), // TODO return more meaningful error code
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &CountComments = &CountComments {};

#[no_mangle]
pub extern "C" fn init() {
    use_runnable(RUNNABLE);
}
