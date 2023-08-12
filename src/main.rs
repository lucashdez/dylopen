use std::process::Command;

use wrapper::*;
mod wrapper;

fn main() {
    let a = 0;
    Command::new("pwd");
}
