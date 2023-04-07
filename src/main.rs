use std::{fs::create_dir_all, path::PathBuf, process::exit};

use clap::Parser;
use walk_dir::WalkableDir;

mod walk_dir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to walk to determine the structure to be copied to the
    /// destination.
    source: PathBuf,

    /// The directory to copy walked paths into.
    dest: PathBuf,

    /// Controls the recursion depth of the directory walking process.
    /// The allowed range is 0-255. Setting this to 0 will effectively not walk
    /// the source directory.
    #[arg(short, long, default_value_t = 255)]
    depth: u8,

    /// Controls whether folders should actually be created at the destination.
    /// This defaults to false, so this program will run in a "dry-run" mode
    /// unless this argument is specified.
    #[arg(short = 'x', long, default_value_t = false)]
    execute: bool,

    /// Whether or not the destination directory is allowed to be a subdirectory
    /// of the source. This can have undesired behaviour, as directories within
    /// the subdirectory then get copied deeper into the same subdirectory.
    #[arg(long, default_value_t = false)]
    allow_nesting: bool,

    /// Whether or not a non-zero exit code should be used in the case that any
    /// directory is not able to be moved.
    #[arg(short = 'E', long, default_value_t = false)]
    exit_code_failure: bool,
}

fn main() {
    let args = Args::parse();

    if !args.allow_nesting && args.dest.starts_with(&args.source) {
        eprintln!("WARN: Your destination directory is a subdirectory of your source directory.");
        eprintln!("WARN: This can have unintended side effects.");
        eprintln!("WARN: If you would like to proceed, specify the --allow-nesting option.");

        if args.execute {
            exit(1);
        }
    }

    let mut did_fail = false;

    println!("INFO: Walking {:?}", &args.source);
    let walkable = WalkableDir::new(&args.source, args.depth);
    walkable.for_each(|d| {
        let dest_dir = if let Ok(stripped_dir) = d.strip_prefix(&args.source) {
            args.dest.join(stripped_dir)
        } else {
            eprintln!(
                "ERROR: Directory {d:?} is not a child of {source:?}. Skipping.",
                source = args.source
            );
            if args.exit_code_failure {
                did_fail = true;
            }
            return;
        };

        if args.execute {
            println!("INFO: Creating directory {dest_dir:?}");
            create_dir_all(&dest_dir).unwrap_or_else(|e| {
                eprintln!("ERROR: Could not create directory {dest_dir:?}: {e:?}");
                if args.exit_code_failure {
                    did_fail = true;
                }
            });
        } else {
            println!("DRY: Creating directory {d:?} -> {dest_dir:?}");
        }

        if did_fail {
            exit(2);
        }
    });
}
