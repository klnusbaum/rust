// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-cloudabi spawning processes is not supported
// ignore-emscripten spawning processes is not supported

use std::{env, process};

fn child() {
    print!("[stdout 0]");
    print!("[stdout {}]", 1);
    println!("[stdout {}]", 2);
    println!();
    eprint!("[stderr 0]");
    eprint!("[stderr {}]", 1);
    eprintln!("[stderr {}]", 2);
    eprintln!();
}

fn parent() {
    let this = env::args().next().unwrap();
    let output = process::Command::new(this).arg("-").output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert_eq!(stdout, "[stdout 0][stdout 1][stdout 2]\n\n");
    assert_eq!(stderr, "[stderr 0][stderr 1][stderr 2]\n\n");
}

fn main() {
    if env::args().count() == 2 { child() } else { parent() }
}
