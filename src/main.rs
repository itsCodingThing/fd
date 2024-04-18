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
            Cmd::print_cmd();
        }
        Cmds::Mkd => {
            Cmd::mkd_cmd(cmd_parsed.args);
        }
        Cmds::Mkf => {
            Cmd::mkf_cmd(cmd_parsed.args);
        }
        Cmds::Rm => {
            Cmd::rm_cmd(cmd_parsed.args);
        }
        Cmds::Zp => {
            Cmd::zp_cmd();
        }
    }
}
