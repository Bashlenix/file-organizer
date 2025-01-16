use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Struct to encapsulate file organization logic.
pub struct FileOrganizer {
    source_dir: PathBuf,
    recursive: bool,
    extensions: HashMap<String, Vec<String>>,
}

impl FileOrganizer {
    /// Create a new instance of `FileOrganizer`.
    pub fn new(source_dir: PathBuf, recursive: bool, config_file: &Path) -> Self {
        let extensions =
            Self::load_extensions(config_file).expect("Failed to load extensions configuration.");
        Self {
            source_dir,
            recursive,
            extensions,
        }
    }

    /// Load extensions from a JSON configuration file.
    fn load_extensions(config_file: &Path) -> Result<HashMap<String, Vec<String>>, String> {
        let content = fs::read_to_string(config_file)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))
    }

    /// Organize files based on their extensions.
    pub fn organize(&self) {
        if !self.source_dir.is_dir() {
            eprintln!(
                "Error: '{}' is not a valid directory.",
                self.source_dir.display()
            );
            return;
        }

        if self.recursive {
            self.organize_recursively();
            println!(
                "Files organized recursively in '{}'.",
                self.source_dir.display()
            );
        } else {
            self.organize_non_recursively();
            println!("Files organized in '{}'.", self.source_dir.display());
        }
    }

    /// Organize files non-recursively.
    fn organize_non_recursively(&self) {
        let entries = fs::read_dir(&self.source_dir)
            .unwrap_or_else(|_| panic!("Cannot read directory: {}", self.source_dir.display()))
            .filter_map(|entry| entry.ok())
            .map(|e| e.path())
            .collect::<Vec<PathBuf>>();

        for entry in entries {
            if let Some(file_name) = entry.file_name() {
                if entry.is_file() {
                    self.categorize_file(&entry, &self.source_dir, file_name);
                }
            }
        }
    }

    /// Organize files recursively while retaining parent folder structure.
    fn organize_recursively(&self) {
        let entries = WalkDir::new(&self.source_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect::<Vec<_>>();

        for entry in entries {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                let parent_dir = path.parent().unwrap_or(&self.source_dir);
                self.categorize_file(path, parent_dir, file_name);
            }
        }
    }

    /// Categorize a single file into its respective folder.
    fn categorize_file(&self, entry: &Path, parent_dir: &Path, file_name: &std::ffi::OsStr) {
        let file_extension = entry
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let mut moved = false;

        for (category, ext_list) in &self.extensions {
            if ext_list.contains(&file_extension) {
                let target_dir = parent_dir.join(category);
                self.create_and_move_file(&target_dir, entry, file_name);
                moved = true;
                break;
            }
        }

        if !moved {
            let other_dir = parent_dir.join("Others");
            self.create_and_move_file(&other_dir, entry, file_name);
        }
    }

    /// Create target directory if it doesn't exist and move the file.
    fn create_and_move_file(&self, target_dir: &Path, entry: &Path, file_name: &std::ffi::OsStr) {
        if !target_dir.exists() {
            fs::create_dir_all(target_dir).unwrap();
        }
        fs::rename(entry, target_dir.join(file_name)).unwrap();
    }
}
