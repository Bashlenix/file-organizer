use walkdir::WalkDir;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// Organizes files in a directory into categorized folders.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The source directory to organize
    source_dir: String,

    /// Enable recursive organization
    #[arg(short, long)]
    recursive: bool,
}

fn organize_files(source_dir: &Path, recursive: bool) {
    let extensions = vec![
        ("Documents", vec!["pdf", "doc", "docx", "txt", "rtf", "xls", "xlsx", "csv", "ppt", "pptx"]),
        ("Images", vec!["jpg", "jpeg", "png", "gif", "tiff", "svg", "eps", "ai", "webp", "cr2", "nef", "arw", "dng"]),
        ("Videos", vec!["mp4", "mkv", "avi", "mov", "wmv", "webm", "flv", "mpeg", "mpg", "m2ts", "mts"]),
        ("Music", vec!["mp3", "wav", "aiff", "pcm", "aac", "ogg", "wma", "flac", "alac", "midi", "mid", "m4a", "amr"]),
    ];

    if !source_dir.is_dir() {
        eprintln!("Error: '{}' is not a valid directory.", source_dir.display());
        return;
    }

    let entries = if recursive {
        WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .collect::<Vec<PathBuf>>()
    } else {
        fs::read_dir(source_dir)
            .unwrap_or_else(|_| panic!("Cannot read directory: {}", source_dir.display()))
            .filter_map(|entry| entry.ok())
            .map(|e| e.path())
            .collect()
    };

    for entry in entries {
        if entry.is_file() {
            let file_extension = entry.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();

            let mut moved = false;

            for (category, ext_list) in &extensions {
                if ext_list.contains(&file_extension.as_str()) {
                    let category_dir = source_dir.join(category);
                    if !category_dir.exists() {
                        fs::create_dir_all(&category_dir).unwrap();
                    }
                    fs::rename(&entry, category_dir.join(entry.file_name().unwrap())).unwrap();
                    moved = true;
                    break;
                }
            }

            if !moved {
                let other_dir = source_dir.join("Others");
                if !other_dir.exists() {
                    fs::create_dir_all(&other_dir).unwrap();
                }
                fs::rename(&entry, other_dir.join(entry.file_name().unwrap())).unwrap();
            }
        }
    }

    println!("Files organized in '{}'.", source_dir.display());
}

fn main() {
    let args = Args::parse();
    let source_dir = Path::new(&args.source_dir);
    organize_files(source_dir, args.recursive);
}

