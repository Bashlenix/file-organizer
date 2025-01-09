use std::env;
use std::path::Path;
mod file_organizer;
use clap::Parser;
use file_organizer::FileOrganizer;

/// Command-line arguments for the file organizer.
#[derive(Parser, Debug)]
#[command(
    author = "Muhammad Otah Bashi",
    version = "1.0",
    about = "Organizes files in a directory into categorized folders",
    long_about = "This tool helps you organize files in a directory by categorizing them into folders based on file extensions. You can use it recursively to process nested directories.",
    help_template = "\
{name} {version}
{author}

{about}

USAGE:
    {usage}

ARGS:
    {all-args}

OPTIONS:
    -h, --help       Show this help message and exit
    -r, --recursive  Organize files recursively
"
)]
struct Args {
    /// The source directory to organize
    source_dir: String,

    /// Enable recursive organization
    #[arg(short, long, help = "Organize files recursively")]
    recursive: bool,
}

fn main() {
    let args = Args::parse();
    let source_dir = Path::new(&args.source_dir);

    // Get the path to the config file relative to the current working directory
    let config_file = env::current_dir()
        .expect("Failed to get current directory")
        .join("extensions.json");

    if !config_file.exists() {
        eprintln!(
            "Error: Configuration file '{}' not found.",
            config_file.display()
        );
        return;
    }

    let organizer = FileOrganizer::new(source_dir.to_path_buf(), args.recursive, &config_file);
    organizer.organize();
}
