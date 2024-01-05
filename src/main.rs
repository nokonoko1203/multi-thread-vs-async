use rand::Rng;
use std::fs::File;
use std::io::{BufWriter, Write};

use app::{
    async_process::cpu_bound_process as async_cpu_bound_process,
    async_process::io_bound_process as async_io_bound_process,
    multi_thread::cpu_bound_process as multi_thread_cpu_bound_process,
    multi_thread::io_bound_process as multi_thread_io_bound_process,
    single_thread::cpu_bound_process as single_thread_cpu_bound_process,
    single_thread::io_bound_process as single_thread_io_bound_process,
};

async fn generate_large_file(file_name: &str, lines: usize) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(file_name)?);
    let mut rng = rand::thread_rng();

    for _ in 0..lines {
        writeln!(file, "{}", rng.gen::<u64>())?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("output")?;

    for entry in std::fs::read_dir("output")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "txt" {
            std::fs::remove_file(path)?;
        }
    }

    // println!("Generating large file...");
    // generate_large_file("output/large_file.txt", 10_000_000).await?;

    // println!("Processing cpu bound task...");
    // single_thread_cpu_bound_process()?;
    // multi_thread_cpu_bound_process()?;
    // async_cpu_bound_process().await?;

    println!("Processing io bound task...");
    // single_thread_io_bound_process()?;
    // multi_thread_io_bound_process()?;
    async_io_bound_process().await?;

    Ok(())
}
