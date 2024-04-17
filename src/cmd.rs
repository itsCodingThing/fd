use std::fs;
use std::process::exit;

pub enum Cmds {
    Ls,
    Mkd,
    Mkf,
    Rm,
}

pub struct CmdParsed {
    pub cmd: Cmds,
    pub args: String,
}

pub struct Cmd;
impl Cmd {
    pub fn parse_cmd(args: Vec<String>) -> Option<CmdParsed> {
        let copy_args = args.clone();
        if args.len() < 1 || !args[0].starts_with("--") {
            println!("wrong cmd passed...!");
            return None;
        }

        let dirty_cmd = copy_args[0].clone().split_off(2);
        let cmd_input = copy_args[1].clone().trim().to_string();
        match dirty_cmd.as_str() {
            "ls" => {
                return Some(CmdParsed {
                    cmd: Cmds::Ls,
                    args: cmd_input,
                });
            }
            "mkd" => {
                return Some(CmdParsed {
                    cmd: Cmds::Mkd,
                    args: cmd_input,
                });
            }
            "mkf" => {
                return Some(CmdParsed {
                    cmd: Cmds::Mkf,
                    args: cmd_input,
                });
            }
            "rm" => {
                return Some(CmdParsed {
                    cmd: Cmds::Rm,
                    args: cmd_input,
                });
            }
            _ => {
                return None;
            }
        }
    }

    pub fn rm_cmd(args: String) {
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

    pub fn mkd_cmd(args: String) {
        fs::create_dir_all(args).unwrap_or_else(|_| {
            println!("unable to create dir");
            exit(0);
        });
        println!("dir created");
    }

    pub fn mkf_cmd(args: String) {
        fs::File::create_new(args).unwrap_or_else(|_| {
            println!("uable to create file already exists...!");
            exit(0);
        });
        println!("file created");
    }

    pub fn print_cmd() {
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
}
