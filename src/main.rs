use std::path::Path;
use std::process::Command;

use clap::Parser;

use mimalloc_bench::Allocator;
use mimalloc_bench::Benchmark;

#[derive(Parser)]
enum Cli {
    Record {
        #[arg(short, long)]
        allocator: Allocator,

        #[arg(short, long)]
        benchmark: Benchmark,

        #[arg(short, long)]
        release: bool,
    },
}

fn main() {
    match Cli::parse() {
        Cli::Record {
            allocator,
            benchmark,
            release,
        } => {
            let path = Path::new("extern")
                .join(allocator.path(release))
                .canonicalize()
                .unwrap();

            let status = Command::new("rr")
                .arg("record")
                .arg(format!("--env=LD_PRELOAD={}", path.display()))
                .arg(benchmark.path())
                .args(benchmark.args())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            assert!(status.success());
        }
    }
}
