// SPDX-License-Identifier: MPL-2.0
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::io;
use std::path::{Path, PathBuf};

pub trait UnwrapToString {
    fn unwrap_to_string(&self) -> String;
}

pub trait OwnedUnwrapToString {
    fn unwrap_to_string(self) -> String;
}

impl<T: OwnedUnwrapToString, S: Debug> OwnedUnwrapToString for Result<T, S> {
    fn unwrap_to_string(self) -> String {
        self.unwrap().unwrap_to_string()
    }
}

impl UnwrapToString for Option<&str> {
    fn unwrap_to_string(&self) -> String {
        self.unwrap_or("").to_string()
    }
}

impl OwnedUnwrapToString for Option<String> {
    fn unwrap_to_string(self) -> String {
        self.unwrap_or(String::with_capacity(0))
    }
}

impl<T: Debug> OwnedUnwrapToString for Result<String, T> {
    fn unwrap_to_string(self) -> String {
        self.unwrap()
    }
}

impl<'a, T: Debug> OwnedUnwrapToString for Result<&'a str, T> {
    fn unwrap_to_string(self) -> String {
        self.unwrap().to_string()
    }
}

impl OwnedUnwrapToString for OsString {
    fn unwrap_to_string(self) -> String {
        self.into_string().unwrap()
    }
}

impl UnwrapToString for Option<&OsStr> {
    fn unwrap_to_string(&self) -> String {
        self.map(|os_str| os_str.to_os_string().unwrap_to_string())
            .unwrap()
    }
}

impl UnwrapToString for Option<&Path> {
    fn unwrap_to_string(&self) -> String {
        self.map(|p| p.to_str().unwrap_or("")).unwrap_to_string()
    }
}

impl UnwrapToString for &Path {
    fn unwrap_to_string(&self) -> String {
        self.to_str().unwrap_to_string()
    }
}

impl OwnedUnwrapToString for PathBuf {
    fn unwrap_to_string(self) -> String {
        self.into_os_string().into_string().unwrap()
    }
}
