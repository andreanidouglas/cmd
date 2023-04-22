use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cmd {
    source: String,
    destination: String,

    #[arg(short)]
    recursive: bool,

    #[arg(short)]
    verbose: bool,
}

fn verbose_print(verbose: bool, printable: &str) {
    if verbose {
        eprintln!("{}", printable);
    }

}

fn copy_file(dst: &Path, src: &Path, verbose: bool) -> anyhow::Result<u64> {
    match std::fs::copy(src, dst) {
        Ok(count) => {
            verbose_print(verbose, &format!("cp {} {}", src.to_string_lossy(), dst.to_string_lossy()));
            Ok(count)
        },
        Err(e) => anyhow::bail!(format!("could not copy file {} to {}. {}", src.to_string_lossy(), dst.to_string_lossy(), e))
    }
}

fn copy_dir_to(dst: &Path, src: &Path, verbose: bool) -> anyhow::Result<u64> {
    let dst_path = Path::new(dst);

    if ! dst_path.exists() {
        verbose_print(verbose, &format!("mkdir {}", dst.to_string_lossy()));
        std::fs::create_dir(dst)?;
    }
    if ! dst_path.is_dir() {
        anyhow::bail!(format!("could not copy {} over {}", src.to_string_lossy(), dst.to_string_lossy()))
    }

    let mut dst_path = dst_path.to_path_buf();

    if let Ok(contents) = src.read_dir() {
        for content in contents.flatten() {
            if let Ok(t) = content.file_type() {
                if t.is_dir() {
                    verbose_print(verbose, &format!("cp {} {}", content.path().as_path().to_string_lossy(), dst_path.as_path().to_string_lossy()));
                    dst_path.push(content.file_name());
                    copy_dir_to(dst_path.as_path(), content.path().as_path(), verbose)?;
                } else {
                    dst_path.push(content.file_name());
                    copy_file(dst_path.as_path(), content.path().as_path(), verbose)?;
                }
            }
        }
    }

    Ok(0)
}

fn main() -> anyhow::Result<()> {
    let args = Cmd::parse();

    let src_path = Path::new(&args.source);
    if src_path.is_dir() {
        if ! args.recursive {
            eprintln!("-r not specified; omitting directory '{}'", src_path.to_string_lossy());
            return Ok(())
        } else {
            let dst = Path::new(&args.destination);
            copy_dir_to(&dst, src_path, args.verbose)?;
        }
    } else {
        let src = Path::new(&args.source);
        let dst = Path::new(&args.destination);
        copy_file(dst, src, args.verbose)?;
    }
    Ok(())
}