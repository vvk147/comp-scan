use anyhow::Result;
use bytesize::ByteSize;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct DiskReport {
    pub large_files: Vec<FileEntry>,
    pub temp_files: Vec<FileEntry>,
    pub cache_dirs: Vec<DirEntry>,
    pub total_temp_size: u64,
    pub total_cache_size: u64,
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub size: u64,
    pub file_count: usize,
}

pub fn quick_scan() -> Result<DiskReport> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let temp_dirs = get_temp_directories();
    let cache_dirs = get_cache_directories(&home);

    let mut temp_files = Vec::new();
    let mut total_temp_size = 0u64;
    for dir in &temp_dirs {
        if dir.exists() {
            if let Ok(size) = dir_size(dir, 2) {
                total_temp_size += size;
                temp_files.push(FileEntry {
                    path: dir.clone(),
                    size,
                });
            }
        }
    }

    let mut cache_entries = Vec::new();
    let mut total_cache_size = 0u64;
    for dir in &cache_dirs {
        if dir.exists() {
            let (size, count) = dir_size_and_count(dir, 2);
            total_cache_size += size;
            cache_entries.push(DirEntry {
                path: dir.clone(),
                size,
                file_count: count,
            });
        }
    }

    cache_entries.sort_by(|a, b| b.size.cmp(&a.size));

    Ok(DiskReport {
        large_files: Vec::new(),
        temp_files,
        cache_dirs: cache_entries,
        total_temp_size,
        total_cache_size,
    })
}

pub fn deep_scan() -> Result<DiskReport> {
    let mut report = quick_scan()?;
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    let mut large_files = Vec::new();
    let threshold = 100 * 1024 * 1024; // 100 MB

    let walker = WalkDir::new(&home)
        .max_depth(5)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !name.starts_with('.') || name == ".cache" || name == ".local"
        });

    for entry in walker.flatten() {
        if entry.file_type().is_file() {
            if let Ok(meta) = entry.metadata() {
                if meta.len() > threshold {
                    large_files.push(FileEntry {
                        path: entry.path().to_path_buf(),
                        size: meta.len(),
                    });
                }
            }
        }
    }

    large_files.sort_by(|a, b| b.size.cmp(&a.size));
    large_files.truncate(20);
    report.large_files = large_files;
    Ok(report)
}

pub fn print_summary(report: &DiskReport) {
    println!("  Temp files:  {} reclaimable", ByteSize(report.total_temp_size));
    println!("  Cache dirs:  {} reclaimable", ByteSize(report.total_cache_size));

    if !report.cache_dirs.is_empty() {
        println!("\n  Largest cache directories:");
        for (i, dir) in report.cache_dirs.iter().take(5).enumerate() {
            println!(
                "    {}. {:>10}  {} ({} files)",
                i + 1,
                ByteSize(dir.size),
                dir.path.display(),
                dir.file_count
            );
        }
    }

    if !report.large_files.is_empty() {
        println!("\n  Large files (>100MB):");
        for (i, f) in report.large_files.iter().take(10).enumerate() {
            println!(
                "    {}. {:>10}  {}",
                i + 1,
                ByteSize(f.size),
                f.path.display()
            );
        }
    }
}

fn get_temp_directories() -> Vec<PathBuf> {
    let mut dirs = vec![std::env::temp_dir()];

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            dirs.push(home.join("Library/Caches"));
        }
    }

    #[cfg(target_os = "linux")]
    {
        dirs.push(PathBuf::from("/tmp"));
        dirs.push(PathBuf::from("/var/tmp"));
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(temp) = std::env::var("TEMP") {
            dirs.push(PathBuf::from(temp));
        }
    }

    dirs
}

fn get_cache_directories(home: &PathBuf) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    let candidates = [
        ".cache",
        ".npm",
        ".cargo/registry",
        ".rustup/toolchains",
        "node_modules",
        ".gradle/caches",
        ".m2/repository",
        ".pip/cache",
    ];

    for c in &candidates {
        let p = home.join(c);
        if p.exists() {
            dirs.push(p);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mac_caches = [
            "Library/Caches",
            "Library/Logs",
            "Library/Developer/Xcode/DerivedData",
        ];
        for c in &mac_caches {
            let p = home.join(c);
            if p.exists() {
                dirs.push(p);
            }
        }
    }

    dirs
}

fn dir_size(path: &PathBuf, max_depth: usize) -> Result<u64> {
    let mut total = 0u64;
    for entry in WalkDir::new(path).max_depth(max_depth).into_iter().flatten() {
        if entry.file_type().is_file() {
            if let Ok(meta) = entry.metadata() {
                total += meta.len();
            }
        }
    }
    Ok(total)
}

fn dir_size_and_count(path: &PathBuf, max_depth: usize) -> (u64, usize) {
    let mut total = 0u64;
    let mut count = 0usize;
    for entry in WalkDir::new(path).max_depth(max_depth).into_iter().flatten() {
        if entry.file_type().is_file() {
            if let Ok(meta) = entry.metadata() {
                total += meta.len();
                count += 1;
            }
        }
    }
    (total, count)
}
