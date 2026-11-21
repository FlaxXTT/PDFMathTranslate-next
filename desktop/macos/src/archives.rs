use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

const OUTPUT_KINDS: [&str; 3] = ["mono", "dual", "glossary"];
const ARCHIVE_NAMES: [(&str, &str); 4] = [
    ("all", "all_translations.zip"),
    ("mono", "all_mono_translations.zip"),
    ("dual", "all_dual_translations.zip"),
    ("glossary", "all_glossaries.zip"),
];

#[derive(Deserialize)]
pub(crate) struct ArchiveDocument {
    pub(crate) name: String,
    pub(crate) files: [Option<PathBuf>; 3],
}

struct Member {
    kind: &'static str,
    source: PathBuf,
    name: String,
}

pub(crate) fn write(
    directory: &Path,
    entries: Vec<ArchiveDocument>,
) -> Result<BTreeMap<String, PathBuf>, String> {
    let parent = directory
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if directory.file_name().is_none() {
        return Err("Invalid archive output directory".to_owned());
    }
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let pending = tempfile::Builder::new()
        .prefix(".zip-")
        .tempdir_in(parent)
        .map_err(|error| error.to_string())?;

    write_archives(pending.path(), directory, entries).map_err(|error| error.to_string())
}

fn write_archives(
    pending: &Path,
    directory: &Path,
    entries: Vec<ArchiveDocument>,
) -> Result<BTreeMap<String, PathBuf>, Box<dyn Error>> {
    let mut members = Vec::new();
    let mut used_stems = HashSet::new();
    for document in entries {
        let stem = Path::new(&document.name)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or("Invalid document filename for archive")?;
        // Keep entries at the archive root on both macOS and Windows.
        let base = stem.replace('\\', "_");
        let mut stem = base.clone();
        let mut suffix = 2;
        while !used_stems.insert(stem.clone()) {
            stem = format!("{base} ({suffix})");
            suffix += 1;
        }
        for (kind, source) in OUTPUT_KINDS.into_iter().zip(document.files) {
            if let Some(source) = source {
                let mut name = format!("{stem}.{kind}");
                if let Some(extension) = source
                    .extension()
                    .and_then(|suffix| suffix.to_str())
                    .filter(|suffix| !suffix.is_empty())
                {
                    name.push('.');
                    name.push_str(extension);
                }
                members.push(Member { kind, source, name });
            }
        }
    }

    let mut paths = BTreeMap::new();
    for (kind, filename) in ARCHIVE_NAMES {
        let mut files = members
            .iter()
            .filter(|member| kind == "all" || member.kind == kind)
            .peekable();
        if files.peek().is_none() {
            continue;
        }
        let mut archive = ZipWriter::new(File::create(pending.join(filename))?);
        for member in files {
            let mut input = File::open(&member.source)?;
            let options = SimpleFileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .large_file(input.metadata()?.len() >= u64::from(u32::MAX));
            archive.start_file(&member.name, options)?;
            io::copy(&mut input, &mut archive)?;
        }
        archive.finish()?;
        paths.insert(kind.to_owned(), directory.join(filename));
    }

    if directory.try_exists()? {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Archive output already exists",
        )
        .into());
    }
    fs::rename(pending, directory)?;
    Ok(paths)
}
