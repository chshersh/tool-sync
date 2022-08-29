use flate2::read::GzDecoder;
use std::fs::File;
use std::path::{Path, PathBuf};
use tar;
use zip;

use crate::model::asset_name::mk_exe_name;

pub struct Archive<'a> {
    archive_path: &'a PathBuf,
    tmp_dir: &'a Path,
    exe_name: &'a str,
    archive_type: ArchiveType<'a>,
}

/// Archive type that specifies how to unpack asset
enum ArchiveType<'a> {
    Exe(&'a str),
    Zip(&'a str),
    TarGz(&'a str),
}

pub enum UnpackError {
    IOError(std::io::Error),
    ZipError(zip::result::ZipError),
    ExeNotFound(String),
}

impl UnpackError {
    pub fn display(&self) -> String {
        match self {
            UnpackError::IOError(e) => format!("{}", e),
            UnpackError::ZipError(e) => format!("{}", e),
            UnpackError::ExeNotFound(archive_name) => {
                format!("Can't find executable in archive: {}", archive_name)
            }
        }
    }
}

impl<'a> Archive<'a> {
    pub fn from(
        archive_path: &'a PathBuf,
        tmp_dir: &'a Path,
        exe_name: &'a str,
        asset_name: &'a str,
    ) -> Option<Archive<'a>> {
        let tar_gz_dir = asset_name.strip_suffix(".tar.gz");

        match tar_gz_dir {
            Some(tar_gz_dir) => Some(Archive {
                archive_path,
                tmp_dir,
                exe_name,
                archive_type: ArchiveType::TarGz(tar_gz_dir),
            }),
            None => {
                let zip_dir = asset_name.strip_suffix(".zip");

                match zip_dir {
                    Some(zip_dir) => Some(Archive {
                        archive_path,
                        tmp_dir,
                        exe_name,
                        archive_type: ArchiveType::Zip(zip_dir),
                    }),
                    None => {
                        let exe_file = asset_name.strip_suffix(".exe");

                        match exe_file {
                            Some(_) => Some(Archive {
                                archive_path,
                                tmp_dir,
                                exe_name,
                                archive_type: ArchiveType::Exe(asset_name),
                            }),
                            None => None,
                        }
                    }
                }
            }
        }
    }

    /// Unpack archive and return path to the executable tool
    pub fn unpack(&self) -> Result<PathBuf, UnpackError> {
        match self.archive_type {
            // already .exe file without archive (on Windows): no need to unpack
            ArchiveType::Exe(exe_file) => Ok(PathBuf::from(exe_file)),

            // unpack .tar.gz archive
            ArchiveType::TarGz(asset_name) => {
                unpack_tar(self.archive_path, self.tmp_dir).map_err(UnpackError::IOError)?;
                find_path_to_exe(self.archive_path, self.tmp_dir, self.exe_name, asset_name)
            }

            // unpack .zip archive
            ArchiveType::Zip(asset_name) => {
                unpack_zip(self.archive_path, self.tmp_dir)?;
                find_path_to_exe(self.archive_path, self.tmp_dir, self.exe_name, asset_name)
            }
        }
    }
}

fn unpack_tar(tar_path: &PathBuf, tmp_dir: &Path) -> Result<(), std::io::Error> {
    // unpack tar_path to tmp_dir
    let tar_file = File::open(tar_path)?;
    let tar_decoder = GzDecoder::new(tar_file);
    let mut archive = tar::Archive::new(tar_decoder);
    archive.unpack(tmp_dir)
}

fn unpack_zip(zip_path: &PathBuf, tmp_dir: &Path) -> Result<(), UnpackError> {
    let zip_archive_file = File::open(&zip_path).map_err(UnpackError::IOError)?;

    let mut archive = zip::ZipArchive::new(zip_archive_file).map_err(UnpackError::ZipError)?;

    archive.extract(tmp_dir).map_err(UnpackError::ZipError)
}

fn find_path_to_exe(
    archive_path: &PathBuf,
    tmp_dir: &Path,
    exe_name: &str,
    asset_name: &str,
) -> Result<PathBuf, UnpackError> {
    let path_candidates = exe_paths(exe_name, asset_name);

    // find a path
    for path in path_candidates {
        // create path to the final executable
        let mut tool_path = PathBuf::new();
        tool_path.push(tmp_dir);
        tool_path.push(path);

        // check if this path actually exists
        if tool_path.is_file() {
            return Ok(tool_path);
        }
    }

    Err(UnpackError::ExeNotFound(format!(
        "{}",
        archive_path.display()
    )))
}

// List of potential paths where an executable can be inside the archive
fn exe_paths(exe_name: &str, asset_name: &str) -> Vec<PathBuf> {
    let exe_name = mk_exe_name(exe_name);

    vec![
        [asset_name, &exe_name].iter().collect(),
        [&exe_name].iter().collect(),
        ["bin", &exe_name].iter().collect(),
    ]
}
