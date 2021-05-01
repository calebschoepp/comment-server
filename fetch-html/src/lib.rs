use suborbital::req;
use suborbital::runnable::*;

struct FetchHtml {}

impl Runnable for FetchHtml {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let validate_state = req::state("validate");
        if let None = validate_state {
            return Err(RunErr::new(1, "No state produced by validate")); // TODO return more meaningful error code
        }
        let validate_state = validate_state.unwrap().split(",");
        // let platform

        Ok(String::from(format!("success:\n",)).as_bytes().to_vec())
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &FetchHtml = &FetchHtml {};

#[no_mangle]
pub extern "C" fn init() {
    use_runnable(RUNNABLE);
}
