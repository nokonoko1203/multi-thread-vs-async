use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn process_line(line: &str) -> String {
    format!("{:x}", md5::compute(line))
}

pub fn cpu_bound_process() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    let reader = BufReader::new(File::open("output/large_file.txt")?);
    let mut writer = BufWriter::new(File::create("output/output_single.txt")?);

    for line in reader.lines() {
        let line = line?;
        let result = process_line(&line);
        writeln!(writer, "{}", result)?;
    }

    let duration = start.elapsed();
    println!("Time taken (Single Thread): {:?}", duration);

    Ok(())
}

fn generate_files(file_count: usize, lines_per_file: usize) -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    for i in 0..file_count {
        let mut file = BufWriter::new(File::create(format!("output/file_{}.txt", i))?);

        for _ in 0..lines_per_file {
            writeln!(file, "{}", rng.gen::<u64>())?;
        }
    }

    Ok(())
}

pub fn io_bound_process() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    let file_count = 10000;
    let lines_per_file = 10000;

    generate_files(file_count, lines_per_file)?;

    let duration = start.elapsed();
    println!("Time taken (Single Thread): {:?}", duration);

    Ok(())
}
