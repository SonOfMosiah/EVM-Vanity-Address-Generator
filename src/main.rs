extern crate evm_vanity_address_generator;

use std::env;
use std::process;

use evm_vanity_address_generator::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if config.gpu_device == 255 {
	    if let Err(e) = evm_vanity_address_generator::cpu(config) {
	        eprintln!("CPU application error: {}", e);
	        process::exit(1);
	    }
    } else {
	    if let Err(e) = evm_vanity_address_generator::gpu(config) {
	        eprintln!("GPU application error: {}", e);
	        process::exit(1);
	    }
    }
}