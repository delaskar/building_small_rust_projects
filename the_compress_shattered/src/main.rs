// std rust crates
use std::io;
use std::fs::{self, File};
use std::path::Path;

// flat2 crate
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;


fn check_file_name() -> String {
    /* This code checks if the user-supplied filename 
    exists as a file in the current directory.*/

    // Get current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("Failed to get the current directory: {}", e)
    };

    // User input
    println!("Write the name file:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed_input = input.trim();

    // Check if the file exists in the current directory
    for entry in fs::read_dir(current_dir).unwrap() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.file_name().unwrap() == trimmed_input {
                return trimmed_input.to_string();
            }
        }
    }
    panic!("File not found in current directory");
}

fn file_compress(file_enter: &String, file_out: &String) -> std::io::Result<()>  {
    /* This function compress file by user */
    let mut entry = File::open(file_enter)?;
    let mut out = File::create(file_out)?;

    let compression = Compression::default();
    let mut compresor = ZlibEncoder::new(&mut out, compression);
    std::io::copy(&mut entry, &mut compresor)?;
    compresor.finish()?;
    Ok(()) 
}

fn decompress_file(file_enter: &String, file_out: &String) -> std::io::Result<()> {
    let mut entry = File::open(file_enter)?;
    let mut out = File::create(file_out)?;
    let mut decompression = ZlibDecoder::new(&mut entry);
    std::io::copy(&mut decompression, &mut out)?;
    Ok(())
}

fn add_suffix(input: &str) -> String {
    /* This function adds a suffix to the original file name */

    let path = Path::new(input);
    let stem = path.file_stem().unwrap().to_str().unwrap();  // Get the file name without extension
    let suffix = "_file_compressed";
    let result = stem.to_owned() + suffix;
    result
}

fn user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to compress shattered!");
    println!("");

    // Read name user file
    let user_name_file: String = check_file_name();

    // Create new name file
    let new_name: String = add_suffix(&user_name_file);

    // Compress user file
    match file_compress(&user_name_file, &new_name) {
        Ok(()) => println!("File compreseed succesfully"),
        Err(e) => println!("Error while compressed file: {}", e)
    }

    let ask_user = user_input("Do you want to unzip the file?. Write (y) or (n)");
    let decompress_name_file = check_file_name();
    let out_file_name: String = String::from("Decompressed_File");

    if let "y" = ask_user.as_str() {
        match decompress_file(&decompress_name_file, &out_file_name) {
            Ok(()) => println!("File compreseed succesfully"),
            Err(e) => println!("Error while compressed file: {}", e)
        }
    }
}
