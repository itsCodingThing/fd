mod cmd;

use std::env;
use std::process::exit;

use cmd::{Cmd, Cmds};

fn main() {
    let mut list_args: Vec<String> = env::args().collect();
    list_args.remove(0);

    let cmd_parsed = match Cmd::parse_cmd(list_args.clone()) {
        Some(parsed) => parsed,
        None => {
            println!("invalid cmd passed...!");
            exit(0);
        }
    };

    match cmd_parsed.cmd {
        Cmds::Ls => {
            let _ = Cmd::print_cmd();
        }
        Cmds::Mkd => {
            if list_args.len() < 3 {
                println!("no mk args passed...!");
                exit(0);
            }
            let cmd_args = list_args[2].clone();
            let cmd_args_parsed = cmd_args.trim();

            Cmd::mkd_cmd(cmd_args_parsed.to_string());
        }
        Cmds::Mkf => {
            if list_args.len() < 3 {
                println!("no mk args passed...!");
                exit(0);
            }
            let cmd_args = list_args[2].clone();
            let cmd_args_parsed = cmd_args.trim();

            Cmd::mkf_cmd(cmd_args_parsed.to_string());
        }
        Cmds::Rm => {
            if list_args.len() < 3 {
                println!("no rm args passed...!");
                exit(0);
            }
            let cmd_args = list_args[2].clone();
            let cmd_args_parsed = cmd_args.trim();

            Cmd::rm_cmd(cmd_args_parsed.to_string());
        }
    }
}
