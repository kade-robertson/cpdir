use std::path::PathBuf;

use clap::Parser;
use walk_dir::WalkableDir;

mod walk_dir;

#[derive(Parser)]
struct Args {
    /// The directory to walk to determine the structure to be copied to the
    /// destination.
    source: PathBuf,

    /// Controls the recursion depth of the directory walking process.
    /// The allowed range is 0-255. Setting this to 0 will effectively not walk
    /// the source directory.
    #[arg(short, long, default_value_t = 255)]
    depth: u8,
}

fn main() {
    let args = Args::parse();

    println!("Walking {:?}", &args.source);
    let walkable = WalkableDir::new(args.source, args.depth);

    walkable.for_each(|d| {
        println!(" - {:?}", &d);
    });
}
