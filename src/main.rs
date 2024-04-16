use std::env;
use std::fs;
use std::process::exit;

fn main() {
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
        "rm" => {
            if list_args.len() < 3 {
                println!("no rm args passed...!");
                exit(0);
            }
            let cmd_args = list_args[2].clone();
            let cmd_args_parsed = cmd_args.1.trim();

            rm_cmd(cmd_args_parsed.to_string());
        }
        _ => {
            println!("invalid cmds...!");
        }
    }
}

fn rm_cmd(args: String) {
    if args.ends_with("/") {
        fs::remove_dir_all(args).unwrap_or_else(|_| {
            println!("unable to delete dir.");
            exit(0);
        });
        println!("dir removed.");
    } else {
        fs::remove_file(args).unwrap_or_else(|_| {
            println!("unable to remove file.");
            exit(0);
        });
        println!("file removed.");
    }
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

fn print_dir() {
    let dir_path = "./";

    let entry_itr = fs::read_dir(dir_path).unwrap_or_else(|_| {
        println!("unable to read dir.");
        exit(0);
    });

    for entry in entry_itr {
        let d = entry.unwrap_or_else(|_| {
            exit(0);
        });
        let epath = d.path();
        let fname = epath.file_name().unwrap();

        if epath.is_dir() {
            println!("/{:?}", fname);
        } else {
            println!("{:?}", fname)
        }
    }
}
