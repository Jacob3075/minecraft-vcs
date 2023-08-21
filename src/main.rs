use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::arguments::Arguments;

mod arguments;
mod authentication;
mod configuration;
mod drive;

fn main() {
    let args = Arguments::parse();
    dbg!(args);
}
