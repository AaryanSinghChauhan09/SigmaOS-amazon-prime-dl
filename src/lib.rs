//! SigmaOS-amazon-prime-dl: Amazon Prime video downloader model.
//!
//! Real implementation: videos, downloads, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Video {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Download {
    pub id: String,
    pub video_id: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmazonPrimeDlError {
    EmptyVideoName,
    DuplicateVideo(String),
}

impl fmt::Display for AmazonPrimeDlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVideoName => write!(f, "video name must not be empty"),
            Self::DuplicateVideo(id) => write!(f, "duplicate video {:?}", id),
        }
    }
}

impl std::error::Error for AmazonPrimeDlError {}

pub struct AmazonPrimeDl {
    pub name: String,
    videos: BTreeMap<String, Video>,
    downloads: BTreeMap<String, Download>,
}

impl AmazonPrimeDl {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), videos: BTreeMap::new(), downloads: BTreeMap::new() }
    }

    pub fn add_video(&mut self, v: Video) -> Result<(), AmazonPrimeDlError> {
        if v.name.trim().is_empty() { return Err(AmazonPrimeDlError::EmptyVideoName); }
        let id = v.id.clone();
        if self.videos.contains_key(&id) {
            return Err(AmazonPrimeDlError::DuplicateVideo(id.clone()));
        }
        self.videos.insert(id, v);
        Ok(())
    }

    pub fn video_count(&self) -> usize { self.videos.len() }

    pub fn get_video(&self, id: &str) -> Option<&Video> { self.videos.get(id) }

    pub fn add_download(&mut self, d: Download) {
        self.downloads.insert(d.id.clone(), d);
    }

    pub fn get_download(&self, id: &str) -> Option<&Download> { self.downloads.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = AmazonPrimeDl::new("AmazonPrimeDl");
        assert_eq!(a.name, "AmazonPrimeDl");
        assert_eq!(a.video_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = AmazonPrimeDl::new("AmazonPrimeDl");
        let video = Video { id: "v1".into(), name: "Movie".into(), url: "https://prime.com/m".into() };
        a.add_video(video).unwrap();
        assert_eq!(a.video_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = AmazonPrimeDl::new("AmazonPrimeDl");
        let video = Video { id: "x".into(), name: "".into(), url: "x".into() };
        assert!(a.add_video(video).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AmazonPrimeDl::new("AmazonPrimeDl");
        let video = Video { id: "x".into(), name: "X".into(), url: "x".into() };
        a.add_video(video).unwrap();
        let video2 = Video { id: "x".into(), name: "X".into(), url: "x".into() };
        assert!(a.add_video(video2).is_err());
    }
}
