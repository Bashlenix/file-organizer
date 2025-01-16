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

2. Reads categorization rules from an external <span style="color:red">extensions.json</span> configuration file for maximum flexibility.

3. Categorizes files based on their extensions into predefined categories:

    * **Documents:** Includes formats like pdf, doc, docx, txt, xls, xlsx, etc.

    * **Images:** Handles formats such as jpg, png, gif, svg, raw, etc.

    * **Videos:** Supports popular video formats like mp4, avi, mkv, mov, etc.

    * **Music:** Organizes audio files like mp3, wav, aac, flac, etc.

4. Moves unsupported or unrecognized files to an Others folder.

5. Creates folders dynamically if they don’t already exist.

6. Recursive Organization: When the <span style="color:red">-r</span> or <span style="color:red">--recursive</span> flag is used, it organizes files within subdirectories while preserving the nested folder structure.


## Supported File Types

The program categorizes files based on the extensions listed in the <span style="color:red">extensions.json</span> file. Below are some default categories:

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

1. **File Categorization:** Files are categorized based on extensions read from <span style="color:red">extensions.json</span>.

1. **Folder Creation:** If a folder for a specific category (<span style="color:#cccc00">e.g.,</span> Images) doesn’t exist, the program creates it.

1. **File Moving:** The program moves the file into the appropriate folder. If no match is found, the file is placed in an Others folder.

1. **Recursive Option:** When recursive mode is enabled, the program processes all subdirectories while maintaining their structure.

## Command-Line Interface

The program uses a simple and intuitive command-line interface powered by the <span style="color:red">clap</span> crate. Users can interact with it as follows:

* Organize files in a directory:
 ```rust
file_organizer <SOURCE_DIR>
```

* Enable recursive processing:
```rust
file_organizer <SOURCE_DIR> -r
```

* View help message:
```rust
file_organizer --help
```

## Example
Given a directory structure like this:

```
test_main/
├── document.pdf
├── image.jpg
├── sub_dir/
│   ├── video.mp4
│   ├── song.mp3
```

## Example
```rust
file_organizer test_main -r
```

## Results in:

```
test_main/
├── Documents/
│   └── document.pdf
├── Images/
│   └── image.jpg
├── sub_dir/
│   ├── Videos/
│   │   └── video.mp4
│   ├── Music/
│   │   └── song.mp3
```

## Features

* **Configuration File:** Categorization rules are defined in an external <span style="color:red">extensions.json</span> file, making it easy to customize.

* **Recursive Mode:** Organizes files within nested directories while preserving their structure.

* **Dynamic Folder Creation:** Automatically creates category folders as needed.

* **Error Handling:** Ensures invalid paths or missing configuration files are reported gracefully.

* **Cross-Platform:** Runs efficiently on  macOS, and Linux.


## Potential Use Cases

* **Project Management:** Developers organizing project files by type.

* **Media Sorting:** Photographers or videographers sorting raw files into folders.

* **Personal Use:** Cleaning up download folders or desktop clutter.

* **IT Maintenance:** Automating folder structure creation for shared drives.

<!-- ![Octocat](https://github.githubassets.com/images/icons/emoji/octocat.png) -->
