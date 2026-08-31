use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, ensure};

pub(crate) fn secure_output_path(path: &Path, label: &str) -> Result<PathBuf> {
    ensure!(path.file_name().is_some(), "{label} path has no file name");
    let absolute = lexical_absolute(path)?;
    let parent = absolute
        .parent()
        .ok_or_else(|| anyhow::anyhow!("{label} path has no parent"))?;
    secure_directory(parent, label)?;
    Ok(absolute)
}

/// Resolve an existing path without creating anything and reject every
/// symbolic-link component. Callers must still open the final file with a
/// no-follow, descriptor-based API to close the check/open race.
pub(crate) fn secure_existing_path(path: &Path, label: &str) -> Result<PathBuf> {
    let absolute = lexical_absolute(path)?;
    let mut cursor = PathBuf::new();
    for (index, component) in absolute.components().enumerate() {
        cursor.push(component.as_os_str());
        if matches!(component, Component::Prefix(_) | Component::RootDir) {
            continue;
        }
        let metadata = std::fs::symlink_metadata(&cursor)
            .with_context(|| format!("cannot inspect {label} path {}", cursor.display()))?;
        ensure!(
            !metadata.file_type().is_symlink(),
            "{label} path traverses a symbolic link: {}",
            cursor.display()
        );
        if index + 1 < absolute.components().count() {
            ensure!(
                metadata.is_dir(),
                "{label} path component is not a directory: {}",
                cursor.display()
            );
        }
    }
    Ok(absolute)
}

pub(crate) fn secure_directory(path: &Path, label: &str) -> Result<PathBuf> {
    let absolute = lexical_absolute(path)?;
    let mut cursor = PathBuf::new();
    for component in absolute.components() {
        cursor.push(component.as_os_str());
        if matches!(component, Component::Prefix(_) | Component::RootDir) {
            continue;
        }
        match std::fs::symlink_metadata(&cursor) {
            Ok(metadata) => {
                ensure!(
                    !metadata.file_type().is_symlink(),
                    "{label} path traverses a symbolic link: {}",
                    cursor.display()
                );
                ensure!(
                    metadata.is_dir(),
                    "{label} path component is not a directory: {}",
                    cursor.display()
                );
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir(&cursor).with_context(|| {
                    format!("cannot create {label} directory {}", cursor.display())
                })?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(&cursor, std::fs::Permissions::from_mode(0o700))
                        .with_context(|| {
                            format!("cannot secure {label} directory {}", cursor.display())
                        })?;
                }
            }
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("cannot inspect {label} path {}", cursor.display()));
            }
        }
    }
    Ok(absolute)
}

fn lexical_absolute(path: &Path) -> Result<PathBuf> {
    let source = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .context("cannot resolve current directory")?
            .join(path)
    };
    let mut normalized = PathBuf::new();
    for component in source.components() {
        match component {
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ensure!(normalized.pop(), "path escapes the filesystem root");
            }
        }
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn secure_directory_rejects_symlink_components() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir should work");
        let outside = tempfile::tempdir().expect("outside tempdir should work");
        let link = directory.path().join("link");
        symlink(outside.path(), &link).expect("symlink should be created");
        let error = secure_directory(&link.join("child"), "fixture")
            .expect_err("symlink component should fail");
        assert!(error.to_string().contains("symbolic link"));
        assert!(!outside.path().join("child").exists());
    }

    #[cfg(unix)]
    #[test]
    fn secure_existing_path_rejects_final_symlink() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir should work");
        let target = directory.path().join("target");
        std::fs::write(&target, "target").expect("target should be written");
        let link = directory.path().join("link");
        symlink(&target, &link).expect("symlink should be created");
        let error = secure_existing_path(&link, "fixture").expect_err("final symlink should fail");
        assert!(error.to_string().contains("symbolic link"));
    }
}
