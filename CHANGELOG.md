# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Updated author to epistates, inc. <nick@epistates.com>
- Enhanced README with badges (crates.io, docs.rs, license, CI)
- Restructured README with release focus
- Added Overview section to README
- Simplified Features section with emoji headers
- Updated Installation section for crates.io
- Replaced development roadmap with Contributing and Roadmap sections

### Added
- MIT LICENSE file
- Comprehensive inline documentation for docs.rs
- Module-level documentation for main.rs and parser modules
- Function and struct documentation with rustdoc formatting
- Cargo.toml metadata: homepage, documentation, readme, rust-version
- .gitignore updated to allow CHANGELOG.md

## [0.4.0] - 2025-11-07 - Themes & Clipboard 🎨📋

### Fixed

**Section Extraction with Code Blocks:**
- Fixed bug where section extraction would incorrectly stop at headings inside code blocks
- Both TUI content display and clipboard copy (`y`) now correctly handle code blocks
- CLI section extraction (`-s`) also fixed to skip code fence content
- Example: Extracting "List all headings" section now includes full content with ``` code blocks containing `# headings`

**Code Fence Visibility:**
- Fixed nearly invisible ``` code fence markers in all themes
- Added `code_fence` color to Theme struct with visible colors for all 6 themes
- Code fences now use theme-specific colors instead of dim hardcoded gray
- Ocean Dark: bright cyan-blue, Nord: frost cyan, Dracula: purple, Solarized: cyan, Monokai: cyan, Gruvbox: aqua

### Improved

**Smart Collapse:**
- When on a child heading with no children, pressing `h` or `←` now collapses the parent heading
- Makes navigation more intuitive - you can quickly collapse sections from anywhere in the subtree
- Selection automatically moves to the parent when collapsing from a child

**Scrollable Help Menu:**
- Help popup (`?`) is now scrollable with `j/k` or `↓/↑` keys
- All content is now accessible even on smaller terminal windows
- Scrollbar indicator shows current position
- Help menu size increased to 80% of screen height
- Scroll instructions shown at both top and bottom of help menu for better discoverability

**Bookmark Visual Indicator:**
- Bookmarked headings now display a `⚑` flag indicator in the outline
- Makes it easy to see at a glance which heading is bookmarked
- Indicator appears next to the heading text when bookmark is set with `m` key

**Enhanced Status Bar:**
- Redesigned with bullet separators for better visual scanning
- Grouped related actions together: "Copy: y:Text Y:Link" makes it crystal clear
- Clearer labels: "w:View" instead of "w:Toggle", "[]:Size" instead of "[]:Resize"
- All clipboard functionality visible on main screen without opening help
- Format: `[Focus] pos/total (%)⚑ • Outline:30% • w:View • []:Size • m:Mark • Copy: y:Text Y:Link • t:Theme • ?:Help`

### Added - Themes

**8 Beautiful Color Schemes:**
- **Ocean Dark** (default) - Base16 Ocean with cool blues
- **Nord** - Arctic, north-bluish color palette
- **Dracula** - Dark theme with vibrant colors
- **Solarized** - Precision colors for machines and people
- **Monokai** - Sublime Text's iconic color scheme
- **Gruvbox** - Retro groove color scheme
- **Tokyo Night** ✨ - Modern night theme celebrating Tokyo's neon lights (optimized for low-light)
- **Catppuccin Mocha** ✨ - Soothing pastel theme for cozy night coding (warm, eye-friendly)

**Theme Features:**
- Press `t` to open interactive theme picker menu
- Navigate themes with `j/k` or `↓/↑` keys
- Current theme marked with ✓ checkmark
- Selected theme shows description
- Press Enter to apply, Esc to cancel
- Theme applies to all UI elements (outline, content, status bar, help)
- Each theme has carefully chosen colors for headings, borders, and UI elements
- Full theme integration with foreground/background colors
- Inline markdown formatting (bold, italic, code) respects theme colors
- List bullets and blockquotes use theme-specific colors

**Night Mode Themes:**
- **Tokyo Night** - Very dark blue-black background (RGB 26,27,38) with soft neon accents
  - Inspired by Tokyo's nighttime cityscape
  - Reduced blue light exposure with warm purples and oranges
  - Perfect for late-night coding sessions
- **Catppuccin Mocha** - Deep navy background (RGB 30,30,46) with pastel accents
  - Soothing, low-saturation colors designed for extended viewing
  - Warm pink and peach tones reduce eye strain
  - Cozy, comfortable aesthetic for night work

### Added - Clipboard Integration

**Copy Functionality:**
- Press `y` to copy current section content to clipboard
- Press `Y` to copy anchor link (#heading-name format) for current heading
- Uses arboard for cross-platform clipboard support (Linux/macOS/Windows)
- Wayland support included for modern Linux systems

### Technical
- Created theme module with Theme struct and ThemeName enum
- Added `arboard` dependency with wayland-data-control feature
- Added `extract_section()` method to Document for section extraction
- Fixed section extraction to track code block boundaries (```) and skip heading detection within code
- Both `extract_section_content()` (TUI) and `Document.extract_section()` (CLI/copy) now handle code blocks
- Theme state integrated into App struct
- UI rendering fully refactored to use theme colors throughout
- `format_inline_markdown()` now accepts theme parameter
- All markdown rendering uses theme colors (headings, lists, blockquotes, inline styles)
- Removed unused code: `render_tree()`, `heading_index` field, `ThemeName::name()` method
- Implemented `text_style()`, `content_style()`, and `code_fence_style()` theme methods
- Background and foreground colors now applied to all content areas
- Added `code_fence` color field to all 8 themes
- Code fence rendering in `render_markdown_enhanced()` now uses theme instead of hardcoded colors
- Added `TokyoNight` and `CatppuccinMocha` to ThemeName enum
- Implemented `tokyo_night()` and `catppuccin_mocha()` theme constructors with official color palettes
- Replaced theme cycling with interactive theme picker menu
- Added `show_theme_picker` and `theme_picker_selected` state to App
- Implemented `render_theme_picker()` with centered popup using existing `centered_area()` helper
- Added `toggle_theme_picker()`, `theme_picker_next()`, `theme_picker_previous()`, and `apply_selected_theme()` methods
- Theme picker shows all 8 themes with descriptions and current theme marker (✓)
- Removed unused `ThemeName::next()` method
- Bookmark indicator renders in outline by checking `bookmark_position` against item index
- Help overlay updated with scroll instructions at top and bottom for better UX
- Status bar updated to include `y:Copy` and `Y:Anchor` shortcuts in main view
- Added `justfile` with common development tasks (build, test, install, release, CI, etc.)
- Optimized `Cargo.toml` release profile with LTO, size optimization, and symbol stripping
- Release binary size reduced to 1.9MB with opt-level="z", lto=true, codegen-units=1, strip=true
- Zero compiler warnings

