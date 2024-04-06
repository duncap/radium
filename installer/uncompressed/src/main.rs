use dirs::home_dir;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
const OS: &str = "windows";

#[cfg(target_os = "macos")]
const OS: &str = "macos";

#[cfg(target_os = "linux")]
const OS: &str = "linux";

fn get_editor_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = home_dir() {
        if OS == "linux" || OS == "macos" {
            let vim_path = home.join(".vim");
            paths.push(vim_path);
            let sublime_path = if OS == "macos" {
                home.join("Library/Application Support/Sublime Text/Packages")
            } else {
                home.join(".config/sublime-text-3/Packages")
            };
            paths.push(sublime_path);
        } else if OS == "windows" {
            let vim_path = home.join("vimfiles");
            paths.push(vim_path);
            let sublime_path = home.join("AppData/Roaming/Sublime Text 3/Packages");
            paths.push(sublime_path);
        }
    }
    paths
}

fn install_syntax_highlighters(paths: Vec<PathBuf>) -> bool {
    let mut found = false;
    for path in paths {
        if path.exists() {
            println!("Found editor at: {:?}", path.display());
            found = true;
        }
    }
    found
}

fn get_sublime_path() -> Option<PathBuf> {
    if let Some(home) = home_dir() {
        let sublime_path = if cfg!(target_os = "macos") {
            home.join("Library/Application Support/Sublime Text/Packages")
        } else if cfg!(target_os = "windows") {
            home.join("AppData/Roaming/Sublime Text/Packages")
        } else {
            home.join(".config/sublime-text/Packages")
        };

        if sublime_path.exists() {
            return Some(sublime_path);
        }
    }
    None
}

fn install_syntax_highlighter_for_sublime(sublime_packages_path: &Path) -> io::Result<()> {
    let radium_syntax_path = sublime_packages_path.join("radium");

    if !radium_syntax_path.exists() {
        fs::create_dir(&radium_syntax_path)?;
    }

    // Adjusted to include the 'src' directory in the path
    let current_dir = std::env::current_dir()?;
    let radium_sublime_syntax_src =
        current_dir.join("src/highlighters/sublime/radium.sublime-syntax");
    let radium_package_py_src = current_dir.join("src/highlighters/sublime/radium-package.py");

    let radium_sublime_syntax_dst = radium_syntax_path.join("radium.sublime-syntax");
    let radium_package_py_dst = sublime_packages_path.join("User/radium-package.py");

    if radium_sublime_syntax_src.exists() {
        fs::copy(radium_sublime_syntax_src, radium_sublime_syntax_dst)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "radium.sublime-syntax not found",
        ));
    }

    if radium_package_py_src.exists() {
        fs::copy(radium_package_py_src, radium_package_py_dst)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "radium-package.py not found",
        ));
    }

    Ok(())
}

fn main() {
    println!("Detected OS: {}", OS);
    let editor_paths = get_editor_paths();
    if !install_syntax_highlighters(editor_paths) {
        println!("Could not find any supported text editor.");
    } else {
        // If we've found and attempted to install syntax highlighters for Vim or Sublime Text
        println!("Attempted to install syntax highlighters.");
    }

    if let Some(sublime_path) = get_sublime_path() {
        match install_syntax_highlighter_for_sublime(&sublime_path) {
            Ok(_) => println!("Successfully installed Radium syntax highlighter for Sublime Text."),
            Err(e) => eprintln!(
                "Failed to install syntax highlighter for Sublime Text: {}",
                e
            ),
        }
    }
}
