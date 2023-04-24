#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::single_match_else)]

#[macro_use]
extern crate rocket;

mod logger;
mod server;

fn main() {
    logger::init();

    server::run().unwrap();
}
