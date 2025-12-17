//! Image utilities for TUI rendering.
//!
//! Provides helper functions for loading and processing images, particularly
//! extracting the first frame from GIF files for rendering.

use std::path::Path;
use std::io::BufReader;

use image::RgbaImage;
use image::Rgba;

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

/// Simple image cache placeholder (kept for now, not actively used).
#[derive(Default)]
pub struct ImageCache;

impl ImageCache {
    /// Create a new image cache.
    pub fn new() -> Self {
        Self
    }

    /// Extract the first frame from an image file, properly handling GIFs
    ///
    /// For regular images, returns the image as-is.
    /// For GIFs, extracts and composites the first frame with proper transparency handling.
    pub fn extract_first_frame(path: &Path) -> Result<image::DynamicImage, ImageError> {
        use std::fs::File;

        let file = File::open(path)
            .map_err(|_| ImageError::NotFound)?;
        let reader = BufReader::new(file);

        // Try to decode as GIF first
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        match options.read_info(reader) {
            Ok(mut decoder) => {
                // This is a GIF - extract first frame
                let width = decoder.width() as u32;
                let height = decoder.height() as u32;

                if let Ok(Some(frame)) = decoder.read_next_frame() {
                    // Create canvas for first frame
                    let mut canvas = RgbaImage::from_pixel(width, height, Rgba([0, 0, 0, 0]));

                    // Composite first frame onto canvas
                    let frame_buffer = &frame.buffer;
                    let frame_width = frame.width as u32;
                    let frame_height = frame.height as u32;
                    let left = frame.left as u32;
                    let top = frame.top as u32;

                    for y in 0..frame_height {
                        for x in 0..frame_width {
                            let src_idx = ((y * frame_width + x) * 4) as usize;
                            if src_idx + 3 < frame_buffer.len() {
                                let pixel = Rgba([
                                    frame_buffer[src_idx],
                                    frame_buffer[src_idx + 1],
                                    frame_buffer[src_idx + 2],
                                    frame_buffer[src_idx + 3],
                                ]);

                                let canvas_x = left + x;
                                let canvas_y = top + y;

                                if canvas_x < width && canvas_y < height && pixel[3] > 0 {
                                    canvas.put_pixel(canvas_x, canvas_y, pixel);
                                }
                            }
                        }
                    }

                    Ok(image::DynamicImage::ImageRgba8(canvas))
                } else {
                    // Failed to read first frame, try as regular image
                    image::ImageReader::open(path)
                        .ok()
                        .and_then(|r| r.decode().ok())
                        .ok_or_else(|| ImageError::InvalidFormat("Failed to decode GIF first frame".to_string()))
                }
            }
            Err(_) => {
                // Not a GIF or already read - try regular image decoder
                image::ImageReader::open(path)
                    .ok()
                    .and_then(|r| r.decode().ok())
                    .ok_or_else(|| ImageError::InvalidFormat("Unsupported image format".to_string()))
            }
        }
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

    // Note: Integration tests for image loading would require actual image files.
    // Basic unit tests above verify cache structure and initialization.
}
