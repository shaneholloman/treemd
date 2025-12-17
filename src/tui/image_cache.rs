//! Image caching and lazy loading system for TUI rendering.
//!
//! This module manages loading and caching images with lazy loading based on viewport visibility.
//! It uses `ratatui-image`'s Picker to detect terminal capabilities and render images using
//! the appropriate graphics protocol (Sixel, Kitty, iTerm2, or halfblocks fallback).
//!
//! ## Architecture
//!
//! - **Picker**: Initialized once after entering alternate screen, detects graphics protocol
//! - **Cache**: LRU-evicted HashMap storing loaded images with dimensions
//! - **Lazy Loading**: Images loaded only when visible in viewport
//! - **Background Loading**: Uses ThreadProtocol for non-blocking image processing

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use image::GenericImageView;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;

/// Result type for image cache operations
pub type ImageCacheResult<T> = Result<T, ImageError>;

/// Errors that can occur during image loading and caching
#[derive(Debug, Clone)]
pub enum ImageError {
    /// Image file not found
    NotFound,
    /// Image is currently being loaded
    Loading,
    /// Invalid image format
    InvalidFormat(String),
    /// Other errors (IO, parsing, etc.)
    Failed(String),
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::NotFound => write!(f, "Image not found"),
            ImageError::Loading => write!(f, "Image loading"),
            ImageError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            ImageError::Failed(s) => write!(f, "Failed: {}", s),
        }
    }
}

impl std::error::Error for ImageError {}

/// Cached image with metadata
struct CachedImage {
    /// The loaded image protocol
    protocol: Box<dyn StatefulProtocol>,
    /// Last access time for LRU eviction
    last_used: Instant,
    /// Original image width in pixels
    width: u32,
    /// Original image height in pixels
    height: u32,
}

/// Image cache with lazy loading and LRU eviction.
///
/// Manages loading and caching of images with:
/// - Terminal capability detection via Picker
/// - Lazy loading based on viewport visibility
/// - LRU eviction when cache exceeds max size
/// - Support for multiple image formats
#[derive(Default)]
pub struct ImageCache {
    /// Terminal graphics protocol picker (initialized once)
    picker: Option<Picker>,
    /// Cached loaded images, keyed by file path
    loaded_images: HashMap<PathBuf, CachedImage>,
    /// Maximum number of images to keep in cache (default: 10)
    max_cache_size: usize,
}

impl ImageCache {
    /// Create a new image cache
    pub fn new() -> Self {
        Self {
            picker: None,
            loaded_images: HashMap::new(),
            max_cache_size: 10,
        }
    }

    /// Initialize the Picker for graphics protocol detection.
    ///
    /// This should be called once after entering alternate screen mode.
    /// If initialization fails, images will fallback to placeholder rendering.
    pub fn initialize(&mut self) -> Result<(), String> {
        match Picker::from_termios() {
            Ok(picker) => {
                self.picker = Some(picker);
                Ok(())
            }
            Err(e) => {
                // Gracefully handle picker initialization failure
                // Images can still be displayed as placeholders
                Err(format!("Failed to initialize image picker: {}", e))
            }
        }
    }

    /// Get the Picker instance (if initialized)
    pub fn picker(&self) -> Option<&Picker> {
        self.picker.as_ref()
    }

    /// Check if an image is currently cached
    pub fn has_cached(&self, path: &Path) -> bool {
        self.loaded_images.contains_key(path)
    }

    /// Check if an image is currently loading
    ///
    /// This is a placeholder for future async loading implementation.
    /// Currently always returns false since we implement sync loading.
    pub fn is_loading(&self, _path: &Path) -> bool {
        false
    }

