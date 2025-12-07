//! File system watcher for live reload functionality.
//!
//! Watches the currently open file for changes and notifies the TUI
//! to reload when modifications are detected.

use notify::{
    event::{AccessKind, AccessMode, ModifyKind},
    Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::{Duration, Instant};

/// Manages file watching for live reload.
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<Result<Event, notify::Error>>,
    current_path: Option<PathBuf>,
    /// Debounce: ignore events within this duration of the last reload
    last_reload: Instant,
    debounce_duration: Duration,
}

impl FileWatcher {
    /// Create a new file watcher.
    pub fn new() -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();
        let watcher = notify::recommended_watcher(tx)?;

        Ok(Self {
            watcher,
            receiver: rx,
            current_path: None,
            last_reload: Instant::now(),
            debounce_duration: Duration::from_millis(100),
        })
    }

    /// Start watching a file. Stops watching any previously watched file.
    pub fn watch(&mut self, path: &PathBuf) -> Result<(), notify::Error> {
        // Unwatch previous file if any
        if let Some(ref old_path) = self.current_path {
            let _ = self.watcher.unwatch(old_path);
        }

        // Watch the new file (non-recursive since it's a single file)
        self.watcher.watch(path, RecursiveMode::NonRecursive)?;
        self.current_path = Some(path.clone());

        // Reset debounce timer
        self.last_reload = Instant::now();

        Ok(())
    }

    /// Stop watching the current file.
    #[allow(dead_code)]
    pub fn unwatch(&mut self) {
        if let Some(ref path) = self.current_path {
            let _ = self.watcher.unwatch(path);
        }
        self.current_path = None;
    }

    /// Check if the watched file has been modified.
    /// Returns true if a reload should be triggered.
    pub fn check_for_changes(&mut self) -> bool {
        // Drain all pending events
        let mut should_reload = false;

        loop {
            match self.receiver.try_recv() {
                Ok(Ok(event)) => {
                    // Check if this is a modification event we care about
                    if self.is_relevant_event(&event) {
                        should_reload = true;
                    }
                }
                Ok(Err(_)) => {
                    // Watch error, ignore
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        // Apply debouncing
        if should_reload {
            let now = Instant::now();
            if now.duration_since(self.last_reload) >= self.debounce_duration {
                self.last_reload = now;
                return true;
            }
        }

        false
    }

    /// Mark that a reload just happened (for debouncing after internal saves).
    #[allow(dead_code)]
    pub fn mark_reloaded(&mut self) {
        self.last_reload = Instant::now();
    }

    /// Check if an event is relevant for triggering a reload.
    fn is_relevant_event(&self, event: &Event) -> bool {
        // Only care about events for our watched file
        if let Some(ref watched_path) = self.current_path {
            let matches_path = event.paths.iter().any(|p| p == watched_path);
            if !matches_path {
                return false;
            }
        } else {
            return false;
        }

        // Check event kind - we care about modifications and writes
        matches!(
            event.kind,
            EventKind::Modify(ModifyKind::Data(_))
                | EventKind::Modify(ModifyKind::Any)
                | EventKind::Access(AccessKind::Close(AccessMode::Write))
                | EventKind::Create(_)
        )
    }

    /// Get the currently watched path.
    #[allow(dead_code)]
    pub fn current_path(&self) -> Option<&PathBuf> {
        self.current_path.as_ref()
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new().expect("Failed to create file watcher")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watcher_creation() {
        let watcher = FileWatcher::new();
        assert!(watcher.is_ok());
    }
}
