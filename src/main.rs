use std::path::Path;

use clap::Parser;
use clap::ValueEnum;
use duct::cmd;

use mimalloc_bench::Allocator;
use mimalloc_bench::Benchmark;

#[derive(Parser)]
enum Cli {
    Run {
        #[arg(short, long)]
        wrap: Option<Wrap>,

        #[arg(short, long)]
        allocator: Allocator,

        #[arg(short, long)]
        benchmark: Benchmark,

        #[arg(short, long)]
        release: bool,
    },
}

#[derive(Copy, Clone, ValueEnum)]
enum Wrap {
    Rr,
    Perf,
}

fn main() {
    match Cli::parse() {
        Cli::Run {
            wrap,
            allocator,
            benchmark,
            release,
        } => {
            let path = Path::new("extern")
                .join(allocator.path(release))
                .canonicalize()
                .unwrap();

            let ld = format!("LD_PRELOAD={}", path.display());

            match wrap {
                None => cmd!["env", ld],
                Some(Wrap::Rr) => cmd!["rr", "record", format!("--env={}", ld)],
                Some(Wrap::Perf) => {
                    cmd![
                        "perf",
                        "record",
                        "--call-graph",
                        "dwarf",
                        "-o",
                        "perf.data",
                        "env",
                        ld,
                    ]
                }
            }
            .before_spawn(move |command| {
                command.arg(benchmark.path());
                command.args(benchmark.args());
                Ok(())
            })
            .run()
            .unwrap();

            if let Some(Wrap::Perf) = wrap {
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
}
