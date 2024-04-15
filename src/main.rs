use std::env;
use std::process::exit;
use std::{fs, io};

fn main() -> io::Result<()> {
    let list_args: Vec<(usize, String)> = env::args().enumerate().collect();

    if list_args.len() < 2 {
        println!("no args passed...!");
        exit(0);
    }

    // cli command
    let mut cmd = list_args[1].clone();
    if !cmd.1.starts_with("--") {
        println!("wrong cmd passed...!");
        exit(0);
    }
    let cmd_parsed = cmd.1.split_off(2);

    match cmd_parsed.as_str() {
        "ls" => {
            let _ = print_dir();
        }
        "mk" => {
            if list_args.len() < 3 {
                println!("no mk args passed...!");
                exit(0);
            }
            let cmd_args = list_args[2].clone();
            let cmd_args_parsed = cmd_args.1.trim();

            mk_cmd(cmd_args_parsed.to_string());
        }
        _ => {
            exit(0);
        }
    }

    Ok(())
}

fn mk_cmd(args: String) {
    if args.ends_with("/") {
        fs::create_dir_all(args).unwrap_or_else(|_| {
            println!("unable to create dir");
            exit(0);
        });
        println!("dir created");
    } else {
        fs::File::create_new(args).unwrap_or_else(|_| {
            println!("uable to create file already exists...!");
            exit(0);
        });
        println!("file created");
    }
}

fn print_dir() -> io::Result<()> {
    let dir_path = "./";

    for entry in fs::read_dir(dir_path)? {
        let d = entry?;
        let epath = d.path();

        let fname = epath.file_name().unwrap();

        if epath.is_dir() {
            println!("/{:?}", fname);
        } else {
            println!("{:?}", fname)
        }
    }

    Ok(())
}
