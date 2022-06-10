use dirs;
use std::path::{Path, PathBuf};

// Expands tilde in a path/filepath.
pub fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Result<PathBuf, std::io::Error> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Ok(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "dirs: Could not find home directory",
            )
        });
    }
    return dirs::home_dir()
        .map(|mut h| {
            if h == Path::new("/") {
                p.strip_prefix("~")
                    .expect("utils::expand_tilde: invlid path")
                    .to_path_buf()
            } else {
                h.push(
                    p.strip_prefix("~/")
                        .expect("utils::expand_tilde: invlid path"),
                );
                return h;
            }
        })
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "cannot expand tilde"));
}

// Returns File object after opening tile-expanded filepath
pub fn expand_open_file(filepath: &str) -> Result<std::fs::File, std::io::Error> {
    return std::fs::File::open(expand_tilde(filepath)?);
}