## [0.3.6] - 2025-11-07 - Collapse Fix 🐛

### Fixed

**Tree Collapse/Expand:**
- Fixed collapse functionality - headings with children can now be properly collapsed and expanded
- Added `has_children` tracking to `OutlineItem` for efficient child detection
- Visual indicators (▼ expanded, ▶ collapsed) now correctly show for parent headings
- Collapse/expand only works on headings that have children (prevents confusion)
- Fixed focus management when toggling outline visibility

### Technical
- Added `collapsed_headings: HashSet<String>` to persist collapse state
- Modified `flatten_tree()` to respect collapsed state when rebuilding
- Added `has_children: bool` field to `OutlineItem` struct
- Improved toggle/expand/collapse functions to check for children before acting

## [0.3.5] - 2025-11-07 - UX Polish Complete ✨

### Added - UX Enhancements

**Toggle Outline Visibility:**
- Press `w` to hide/show outline pane
- Full-width content when hidden
- Perfect for immersive reading mode
- Focus automatically switches to content when hidden

**Adjustable Pane Split:**
- Press `]` to increase outline width: 20% → 30% → 40%
- Press `[` to decrease outline width: 40% → 30% → 20%
- Three size presets for different workflows
- Real-time layout adjustment

**Jump to Heading by Number:**
- Press `1` through `9` to jump to headings 1-9
- Instant navigation without scrolling
- Perfect for documents with important sections
- Automatically updates scrollbar position

**Bookmark System:**
- Press `m` to set bookmark at current position
- Press `'` (single quote) to jump to bookmark
- Bookmark indicator ⚑ appears in status bar
- Great for cross-referencing in long documents

**Enhanced UI:**
- Title bar now shows heading count: "treemd - 12 headings"
- Status bar shows outline width: "Outline:30%" or "Outline:Hidden"
- Bookmark indicator in status bar
- Updated help overlay with all new features

### Technical
- Added `show_outline: bool` to App state
- Added `outline_width: u16` for adjustable split
- Added `bookmark_position: Option<usize>` for bookmarks
- Conditional layout rendering based on outline visibility
- Smart focus management when outline hidden

### Changed
- Help overlay expanded with "UX Features" section
- Status bar redesigned with more information
- Title bar made more informative
- Tab key respects outline visibility

### Project Renamed
- **mdless** → **treemd**
- Updated all documentation
- New repository URL structure
- Updated binary name and CLI
- Better reflects tree-based navigation

