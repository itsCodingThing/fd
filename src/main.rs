mod cmd;

use std::env;
use std::process::exit;

use cmd::Cmd;

fn main() {
    let list_args: Vec<String> = env::args().collect();

    let _cmd_args = Cmd::parse_cmd(list_args.clone());
    if list_args.len() < 2 {
        println!("no args passed...!");
        exit(0);
    }

    // cli command
    let mut cmd = list_args[1].clone();
    if !cmd.starts_with("--") {
        println!("wrong cmd passed...!");
        exit(0);
    }
    let cmd_parsed = cmd.split_off(2);

    match cmd_parsed.as_str() {
        "ls" => {
            let _ = Cmd::print_cmd();
        }
        "mkd" => {
            if list_args.len() < 3 {
                println!("no mk args passed...!");
                exit(0);
            }
            let cmd_args = list_args[2].clone();
            let cmd_args_parsed = cmd_args.trim();

            Cmd::mkd_cmd(cmd_args_parsed.to_string());
        }
        "mkf" => {
            if list_args.len() < 3 {
                println!("no mk args passed...!");
                exit(0);
            }
            let cmd_args = list_args[2].clone();
            let cmd_args_parsed = cmd_args.trim();

            Cmd::mkf_cmd(cmd_args_parsed.to_string());
        }
        "rm" => {
            if list_args.len() < 3 {
                println!("no rm args passed...!");
                exit(0);
            }
            let cmd_args = list_args[2].clone();
            let cmd_args_parsed = cmd_args.trim();

            Cmd::rm_cmd(cmd_args_parsed.to_string());
        }
        _ => {
            println!("invalid cmds...!");
        }
    }
}
