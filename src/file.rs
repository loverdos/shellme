// SPDX-License-Identifier: MPL-2.0
use crate::UnwrapToString;
use is_executable::IsExecutable;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct File {
    path: Rc<PathBuf>,
}

impl From<PathBuf> for File {
    fn from(value: PathBuf) -> Self {
        Self {
            path: Rc::new(value),
        }
    }
}

impl From<&Path> for File {
    fn from(value: &Path) -> Self {
        File::from(value.to_path_buf())
    }
}

impl File {
    fn new<Name: AsRef<str>>(name: Name) -> Self {
        Self::from(PathBuf::from(name.as_ref()))
    }

    fn filename(&self) -> Self {
        Self::new(self.path.file_name().unwrap_to_string())
    }

    fn ext(&self) -> String {
        self.path.extension().unwrap_to_string()
    }

    fn canonical(&self) -> Self {
        Self::from(self.path.canonicalize().unwrap())
    }

    fn as_path_buf(&self) -> Rc<PathBuf> {
        self.path.clone()
    }

    fn as_path(&self) -> &Path {
        self.path.as_path()
    }

    fn to_string(&self) -> String {
        String::from(self.path.to_str().unwrap())
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn is_file(&self) -> bool {
        self.path.is_file()
    }

    fn is_folder(&self) -> bool {
        self.path.is_dir()
    }

    fn is_symlink(&self) -> bool {
        self.path.is_symlink()
    }

    fn is_absolute(&self) -> bool {
        self.path.is_absolute()
    }

    fn is_relative(&self) -> bool {
        self.path.is_relative()
    }

    fn is_exec(&self) -> bool {
        self.as_path().is_executable()
    }

    fn parent(&self) -> Option<File> {
        self.path.as_path().parent().map(File::from)
    }
}
