use std::{
    fs::{self, DirEntry, ReadDir},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cmd {
    path: String,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short='i', long, action)]
    case_insensitive: bool,
}

struct Find {
    folders: Vec<DirEntry>,
    case_insensitivity: bool
}
impl Find {
    fn read_dir(&self, path: &PathBuf) -> anyhow::Result<ReadDir> {
        match fs::read_dir(path) {
            Ok(rd) => Ok(rd),
            Err(e) => Err(anyhow::format_err!("could not read file: \"{}\". {}", path.to_str().unwrap(), e.to_string())),
        }
    }

    fn find(&mut self, path: String, name: Option<String>) -> anyhow::Result<()> {
        let fs_path = PathBuf::from_str(path.as_str())?;
        let mut read_dir = self.read_dir(&fs_path)?;
        
        let name = name.unwrap_or("*".into());
        'inner: loop {
            match read_dir.next() {
                None => {
                    read_dir = match self.folders.pop() {
                        None => break 'inner,
                        Some(entry) => {
                            match self.read_dir(&entry.path()) {
                                Ok(rd) => rd,
                                Err(e) => {
                                    eprintln!("{}", e.to_string());
                                    continue
                                },
                            }
                        }
                    }
                }
                Some(entry) => {
                    let entry = entry?;
                    match entry.file_type().unwrap().is_dir() {
                        true => self.folders.push(entry),
                        false => {
                            let file_name = entry.path().to_str().unwrap().to_string();
                            if match_name(file_name, &name, self.case_insensitivity) {
                                println!("{}", entry.path().to_str().unwrap());
                            }
                        },
                    };
                }
            };
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cmd::parse();
    let mut f = Find {
        folders: Vec::new(),
        case_insensitivity: args.case_insensitive,
    };

    f.find(args.path, args.name)
}


fn match_name(file_name: String, name: &String, case_insensitivity: bool) -> bool {
  
    if name.is_empty() || name == "*" {
        true
    } else if case_insensitivity {
        return file_name.to_lowercase().contains(&name.to_lowercase());
    }
    else {
        return file_name.contains(name);
    }
}
