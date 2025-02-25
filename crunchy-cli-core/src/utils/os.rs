use log::debug;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, io};
use tempfile::{Builder, NamedTempFile};

pub fn has_ffmpeg() -> bool {
    if let Err(e) = Command::new("ffmpeg").stderr(Stdio::null()).spawn() {
        if ErrorKind::NotFound != e.kind() {
            debug!(
                "unknown error occurred while checking if ffmpeg exists: {}",
                e.kind()
            )
        }
        false
    } else {
        true
    }
}

/// Any tempfile should be created with this function. The prefix and directory of every file
/// created with this method stays the same which is helpful to query all existing tempfiles and
/// e.g. remove them in a case of ctrl-c. Having one function also good to prevent mistakes like
/// setting the wrong prefix if done manually.
pub fn tempfile<S: AsRef<str>>(suffix: S) -> io::Result<NamedTempFile> {
    let tempfile = Builder::default()
        .prefix(".crunchy-cli_")
        .suffix(suffix.as_ref())
        .tempfile_in(&env::temp_dir())?;
    debug!(
        "Created temporary file: {}",
        tempfile.path().to_string_lossy()
    );
    Ok(tempfile)
}

/// Check if the given path exists and rename it until the new (renamed) file does not exist.
pub fn free_file(mut path: PathBuf) -> PathBuf {
    let mut i = 0;
    while path.exists() {
        i += 1;

        let ext = path.extension().unwrap().to_string_lossy();
        let mut filename = path.file_stem().unwrap().to_str().unwrap();

        if filename.ends_with(&format!(" ({})", i - 1)) {
            filename = filename.strip_suffix(&format!(" ({})", i - 1)).unwrap();
        }

        path.set_file_name(format!("{} ({}).{}", filename, i, ext))
    }
    sanitize_file(path)
}

/// Sanitizes the given path to not contain any invalid file character.
pub fn sanitize_file(path: PathBuf) -> PathBuf {
    path.with_file_name(sanitize_filename::sanitize(
        path.file_name().unwrap().to_string_lossy(),
    ))
}
