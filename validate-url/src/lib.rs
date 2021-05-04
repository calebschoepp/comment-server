use base64;
use suborbital::req::*;
use suborbital::runnable::*;
use urlencoding;

struct ValidateUrl {}

const ERR_CODE: i32 = 2; // Globally unique error code for the runnable

impl Runnable for ValidateUrl {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        // Setup
        let valid_platforms: Vec<Platform> = vec![
            Platform {
                name: "reddit",
                domain: "reddit.com",
            },
            Platform {
                name: "hackernews",
                domain: "news.ycombinator.com",
            },
        ];

        // Read url parameters
        let platform = url_param("platform");
        let url = url_param("url");
        let url = match base64::decode(url) {
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
            Err(err) => return Err(RunErr::new(ERR_CODE, &err.to_string())),
        };
        let url = match urlencoding::decode(&url) {
            Ok(decoded_url) => decoded_url,
            Err(err) => return Err(RunErr::new(ERR_CODE, &err.to_string())),
        };

        // Validate url
        if let false = valid_platforms
            .iter()
            .any(|p| platform == p.name && url.contains(p.domain))
        {
            return Err(RunErr::new(
                ERR_CODE,
                "Provided url is invalid or does not match platform",
            ));
        }

        Ok(String::from(format!("{}", url)).as_bytes().to_vec())
    }
}

struct Platform {
    name: &'static str,
    domain: &'static str,
}

// initialize the runner, do not edit below //
static RUNNABLE: &ValidateUrl = &ValidateUrl {};

#[no_mangle]
pub extern "C" fn init() {
    use_runnable(RUNNABLE);
}
