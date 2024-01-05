use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

async fn process_line_async(line: String) -> String {
    format!("{:x}", md5::compute(line))
}

pub async fn cpu_bound_process() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    let file = File::open("output/large_file.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let file_output = File::create("output/output_async.txt").await?;
    let mut writer = BufWriter::new(file_output);

    while let Some(line) = lines.next_line().await? {
        let result = process_line_async(line).await;
        writer.write_all(result.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    let duration = start.elapsed();
    println!("Time taken (Async): {:?}", duration);

    Ok(())
}

async fn generate_file_async(i: usize, lines_per_file: usize) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(format!("output/file_{}.txt", i)).await?);
    let mut rng = StdRng::from_entropy();

    for _ in 0..lines_per_file {
        file.write_all(format!("{}\n", rng.gen::<u64>()).as_bytes())
            .await?;
    }

    file.flush().await
}

pub async fn io_bound_process() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    let mut handles = Vec::new();
    for i in 0..10000 {
        let handle = tokio::spawn(generate_file_async(i, 10000));
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await?;
    }

    let duration = start.elapsed();
    println!("Time taken (Async): {:?}", duration);

    Ok(())
}
