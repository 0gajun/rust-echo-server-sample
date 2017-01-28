extern crate getopts;
extern crate rust_unix_socket_sample;

use std::env;
use std::path::Path;
use std::process::exit;
use getopts::Options;

struct Context {
    is_server: bool,
    socket_path: String,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} SOCKET_PATH [options]", program);
    print!("{}", opts.usage(&brief));
}

fn extract_program_name(program_path: &str) -> Result<&str, &str> {
    let file_name =
        try!(Path::new(program_path).file_name().ok_or("Cannot get executable file_name"));
    return Ok(try!(file_name.to_str().ok_or("Cannot convert osStr into str")));
}

fn parse_args() -> Result<Context, String> {
    let args: Vec<String> = env::args().collect();

    let program = match extract_program_name(&args[0]) {
        Ok(program) => program,
        Err(err) => return Err(err.to_string()),
    };

    let mut opts = Options::new();
    opts.optflag("s", "server", "launch as server (by default, launch as client)");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => return Err(err.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(program, opts);
        return Err("".to_string());
    }

    if matches.free.is_empty() {
        print_usage(program, opts);
        return Err("".to_string());
    }

    let socket_path = matches.free[0].clone();

    Ok(Context {
        is_server: matches.opt_present("s"),
        socket_path: socket_path,
    })
}

fn main() {
    let context = match parse_args() {
        Ok(context) => context,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    if context.is_server {
        rust_unix_socket_sample::launch_server(&context.socket_path);
    } else {
        rust_unix_socket_sample::launch_client(&context.socket_path);
    }
}
