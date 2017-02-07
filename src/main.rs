// -*- coding: utf-8 -*-
// src/tmp.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Externs

// Stdlib imports
use std::collections::HashMap;
use std::io;

// Third-party imports

// Local imports


// ===========================================================================
//
// ===========================================================================


fn fib(cache: &mut HashMap<usize, usize>, i: usize) -> usize {
    // Compute val
    match cache.get(&i) {
        Some(&val) => val,
        None if i <= 1 => {
            cache.insert(i, i);
            i
        },
        None => {
            let ret = fib(cache, i-2) + fib(cache, i-1);
            cache.insert(i, ret);
            ret
        },
    }
}

fn main() {
    println!("Fibonacci calculator");
    println!("n:");

    let mut store = HashMap::new();

    loop {
        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // Exit if empty line received, works for Linux, MacOS, or Windows
        let quitvals = vec!["\r", "\n", "\r\n"];
        if quitvals.contains(&input.as_str()) {
            println!("Exiting...");
            break;
        }

        // Convert input to usize
        let n: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let ret: usize = fib(&mut store, n);
        println!("Fib({}) = {}", n, ret);
    }

}


// ===========================================================================
//
// ===========================================================================
