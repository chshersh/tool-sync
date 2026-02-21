use deko::AnyDecoder;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::model::asset_name::mk_exe_name;

pub struct Archive<'a> {
    archive_path: &'a PathBuf,
    tmp_dir: &'a Path,
    exe_name: &'a str,
    tag: &'a str,
    archive_type: ArchiveType<'a>,
}

/// Archive type that specifies how to unpack asset
enum ArchiveType<'a> {
    Exe(&'a str),
    Zip(&'a str),
    TarBall(&'a str),
}

pub enum UnpackError {
    IOError(std::io::Error),
    ZipError(zip::result::ZipError),
    ExeNotFound(String, String),
}

impl Display for UnpackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnpackError::IOError(e) => write!(f, "{e}"),
            UnpackError::ZipError(e) => write!(f, "{e}"),
            UnpackError::ExeNotFound(exe_name, archive_name) => {
                write!(f, "Can't find {exe_name} in archive: {archive_name}")
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
        tag: &'a str,
    ) -> Option<Archive<'a>> {
        match asset_name.rsplit_once('.') {
            // un-compressed binary
            None
            | Some((_, "exe"))
            | Some((_, "AppImage"))
            | Some((_, "py"))
            | Some((_, "pyz"))
            | Some((_, "ttf"))
            | Some((_, "otc"))
            | Some((_, "so")) => Archive {
                archive_path,
                tmp_dir,
                exe_name,
                tag,
                archive_type: ArchiveType::Exe(asset_name),
            }
            .into(),
            // un-compressed binary with some arch-suffix
            Some((_, ext)) if ext.len() > 10 => Archive {
                archive_path,
                tmp_dir,
                exe_name,
                tag,
                archive_type: ArchiveType::Exe(asset_name),
            }
            .into(),
            Some((prefix, "zip")) => Archive {
                archive_path,
                tmp_dir,
                exe_name,
                tag,
                archive_type: ArchiveType::Zip(prefix),
            }
            .into(),
            Some((prefix, "xz" | "txz" | "gz" | "tgz" | "zst")) => Archive {
                archive_path,
                tmp_dir,
                exe_name,
                tag,
                archive_type: ArchiveType::TarBall(prefix.trim_end_matches(".tar")),
            }
            .into(),
            _ => {
                dbg!("unsupported asset format {asset_name}");
                None
            }
        }
    }

    /// Unpack archive and return path to the executable tool
    pub fn unpack(&self) -> Result<PathBuf, UnpackError> {
        match self.archive_type {
            // already executable file without archive: no need to unpack
            ArchiveType::Exe(exe_file) => Ok(self.tmp_dir.join(exe_file)),

            // unpack .tar ball
            ArchiveType::TarBall(asset_name) => {
                unpack_tar(self.archive_path, self.tmp_dir).map_err(UnpackError::IOError)?;
                find_path_to_exe(
                    self.archive_path,
                    self.tmp_dir,
                    self.exe_name,
                    asset_name,
                    self.tag,
                )
            }

            // unpack .zip archive
            ArchiveType::Zip(asset_name) => {
                unpack_zip(self.archive_path, self.tmp_dir)?;
                find_path_to_exe(
                    self.archive_path,
                    self.tmp_dir,
                    self.exe_name,
                    asset_name,
                    self.tag,
                )
            }
        }
    }
}

fn unpack_tar(tar_path: &PathBuf, tmp_dir: &Path) -> Result<(), std::io::Error> {
    // unpack tar_path to tmp_dir
    let tar_file = File::open(tar_path)?;
    let tar_decoder = AnyDecoder::new(BufReader::new(tar_file));
    let mut archive = tar::Archive::new(tar_decoder);
    archive.unpack(tmp_dir)
}

fn unpack_zip(zip_path: &PathBuf, tmp_dir: &Path) -> Result<(), UnpackError> {
    let zip_archive_file = File::open(zip_path).map_err(UnpackError::IOError)?;

    let mut archive = zip::ZipArchive::new(zip_archive_file).map_err(UnpackError::ZipError)?;

    archive.extract(tmp_dir).map_err(UnpackError::ZipError)
}

fn find_path_to_exe(
    archive_path: &Path,
    tmp_dir: &Path,
    exe_name: &str,
    asset_name: &str,
    tag: &str,
) -> Result<PathBuf, UnpackError> {
    let path_candidates = exe_paths(exe_name, asset_name, tag);

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

    Err(UnpackError::ExeNotFound(
        exe_name.to_string(),
        format!("{}", archive_path.display()),
    ))
}

// List of potential paths where an executable can be inside the archive
fn exe_paths(exe_name: &str, asset_name: &str, tag: &str) -> Vec<PathBuf> {
    let exe_name = mk_exe_name(exe_name);
    let numeric_tag = tag.trim_start_matches('v');

    vec![
        exe_name.clone().into(),
        asset_name.into(),
        asset_name.trim_end_matches(".tar.xz").into(),
        asset_name.trim_end_matches(".tar.gz").into(),
        asset_name.trim_end_matches(".tgz").into(),
        asset_name.trim_end_matches(".zip").into(),
        [asset_name, &exe_name].iter().collect(),
        ["tmp", asset_name, &exe_name].iter().collect(),
        [&exe_name, &exe_name].iter().collect(),
        ["bin", &exe_name].iter().collect(),
        [asset_name, "bin", &exe_name].iter().collect(),
        [&format!("{exe_name}-{tag}"), &exe_name].iter().collect(),
        [
            &format!("{exe_name}-x86_64-unknown-linux-gnu"),
            "bin",
            &exe_name,
        ]
        .iter()
        .collect(),
        [&format!("{exe_name}-{tag}-x86_64"), "usr/bin", &exe_name]
            .iter()
            .collect(),
        [
            &format!("{exe_name}-{numeric_tag}-x86_64"),
            "usr/bin",
            &exe_name,
        ]
        .iter()
        .collect(),
        [&format!("{exe_name}-{numeric_tag}"), &exe_name]
            .iter()
            .collect(),
    ]
}
