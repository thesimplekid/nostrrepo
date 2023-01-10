mod errors;
use portan::types::PatchInfo;
use std::{
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use url::Url;

use errors::Error;

/// Clones a git repository
/// ```rust
/// use portan_git::clone_repository;
/// use std::path::Path;
/// use url::Url;
/// use std::str::FromStr;
///
/// let path = Path::new("/home/thesimplekid/portan-testing/");
///
/// clone_repository(&Url::from_str("https://github.com/thesimplekid/bitcoin_palindrome_bot").unwrap(), &path).unwrap();
/// ```
pub fn clone_repository(git_url: &Url, destination_path: &PathBuf) -> Result<Output, Error> {
    // Creates directory if it doesn't exist
    fs::create_dir_all(destination_path)?;

    Ok(Command::new("git")
        .current_dir(destination_path)
        .arg("clone")
        .arg(git_url.as_ref())
        .output()?)
}

/// Generate patch
/// ```rust
/// use portan_git::generate_patch;
/// use std::path::Path;
/// use url::Url;
/// use std::str::FromStr;
/// let path = Path::new("/home/thesimplekid/portan-testing/bitcoin_palindrome_bot");
///
/// let out = generate_patch(path, 1).unwrap();
///
/// ```
pub fn generate_patch(local_repo: &PathBuf, num_commits: usize) -> Result<String, Error> {
    let output = Command::new("git")
        .current_dir(local_repo)
        .arg("format-patch")
        .arg("--stdout")
        .arg(format!("HEAD~{}", num_commits))
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn get_log(local_repo: &PathBuf) -> Result<Vec<String>, Error> {
    let output = Command::new("git")
        .current_dir(local_repo)
        .arg("log")
        .arg("-n 5")
        .arg("--oneline")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .to_string()
        .split("\n")
        .map(|x| x.to_string())
        .collect())
}

pub fn create_directory(path: &PathBuf) -> Result<(), Error> {
    match fs::create_dir(path) {
        Ok(_) => (),
        Err(_) => (),
    }

    Ok(())
}

pub fn save_patch(path: &PathBuf, patch: &PatchInfo) -> Result<(), Error> {
    let mut file = File::create(
        Path::new(path).join(format!("{}.patch", patch.title.clone().replace(" ", "-"))),
    )?;
    let data = patch.patch.clone();
    file.write_all(data.as_bytes())?;
    Ok(())
}
