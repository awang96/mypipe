extern crate clap;

use clap::{
    App,
    Arg
};
use std::process::{
    Command,
    Stdio
};
use std::error::Error;
use std::io::prelude::*;


macro_rules! unwrap_stdout {
    ( $process:expr ; $s:expr ) => {
        {
            match $process.stdout.unwrap()
                .read_to_string(&mut $s)
                {
                    Err(why) => panic!(
                        "Couldn't read stdout: {}",
                        why.description()
                    ),
                    Ok(_) => $s
                }
        };
    }
}

macro_rules! spawn_cmd {
    ( $cmd:expr ) => {
        {
            match Command::new($cmd)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                {
                    Err(why) => panic!(
                        "Couldn't spawn: {}",
                        why.description()
                    ),
                    Ok(process) => process
                }
        };
    }
}

macro_rules! get_args {
    ( $x:expr; $match:expr ) => {
        {
            $match.value_of($x)
                .unwrap()
                .to_string()
        };
    }
}


fn main() {

    let matches = App::new("mypipe")
        .version("0.0.1")
        .about("Custom pipe")
        .author("Alexandre WANG")
        .arg(Arg::with_name("input")
            .short("i")
            .long("in")
            .help("First command")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .help("Second command")
            .required(true)
            .takes_value(true)
        )
        .get_matches();

    let input = get_args!("input"; matches);
    let output = get_args!("output"; matches);

    let process_in = spawn_cmd!(input);

    let mut input_res = String::new();
    input_res = unwrap_stdout!(process_in; input_res);

    let process_out = spawn_cmd!(output);

    match process_out.stdin.unwrap()
        .write_all(input_res.as_bytes())
        {
            Err(why) => panic!("Couldn't write to output stdin: {}",
                why.description()
            ),
            Ok(_) => print!(""),
        }

    let mut output_res = String::new();

    println!("{}", unwrap_stdout!(process_out; output_res).to_string());

}
