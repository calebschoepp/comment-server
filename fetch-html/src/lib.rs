use suborbital::http;
use suborbital::req;
use suborbital::runnable::*;

struct FetchHtml {}

impl Runnable for FetchHtml {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let url = match req::state("url") {
            Some(url) => url,
            None => return Err(RunErr::new(1, "No url found in state")), // TODO return more meaningful error code
        };
        http::get(&url, None)
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &FetchHtml = &FetchHtml {};

#[no_mangle]
pub extern "C" fn init() {
    use_runnable(RUNNABLE);
}
