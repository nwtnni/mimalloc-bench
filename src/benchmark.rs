use std::path::Path;

use clap::ValueEnum;

#[derive(Copy, Clone, ValueEnum)]
pub enum Benchmark {
    Cfrac,
    Espresso,
    Barnes,
    Redis,
    Lean,
    LarsonSized,
    Mstress,
    Rptest,
    Gs,
    Lua,
    AllocTest,
    Sh6Bench,
    Sh8Bench,
    XmallocTest,
    Cscratch,
    GlibcSimple,
    GlibcThread,
    Rocksdb,
    Larson,
    LeanMathlib,
    Mleak,
    Rbstress,
    Cthrash,
    Z3,
}

impl Benchmark {
    pub fn path(&self) -> &'static Path {
        macro_rules! out {
            ($relative:expr) => {
                concat!("out/bench/", $relative)
            };
        }

        let path = match self {
            Benchmark::Cfrac => out!("cfrac"),
            Benchmark::Espresso => out!("espresso"),
            Benchmark::Barnes => out!("barnes"),
            Benchmark::Redis => todo!(),
            Benchmark::Lean => todo!(),
            Benchmark::LarsonSized => out!("larson-sized"),
            Benchmark::Mstress => out!("mstress"),
            Benchmark::Rptest => out!("rptest"),
            Benchmark::Gs => todo!(),
            Benchmark::Lua => todo!(),
            Benchmark::AllocTest => out!("alloc-test"),
            Benchmark::Sh6Bench => out!("sh6bench"),
            Benchmark::Sh8Bench => out!("sh8bench"),
            Benchmark::XmallocTest => out!("xmalloc-test"),
            Benchmark::Cscratch => out!("cache-scratch"),
            Benchmark::GlibcSimple => out!("glibc-simple"),
            Benchmark::GlibcThread => out!("glibc-thread"),
            Benchmark::Rocksdb => todo!(),
            Benchmark::Larson => out!("larson"),
            Benchmark::LeanMathlib => todo!(),
            Benchmark::Mleak => out!("mleak"),
            Benchmark::Rbstress => todo!(),
            Benchmark::Cthrash => out!("cache-thrash"),
            Benchmark::Z3 => todo!(),
        };

        Path::new(path)
    }

    pub fn args(&self) -> Vec<String> {
        let threads = std::thread::available_parallelism().unwrap().get();

        match self {
            Benchmark::Cfrac => vec![String::from("17545186520507317056371138836327483792789528")],
            Benchmark::Espresso => vec![String::from("bench/espresso/largest.espresso")],
            Benchmark::Barnes => vec![],
            Benchmark::Redis => todo!(),
            Benchmark::Lean => todo!(),
            Benchmark::LarsonSized => todo!(),
            Benchmark::Mstress => vec![threads.to_string(), String::from("50"), String::from("25")],
            Benchmark::Rptest => todo!(),
            Benchmark::Gs => todo!(),
            Benchmark::Lua => todo!(),
            Benchmark::AllocTest => todo!(),
            Benchmark::Sh6Bench => todo!(),
            Benchmark::Sh8Bench => todo!(),
            Benchmark::XmallocTest => todo!(),
            Benchmark::Cscratch => todo!(),
            Benchmark::GlibcSimple => todo!(),
            Benchmark::GlibcThread => todo!(),
            Benchmark::Rocksdb => todo!(),
            Benchmark::Larson => todo!(),
            Benchmark::LeanMathlib => todo!(),
            Benchmark::Mleak => todo!(),
            Benchmark::Rbstress => todo!(),
            Benchmark::Cthrash => todo!(),
            Benchmark::Z3 => todo!(),
        }
    }
}
