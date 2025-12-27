// Copyright (c) 2026 Burak Sezer
// All rights reserved.
//
// This code is licensed under the MIT License.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files(the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and / or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions :
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "packdir")]
#[command(about = "A CLI tool to compress directories using zstd", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a folder into a zstd archive
    Compress {
        /// Name of the resulting file (without extension)
        name: String,
        /// Path to the directory to compress
        directory: PathBuf,
        /// Destination directory for the compressed file
        destination: PathBuf,
    },
    /// Decompress a zstd archive
    Decompress {
        /// Path to the compressed file
        file: PathBuf,
        /// Destination directory for extraction
        destination: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { name, directory, destination } => {
            compress_folder(&name, &directory, &destination)?;
        }
        Commands::Decompress { file, destination } => {
            decompress_archive(&file, &destination)?;
        }
    }

    Ok(())
}

fn compress_folder(name: &str, directory: &PathBuf, destination: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !directory.exists() {
        return Err(format!("Directory '{}' does not exist", directory.display()).into());
    }

    if !directory.is_dir() {
        return Err(format!("'{}' is not a directory", directory.display()).into());
    }

    if !destination.exists() {
        return Err(format!("Destination '{}' does not exist", destination.display()).into());
    }

    if !destination.is_dir() {
        return Err(format!("Destination '{}' is not a directory", destination.display()).into());
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let output_filename = format!("{}-{}.zstd", name, timestamp);
    let output_path = destination.join(&output_filename);

    println!("Compressing '{}' to '{}'...", directory.display(), output_path.display());

    let output_file = File::create(&output_path)?;
    let buffered_writer = BufWriter::new(output_file);
    let zstd_encoder = zstd::stream::Encoder::new(buffered_writer, 0)?;
    let mut tar_builder = tar::Builder::new(zstd_encoder);

    let folder_name = directory
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "archive".to_string());

    tar_builder.append_dir_all(&folder_name, directory)?;

    let zstd_encoder = tar_builder.into_inner()?;
    zstd_encoder.finish()?;

    println!("Successfully created '{}'", output_path.display());

    Ok(())
}

fn decompress_archive(file: &PathBuf, destination: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !file.exists() {
        return Err(format!("File '{}' does not exist", file.display()).into());
    }

    if !file.is_file() {
        return Err(format!("'{}' is not a file", file.display()).into());
    }

    if !destination.exists() {
        return Err(format!("Destination '{}' does not exist", destination.display()).into());
    }

    if !destination.is_dir() {
        return Err(format!("Destination '{}' is not a directory", destination.display()).into());
    }

    println!("Decompressing '{}' to '{}'...", file.display(), destination.display());

    let input_file = File::open(file)?;
    let buffered_reader = BufReader::new(input_file);
    let zstd_decoder = zstd::stream::Decoder::new(buffered_reader)?;
    let mut tar_archive = tar::Archive::new(zstd_decoder);

    tar_archive.unpack(destination)?;

    println!("Successfully extracted to '{}'", destination.display());

    Ok(())
}
