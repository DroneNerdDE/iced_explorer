use chrono::{DateTime, Local};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

// 1. DATA STRUCTURES (STATE)
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub size: String,
    pub modified: String,
    pub permissions: String,
}

// Helper function
pub fn read_directory(path: &Path, show_hidden: bool) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {

            if !show_hidden && is_hidden(&entry) {
                continue;
            }

            let metadata = entry.metadata().ok();
            let is_dir = entry.metadata().as_ref().map(|m| m.is_dir()).unwrap_or(false);

            // Format Size
            let size = if is_dir {
                "--".to_string()
            } else {
                metadata.as_ref().map(|m| format_size(m.len())).unwrap_or_else(|| "Unknown".to_string())
            };

            // Format Modified
            let modified = metadata.as_ref()
                .and_then(|m| m.modified().ok())
                .map(|time| {
                    let datetime: DateTime<Local> = time.into();
                    datetime.format("%Y-%m-%d %H:%M").to_string()
                })
                .unwrap_or_else(|| "Unknown".to_string());

            let permissions = metadata.as_ref()
                .map(|m| format_permissions(&m.permissions()))
                .unwrap_or_else(|| "Unknown".to_string());
            
            entries.push(FileEntry {
                path: entry.path(),
                name: entry.file_name().to_string_lossy().to_string(),
                is_dir,
                size,
                modified,
                permissions
            });
        }
    }

    entries.sort_by_key(|e| (!e.is_dir, e.name.clone()));
    entries
}

fn format_size(bytes: u64) -> String {
    let kb = bytes as f64 / 1024.0;
    if kb < 1.0 { return format!("{} B", bytes); }
    let mb = kb / 1024.0;
    if mb < 1.0 { return format!("{:.1} KB", bytes); }
    let gb = mb / 1024.0;
    if gb < 1.0 { return format!("{:.1} MB", bytes); }
    format!("{:.2} GB", gb)
}

fn format_permissions(perms: &std::fs::Permissions) -> String {
    #[cfg(unix)]
    {
        let mode = perms.mode();
        let rwx = |m| match m {
            7 => "rwx",
            6 => "rw-",
            5 => "r-x",
            4 => "r--",
            3 => "-wx",
            2 => "-w-",
            1 => "--x",
            _ => "---",
        };
        format!("{}{}{}", rwx((mode >> 6) & 7), rwx((mode >> 3) & 7), rwx(mode & 7))
    }
    #[cfg(not(unix))]
    {
        if perms.readonly() { "Read-only".to_string() } else {"Writable".to_string() }
    }
}

#[cfg(unix)]
fn is_hidden(entry: &std::fs::DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}

#[cfg(windows)]
fn is_hidden(entry: &std::fs::DirEntry) -> bool {
    use std::os::windows::fs::MetadataExt;

    if let Ok(metadata) = entry.metadata() {
        // Windows file attributes are a bitmask
        // 0x2 is HEX value for FILE_ATTRIBUTE_HIDDEN
        (metadata.file_attributes & 0x2) > 0
    } else {
        false
    }
}