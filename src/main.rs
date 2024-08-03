use clap::Parser;
use image::{ImageFormat, ImageReader};
use std::{fs, io::Cursor, path::Path};

#[derive(Parser, Debug)]
struct Args {
    path_in: std::path::PathBuf,
    path_out: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    if !validate_format(&args.path_in) {
        println!("Invalid format for input file {:?} \u{274C}", args.path_in);
        return;
    }

    if !validate_format(&args.path_out) {
        println!(
            "Invalid format for output file {:?} \u{274C}",
            args.path_out
        );
        return;
    }

    println!("Reading file: {:?}", args.path_in);
    let img = ImageReader::open(&args.path_in)
        .expect("Already checked for valid file")
        .decode()
        .unwrap();

    let file_extention =
        get_file_extention(&args.path_out).expect("Already checked for valid file");

    println!("   -> Converting into .{}", file_extention);
    let mut bytes = Vec::new();
    if let Err(ex) = img.write_to(
        &mut Cursor::new(&mut bytes),
        ImageFormat::from_extension(file_extention).expect("Already checked for valid file"),
    ) {
        println!("{}", ex);
        return;
    }

    if let Err(ex) = fs::write(args.path_out, bytes) {
        println!("{}", ex);
        return;
    }
    println!("   -> Conversion succesful \u{2705}");
}

fn get_file_extention<P: AsRef<Path>>(path: P) -> Option<String> {
    let file_name = path.as_ref().file_name()?.to_str()?;

    let index = file_name.find(".")? + 1;
    let len = file_name.len() - index;
    let ending = file_name.get(index..index + len)?;

    Some(ending.into())
}

fn validate_format<P: AsRef<Path>>(path: P) -> bool {
    let extention = get_file_extention(path);
    if let Some(ext) = extention {
        if let Some(_) = ImageFormat::from_extension(ext) {
            return true;
        }
    }

    false
}
