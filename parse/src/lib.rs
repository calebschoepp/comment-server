use scraper::Html;
use suborbital::req;
use suborbital::runnable::*;

struct Parse {}

impl Runnable for Parse {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let html = match req::state("html") {
            Some(html) => html,
            None => return Err(RunErr::new(1, "No html found in state")), // TODO return more meaningful error code
        };

        Html::parse_document(&html);

        Ok(String::from(format!("hello {}", html)).as_bytes().to_vec())
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &Parse = &Parse {};

#[no_mangle]
pub extern "C" fn init() {
    use_runnable(RUNNABLE);
}
