// SPDX-License-Identifier: MPL-2.0
#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![doc = include_str!("../README.md")]

mod file;
mod strings;
pub use strings::*;

use is_executable::IsExecutable;
use std::ffi::{OsStr, OsString};
use std::io::BufRead;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::slice::Iter;
use std::vec::IntoIter;
use which;

pub type Strs<'a> = &'a [&'a str];
pub const NO_STRS: Strs<'static> = &[];

pub trait FileAPI {
    fn filename(&self) -> String;

    fn ext(&self) -> String;

    fn canonical(&self) -> String;

    fn to_path_buf(&self) -> PathBuf;

    fn exists(&self) -> bool;

    fn is_file(&self) -> bool;

    fn is_folder(&self) -> bool;

    fn is_symlink(&self) -> bool;

    fn is_absolute(&self) -> bool;

    fn is_relative(&self) -> bool;

    fn is_exec(&self) -> bool;

    fn which(&self) -> Option<String>;

    fn parent(&self) -> Option<String>;

    fn read_lines(&self) -> Vec<String>;
}

#[inline]
fn pb<Name: AsRef<str>>(name: Name) -> PathBuf {
    PathBuf::from(name.as_ref())
}

impl<Name: AsRef<str>> FileAPI for Name {
    fn filename(&self) -> String {
        pb(self).file_name().unwrap_to_string()
    }

    fn ext(&self) -> String {
        pb(self).extension().unwrap_to_string()
    }

    fn canonical(&self) -> String {
        pb(self).canonicalize().unwrap().unwrap_to_string()
    }

    fn to_path_buf(&self) -> PathBuf {
        pb(self)
    }

    fn exists(&self) -> bool {
        pb(self).exists()
    }

    fn is_file(&self) -> bool {
        pb(self).is_file()
    }

    fn is_folder(&self) -> bool {
        pb(self).is_dir()
    }

    fn is_symlink(&self) -> bool {
        pb(self).is_symlink()
    }

    fn is_absolute(&self) -> bool {
        pb(self).is_absolute()
    }

    fn is_relative(&self) -> bool {
        pb(self).is_relative()
    }

    fn is_exec(&self) -> bool {
        pb(self).is_executable()
    }

    fn which(&self) -> Option<String> {
        ::which::which_global(self.as_ref())
            .ok()
            .map(|s| s.unwrap_to_string())
    }

    fn parent(&self) -> Option<String> {
        pb(self).parent().map(|s| s.unwrap_to_string())
    }

    fn read_lines(&self) -> Vec<String> {
        let bp = self.to_path_buf();
        let path = bp.as_path();
        let br = std::io::BufReader::new(std::fs::File::open(path).unwrap());
        br.lines().map(|line| line.unwrap()).collect()
    }
}

pub fn me() -> String {
    std::env::current_exe().unwrap_to_string()
}

pub fn argv<'a>() -> Vec<String> {
    std::env::args().collect()
}

pub fn args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

pub mod cli {
    // use super::File;

    pub fn me() -> String {
        std::env::args().next().unwrap()
    }

    pub fn argv() -> Vec<String> {
        std::env::args().collect()
    }

    pub fn program_args() -> Vec<String> {
        std::env::args().skip(1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
