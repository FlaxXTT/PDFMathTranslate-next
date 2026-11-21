use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

use crate::network::http_url;

const READ_ERROR: &str = "The saved service configuration is invalid. Check it and try again.";
const WRITE_ERROR: &str = "Could not save the service configuration.";
const MISSING_PROFILE: &str = "Select a saved service first.";

#[derive(Deserialize, Serialize)]
pub(crate) struct Profile {
    pub(crate) base_url: String,
    pub(crate) model: String,
}

type Profiles = BTreeMap<String, Profile>;

pub(crate) fn validate_endpoint(url: &str, model: &str) -> Result<(String, String), String> {
    let model = model.trim();
    if model.is_empty() || model.chars().count() > 200 {
        return Err("Enter a model name.".into());
    }
    let url = url.trim();
    let parsed =
        http_url(url).filter(|parsed| parsed.query().is_none() && parsed.fragment().is_none());
    if parsed.is_none() || url.chars().count() > 2048 {
        return Err("Enter an HTTP or HTTPS API base URL without credentials, a query, or a fragment.".into());
    }
    let url = url.trim_end_matches('/');
    let url = url.strip_suffix("/chat/completions").unwrap_or(url);
    Ok((url.trim_end_matches('/').into(), model.into()))
}

pub(crate) fn validate(
    name: &str,
    url: &str,
    model: &str,
) -> Result<(String, String, String), String> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 80 {
        return Err("Enter a service name of up to 80 characters.".into());
    }
    let (url, model) = validate_endpoint(url, model)?;
    Ok((name.into(), url, model))
}

pub(crate) fn load(path: &Path) -> Result<Profiles, String> {
    match File::open(path) {
        Ok(file) => {
            serde_json::from_reader(io::BufReader::new(file)).map_err(|_| READ_ERROR.into())
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(BTreeMap::new()),
        Err(_) => Err("Could not read the service configuration.".into()),
    }
}

pub(crate) fn save(path: &Path, name: &str, url: &str, model: &str) -> Result<String, String> {
    let (name, base_url, model) = validate(name, url, model)?;
    edit_profiles(path, |profiles| {
        profiles.insert(name.clone(), Profile { base_url, model });
        Ok(())
    })?;
    Ok(name)
}

pub(crate) fn delete(path: &Path, name: &str) -> Result<(), String> {
    edit_profiles(path, |profiles| {
        profiles.remove(name).ok_or(MISSING_PROFILE)?;
        Ok(())
    })
}

fn edit_profiles(
    path: &Path,
    edit: impl FnOnce(&mut Profiles) -> Result<(), &str>,
) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(|_| WRITE_ERROR)?;
    // Keep a stable lock inode while the JSON file itself is replaced atomically.
    let lock_path = path.with_extension("lock");
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(lock_path)
        .map_err(|_| WRITE_ERROR)?;
    lock.lock().map_err(|_| WRITE_ERROR)?;
    let mut profiles = load(path)?;
    edit(&mut profiles)?;
    let mut output = NamedTempFile::new_in(parent).map_err(|_| WRITE_ERROR)?;
    serde_json::to_writer_pretty(&mut output, &profiles).map_err(|_| WRITE_ERROR)?;
    output.write_all(b"\n").map_err(|_| WRITE_ERROR)?;
    output.as_file().sync_all().map_err(|_| WRITE_ERROR)?;
    output.persist(path).map_err(|_| WRITE_ERROR)?;
    Ok(())
}
