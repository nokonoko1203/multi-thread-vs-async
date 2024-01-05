use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn process_line(line: &str) -> String {
    format!("{:x}", md5::compute(line))
}

pub fn cpu_bound_process() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    let reader = BufReader::new(File::open("output/large_file.txt")?);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let processed_lines: Vec<String> = lines.par_iter().map(|line| process_line(line)).collect();

    let mut writer = BufWriter::new(File::create("output/output_parallel.txt")?);
    for line in processed_lines {
        writeln!(writer, "{}", line)?;
    }

    let duration = start.elapsed();
    println!("Time taken (Parallel): {:?}", duration);

    Ok(())
}

fn generate_file(i: usize, lines_per_file: usize) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(format!("output/file_{}.txt", i))?);
    let mut rng = StdRng::from_entropy();

    for _ in 0..lines_per_file {
        writeln!(file, "{}", rng.gen::<u64>())?;
    }

    Ok(())
}

pub fn io_bound_process() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    (0..10000).into_par_iter().for_each(|i| {
        generate_file(i, 10000).unwrap();
    });

    let duration = start.elapsed();
    println!("Time taken (Parallel): {:?}", duration);

    Ok(())
}
