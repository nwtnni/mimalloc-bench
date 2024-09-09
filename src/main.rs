use std::path::Path;

use clap::Parser;
use duct::cmd;

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

    Perf {
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

            cmd![
                "rr",
                "record",
                format!("--env=LD_PRELOAD={}", path.display()),
                benchmark.path(),
            ]
            .before_spawn(move |command| {
                command.args(benchmark.args());
                Ok(())
            })
            .run()
            .unwrap();
        }
        Cli::Perf {
            allocator,
            benchmark,
            release,
        } => {
            let path = Path::new("extern")
                .join(allocator.path(release))
                .canonicalize()
                .unwrap();

            cmd![
                "perf",
                "record",
                "--call-graph",
                "dwarf",
                "-o",
                "perf.data",
                "env",
                format!("LD_PRELOAD={}", path.display()),
                benchmark.path(),
            ]
            .before_spawn(move |command| {
                command.args(benchmark.args());
                Ok(())
            })
            .run()
            .unwrap();

            cmd!["perf", "script", "--input", "perf.data"]
                .pipe(cmd!(std::env::home_dir()
                    .unwrap()
                    .join(".cargo/bin/inferno-collapse-perf")))
                .pipe(cmd!(std::env::home_dir()
                    .unwrap()
                    .join(".cargo/bin/inferno-flamegraph")))
                .stdout_path("out.svg")
                .run()
                .unwrap();
        }
    }
}
