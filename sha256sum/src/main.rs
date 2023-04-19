use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use anyhow::Context;
use clap::Parser;
use sha256::try_digest;

#[derive(Parser)]
struct Cmd {
    files: Vec<String>,

    #[arg(short)]
    check: bool,
}

fn hash_file(path: &Path, quiet: bool) -> anyhow::Result<String> {
    let hash = try_digest(path).context(format!(
        "could not calculate hash of file: '{}'",
        path.to_str().unwrap()
    ))?;

    if !quiet {
        println!("{}  {}", hash, path.to_str().unwrap());
    }
    Ok(hash)
}

fn check_file(path: &Path) -> anyhow::Result<bool> {
    let mut count = 0usize;

    let file =
        File::open(path).context(format!("could not open file: {}", path.to_string_lossy()))?;
    let lines = io::BufReader::new(file).lines();

    for content in lines.flatten() {
        let mut parts = content.split("  ");
        let hash = parts.next().unwrap();
        let file_path = parts.next().unwrap().trim();
        let hashed = hash_file(Path::new(file_path), true)?;
        if hashed == hash {
            println!("{}: OK", file_path);
        } else {
            println!("{}: FAILED", file_path);
            count += 1;
        }
    }

    if count > 0 {
        eprintln!(
            "sha256sum: WARNING: {} computed checksum did NOT match",
            count
        );
        return Ok(false);
    }
    Ok(true)
}

fn main() -> anyhow::Result<()> {
    let args = Cmd::parse();

    for f in args.files {
        let path = Path::new(&f);

        if args.check {
            check_file(path)?;
        } else {
            hash_file(path, false)?;
        }
    }

    Ok(())
}
