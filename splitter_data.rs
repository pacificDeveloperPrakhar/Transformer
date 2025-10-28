use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn split_csv_by_size(input_file: &str, output_dir: &str, chunk_size_mb: usize) -> io::Result<()> {
    // Ensure output directory exists
    fs::create_dir_all(output_dir)?;

    let chunk_size_bytes = chunk_size_mb * 1024 * 1024;
    let mut file_number = 1;
    let mut current_chunk_size: usize = 0;

    let input = File::open(input_file)?;
    let mut reader = BufReader::new(input);

    let mut header = String::new();
    if reader.read_line(&mut header)? == 0 {
        eprintln!("❌ The input file is empty.");
        return Ok(());
    }

    // Create first output file
    let mut out_path = format!("{}/part_{}.csv", output_dir, file_number);
    let mut writer = BufWriter::new(File::create(&out_path)?);
    writer.write_all(header.as_bytes())?;
    current_chunk_size = header.as_bytes().len();

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let line_size = line.as_bytes().len();

        if current_chunk_size + line_size > chunk_size_bytes {
            writer.flush()?;
            file_number += 1;
            out_path = format!("{}/part_{}.csv", output_dir, file_number);
            writer = BufWriter::new(File::create(&out_path)?);
            writer.write_all(header.as_bytes())?;
            current_chunk_size = header.as_bytes().len();
        }

        writer.write_all(line.as_bytes())?;
        current_chunk_size += line_size;
        line.clear();
    }

    writer.flush()?;
    println!("✅ Done! Created {} smaller files in '{}'.", file_number, output_dir);

    Ok(())
}

fn main() -> io::Result<()> {
    let input_file = "data/english_german.csv";  // path to your 1.3 GB CSV
    let output_dir = "data/split_csv_parts";     // directory to save parts
    let chunk_size_mb = 10;                 // each part ≈10 MB

    split_csv_by_size(input_file, output_dir, chunk_size_mb)
}
