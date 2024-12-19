---
layout: default
---

# Problem the Program Solves

In today's digital age, files can accumulate quickly, creating cluttered directories that are difficult to manage. Users often store documents, images, music, and videos in a single folder without organization. Over time, this can lead to frustration when searching for specific files or when maintaining a clean workspace.

The File Organizer program addresses this issue **by automatically sorting and categorizing files into appropriate folders based on their file extensions**. This eliminates the need for manual sorting, saves time, and ensures that files are easy to find and manage.

* * *

## Target Audience

The program is designed for a broad range of users, including:

**Students and Professionals:** Individuals who frequently download and manage a mix of documents, presentations, images, and other files.

**Content Creators:** Photographers, videographers, and graphic designers who deal with large numbers of media files.

**Casual Users:** Anyone who wants a cleaner and more organized file system without the hassle of manually sorting files.

**Developers and IT Administrators:** People who need a quick way to categorize and organize various file types in their directories for better project management.

## What the Program Does

The program:

1. Scans a given directory for files.

2. Categorizes files based on their extensions into predefined categories:

    * **Documents:** Includes formats like pdf, doc, docx, txt, xls, xlsx, etc.

    * **Images:** Handles formats such as jpg, png, gif, svg, raw, etc.

    * **Videos:** Supports popular video formats like mp4, avi, mkv, mov, etc.

    * **Music:** Organizes audio files like mp3, wav, aac, flac, etc.

3. Moves files to corresponding folders named after their categories (e.g., Documents, Images).

4. Moves unsupported or unrecognized files to an Others folder to ensure no file is left behind.

5. Optionally processes subdirectories recursively if the user enables the recursive mode using the <span style="color:red">-r</span> flag.

## Supported File Types

The program categorizes files into the following groups and supports these extensions:

| **Category**   | **Supported Extensions**                                                                                                    |
|-----------------|---------------------------------------------------------------------------------------------------------------------------|
| **Documents**   | pdf, doc, docx, txt, rtf, xls, xlsx, csv, ppt, pptx                                                                       |
| **Images**      | jpg, jpeg, png, gif, tiff, svg, eps, ai, webp, cr2, nef, arw, dng                                                         |
| **Videos**      | mp4, mkv, avi, mov, wmv, webm, flv, mpeg, mpg, m2ts, mts                                                                  |
| **Music**       | mp3, wav, aiff, pcm, aac, ogg, wma, flac, alac, midi, mid, m4a, amr                                                       |
| **Others**      | Files that do not match the extensions above are categorized under the **Others** folder.                                |

## How It Works

1. **User Input:** The user specifies the directory they want to organize and optionally enables recursive mode with the <span style="color:red">-r</span> flag.

1. **Directory Scanning:** The program scans the directory for files using Rust’s filesystem utilities and processes files one by one.

1. **File Categorization:** Each file is checked against predefined extension lists for categorization.

1. **Folder Creation:** If a folder for a specific category (<span style="color:#cccc00">e.g.,</span> Images) doesn’t exist, the program creates it.

1. **File Moving:** The program moves the file into the appropriate folder. If no match is found, the file is placed in an Others folder.

1. **Recursive Option:** If recursive mode is enabled, the program processes files in all subdirectories as well.

## Command-Line Interface

The program uses a simple and intuitive command-line interface powered by the clap crate. Users can interact with it as follows:

* Organize files in a directory:
 ```rust
file_organizer ./test
```

* Enable recursive processing:
```rust
file_organizer ./test -r
```

Users can also view the help message for details:
```rust
file_organizer --help
```

## Technical Highlights

* **Rust’s Performance:** The program leverages Rust’s performance and safety guarantees for efficient file operations.

* **Clap for Parsing:** The use of clap::Parser makes the command-line interface intuitive and powerful.

* **Extensibility:** The program’s modular design allows users to add more categories or file extensions with minimal effort.

* **Error Handling:** Includes basic error handling to ensure the program doesn’t crash due to invalid inputs or missing directories.


## Potential Use Cases

* **Project Management:** Developers organizing project files by type.

* **Media Sorting:** Photographers or videographers sorting raw files into folders.

* **Personal Use:** Cleaning up download folders or desktop clutter.

* **IT Maintenance:** Automating folder structure creation for shared drives.

<!-- ![Octocat](https://github.githubassets.com/images/icons/emoji/octocat.png) -->
