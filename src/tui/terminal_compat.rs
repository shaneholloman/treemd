use supports_color::{Stream, on};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorMode {
    Rgb,        // True color (16M colors)
    Indexed256, // 256-color palette
}

#[derive(Debug)]
pub struct TerminalCapabilities {
    pub supports_rgb: bool,
    pub is_terminal_app: bool,
    pub macos_version: Option<u32>,
    pub recommended_color_mode: ColorMode,
    pub should_warn: bool,
}

impl TerminalCapabilities {
    /// Detect terminal capabilities and recommend appropriate color mode
    pub fn detect() -> Self {
        let is_terminal_app = std::env::var("TERM_PROGRAM")
            .map(|v| v == "Apple_Terminal")
            .unwrap_or(false);

        let supports_rgb = on(Stream::Stdout)
            .map(|level| level.has_16m)
            .unwrap_or(false);

        let macos_version = Self::detect_macos_version();

        // Determine if we should warn and which color mode to use
        let (should_warn, recommended_color_mode) = if is_terminal_app {
            match macos_version {
                Some(version) if version >= 26 => {
                    // macOS 26+ (Tahoe and later) - Terminal.app works well
                    (false, ColorMode::Rgb)
                }
                Some(_) | None => {
                    // macOS < 26 (Sequoia and earlier) or unknown - use fallback
                    (true, ColorMode::Indexed256)
                }
            }
        } else {
            // Not Terminal.app - trust the terminal's capabilities
            let mode = if supports_rgb {
                ColorMode::Rgb
            } else {
                ColorMode::Indexed256
            };
            (false, mode)
        };

        Self {
            supports_rgb,
            is_terminal_app,
            macos_version,
            recommended_color_mode,
            should_warn,
        }
    }

    /// Detect macOS Darwin version (e.g., 24 for Sequoia, 26 for Tahoe)
    fn detect_macos_version() -> Option<u32> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            // Run `uname -r` to get Darwin version (e.g., "24.6.0" for Sequoia)
            let output = Command::new("uname").arg("-r").output().ok()?;

            let version_str = String::from_utf8(output.stdout).ok()?;
            let major_version = version_str.split('.').next()?.parse::<u32>().ok()?;

            Some(major_version)
        }

        #[cfg(not(target_os = "macos"))]
        None
    }

    /// Get a user-friendly warning message
    pub fn warning_message(&self) -> Option<String> {
        if !self.should_warn {
            return None;
        }

        Some(format!(
            "⚠️  Terminal Compatibility Notice\n\n\
             Apple Terminal.app on macOS {} has limited RGB color support.\n\
             Switching to 256-color mode for better compatibility.\n\n\
             For the best experience, consider using:\n\
             • iTerm2 (https://iterm2.com/)\n\
             • Kitty (https://sw.kovidgoyal.net/kitty/)\n\
             • Alacritty (https://alacritty.org/)\n\n\
             Press any key to continue...",
            self.macos_version
                .map(|v| format!("Sequoia (Darwin {})", v))
                .unwrap_or_else(|| "< 26".to_string())
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capabilities_detection() {
        let caps = TerminalCapabilities::detect();
        // Just ensure it doesn't panic
        println!("Detected capabilities: {:?}", caps);
    }
}