## [0.3.0] - 2025-11-07 - Phase 3 Complete ✨

### Added - Enhanced Rendering

**Syntax Highlighting:**
- Full syntect integration with 50+ programming languages
- Base16 Ocean Dark theme for code blocks
- Automatic language detection from ``` markers
- Fallback to plain text for unknown languages
- Rust, Python, JavaScript, TypeScript, Go, C/C++, Java, and more

**Inline Formatting:**
- Bold text (`**text**`) - White color with bold modifier
- Italic text (`*text*`) - Light purple with italic styling
- Inline code (`` `code` ``) - Orange text with dark gray background
- Formatting works in all contexts (paragraphs, lists, blockquotes)

**Search/Filter:**
- Press `/` to open search overlay
- Real-time filtering of headings as you type
- Case-insensitive substring matching
- Enter to accept, Esc to clear
- Maintains outline tree structure
- Visual search box overlay at top-left

**Enhanced Rendering Pipeline:**
- `render_markdown_enhanced()` - New rendering function
- Code block accumulation and batch highlighting
- `format_inline_markdown()` - Character-by-character parser
- Mixed content handling (inline + block elements)
- Preserved indentation and formatting

### Technical
- Added syntect 5.2 for syntax highlighting
- Added nucleo-matcher 0.3 (prepared for Phase 4)
- New `syntax.rs` module with `SyntaxHighlighter`
- Enhanced keyboard handling for search mode
- Search state management in App struct

### Performance
- Code highlighting: ~5ms per block
- Search filtering: <1ms for 100+ headings
- Inline parsing: ~0.1ms per line
- Binary size: 3.3MB (up from 2.2MB)

### UI/UX
- Updated status bar with `/` key hint
- Updated help overlay with search instructions
- Search overlay with yellow border
- Cursor indicator `_` in search box

## [0.2.0] - 2025-11-07 - Phase 2 Complete ✨

### Added - TUI Mode

**Core Interface:**
- Interactive dual-pane layout (30% outline, 70% content)
- Vim-style keyboard navigation (j/k, g/G, d/u)
- Tab key to switch focus between panes
- Visual focus indicators (colored borders)

**Enhanced Navigation:**
- Scrollbars with position indicators (↑/↓)
- Page up/down scrolling (d/u keys)
- Jump to top/bottom (g/G)
- Synchronized scroll state tracking

**Help System:**
- Help popup overlay (press `?`)
- Comprehensive keyboard shortcut reference
- Styled, centered popup with semi-transparent background
- Easy dismissal (Esc or ? again)

**Rich Markdown Rendering:**
- Color-coded headings by level (5 distinct colors)
- Bullet points styled as `•` with cyan color
- Blockquotes with left border `│` and italic styling
- Code block markers with dimmed styling
- Numbered lists with color highlighting
- Preserved indentation and formatting

**Status Bar:**
- Current focus indicator `[Outline]` / `[Content]`
- Position tracker (e.g., `5/12`)
- Percentage progress `(42%)`
- Quick keyboard shortcut reference
- Custom color scheme (dark blue-gray background)

**Tree Operations:**
- Expand/collapse headings (Enter, Space, h/l)
- Collapsible heading tree structure
- Smart section extraction
- Auto-focus on selected section in content pane

**Visual Design:**
- Custom color palette with RGB colors
- Level-based heading colors (blue → green gradient)
- Smooth, modern UI with ratatui
- Professional styling throughout

### Technical Improvements
- Added ratatui 0.29 for TUI
- Added crossterm 0.28 for terminal handling
- Added color-eyre 0.6 for error handling
- ScrollbarState integration for both panes
- Efficient stateful widget rendering
- Zero-cost event handling

### Binary Stats
- Release build size: 2.2MB
- Dependencies: 4 main crates
- Target: macOS/Linux/Windows

## [0.1.0] - 2025-11-07 - Phase 1 Complete

### Added - CLI Mode

**Core Features:**
- Markdown parsing with pulldown-cmark
- Heading tree extraction and hierarchy
- List all headings (`-l` flag)
- Tree visualization (`--tree` flag)
- Section extraction (`-s "Section Name"`)
- Filter by heading level (`-L 2`)
- Filter by text (`--filter "keyword"`)
- Count headings by level (`--count`)

**Output Formats:**
- Plain text (default)
- JSON output (`-o json`)
- Tree format with box-drawing characters

**Architecture:**
- Modular design (parser, cli, tui modules)
- Clean separation of concerns
- Extensible for future features

### Project Setup
- Cargo workspace configuration
- MIT license
- Comprehensive README
- Demo file with examples
