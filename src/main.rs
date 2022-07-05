extern crate syslog;
extern crate log;
#[macro_use]
extern crate rocket;

use log::LevelFilter;
use syslog::{BasicLogger, Formatter3164};

#[get("/")]
fn get_test() -> &'static str {
    "test!"
}

#[launch]
async fn async_main() -> _ {
    let formatter = Formatter3164::default();

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("could not register logger");

    rocket::build().mount("/", routes![get_test])
}