    /// Load an image file and add it to the cache.
    ///
    /// Returns an error if:
    /// - File not found
    /// - Invalid image format
    /// - Graphics protocol not available
    ///
    /// Uses LRU eviction if cache exceeds max_cache_size.
    pub fn load_image(&mut self, path: &Path) -> ImageCacheResult<()> {
        // Check if already cached
        if let Some(cached) = self.loaded_images.get_mut(path) {
            cached.last_used = Instant::now();
            return Ok(());
        }

        // Require picker to be initialized
        let picker = self
            .picker
            .as_mut()
            .ok_or_else(|| ImageError::Failed("Image picker not initialized".to_string()))?;

        // Load image file
        let image = image::ImageReader::open(path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ImageError::NotFound
                } else {
                    ImageError::Failed(format!("IO error: {}", e))
                }
            })?
            .decode()
            .map_err(|e| ImageError::InvalidFormat(format!("{}", e)))?;

        // Get image dimensions before creating protocol
        let (width, height) = image.dimensions();

        // Create protocol for this image
        // Note: new_resize_protocol doesn't return a Result in ratatui-image v1.0.5
        let protocol = picker.new_resize_protocol(image);

        // Add to cache
        self.loaded_images.insert(
            path.to_path_buf(),
            CachedImage {
                protocol,
                last_used: Instant::now(),
                width,
                height,
            },
        );

        // Evict LRU if over capacity
        self.evict_lru();

        Ok(())
    }

    /// Get a reference to a cached image's protocol
    pub fn get_protocol(&mut self, path: &Path) -> Option<&dyn StatefulProtocol> {
        if let Some(cached) = self.loaded_images.get_mut(path) {
            cached.last_used = Instant::now();
            return Some(&*cached.protocol);
        }
        None
    }

    /// Get the dimensions of a cached image
    pub fn get_dimensions(&self, path: &Path) -> Option<(u32, u32)> {
        self.loaded_images
            .get(path)
            .map(|cached| (cached.width, cached.height))
    }

    /// Clear all cached images
    pub fn clear(&mut self) {
        self.loaded_images.clear();
    }

    /// Evict the least recently used image if cache exceeds max size
    fn evict_lru(&mut self) {
        if self.loaded_images.len() <= self.max_cache_size {
            return;
        }

        // Find least recently used
        if let Some((path, _)) = self
            .loaded_images
            .iter()
            .min_by_key(|(_, cached)| cached.last_used)
            .map(|(p, c)| (p.clone(), c.last_used))
        {
            self.loaded_images.remove(&path);
        }
    }

    /// Set the maximum cache size
    pub fn set_max_cache_size(&mut self, size: usize) {
        self.max_cache_size = size;
        // Evict if needed
        while self.loaded_images.len() > self.max_cache_size {
            self.evict_lru();
        }
    }

    /// Get cache statistics for debugging
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.loaded_images.len(), self.max_cache_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cache() {
        let cache = ImageCache::new();
        assert_eq!(cache.cache_stats(), (0, 10));
    }

    #[test]
    fn test_cache_size_limit() {
        let mut cache = ImageCache::new();
        cache.set_max_cache_size(3);
        assert_eq!(cache.cache_stats(), (0, 3));
    }

    #[test]
    fn test_clear() {
        let mut cache = ImageCache::new();
        // Note: Can't test loading without actual files, but test clearing works
        cache.loaded_images
            .insert(PathBuf::from("test.png"), CachedImage {
                protocol: Box::new(create_dummy_protocol()),
                last_used: Instant::now(),
                width: 100,
                height: 100,
            });
        assert_eq!(cache.cache_stats(), (1, 10));
        cache.clear();
        assert_eq!(cache.cache_stats(), (0, 10));
    }

    // Helper for tests
    fn create_dummy_protocol() -> DummyProtocol {
        DummyProtocol
    }

    struct DummyProtocol;

    impl StatefulProtocol for DummyProtocol {
        fn dimensions(&self) -> (u32, u32) {
            (100, 100)
        }

        fn resize(&mut self, _new_width: u32, _new_height: u32) {}

        fn render(&self, _buf: &mut ratatui::buffer::Buffer, _area: ratatui::prelude::Rect) {}
    }
}
