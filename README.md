# treemd

A markdown navigator with tree-based structural navigation. Like `tree`, but interactive—navigate markdown documents using an expandable/collapsible heading tree with a synchronized content view.

## Features

### Phase 1: CLI Mode ✅

- **List headings**: Quick overview of document structure
- **Tree view**: Hierarchical visualization with box-drawing
- **Section extraction**: Extract specific sections by heading name
- **Filtering**: Filter by text or heading level
- **Multiple output formats**: Plain text, JSON
- **Heading statistics**: Count headings by level

### Phase 2: TUI Mode ✅

- **Interactive dual-pane interface** - Outline + content view
- **Scrollbars** - Visual scrolling indicators for both panes
- **Help overlay** - Press `?` for keyboard shortcuts
- **Rich styling** - Color-coded headings by level
- **Basic markdown rendering** - Headings, lists, blockquotes, code blocks
- **Vim-style navigation** - j/k, g/G, d/u for efficient browsing
- **Smart content display** - Auto-focus on selected section
- **Status bar** - Shows position, percentage, and quick help

### Phase 3: Enhanced Rendering ✅

- **Syntax-highlighted code blocks** - Full syntect integration with 50+ languages
- **Inline formatting** - Bold (`**text**`), italic (`*text*`), inline code (`` `code` ``)
- **Search/filter overlay** - Press `/` to filter headings in real-time
- **Enhanced markdown** - Beautiful rendering of all common elements
- **Theme support** - Base16 Ocean Dark for code highlighting
- **Performance optimized** - Efficient rendering pipeline

### Phase 3.5: UX Polish ✅

- **Toggle outline visibility** - Press `w` for full-width content reading
- **Adjustable pane split** - Press `[`/`]` to resize outline (20%, 30%, 40%)
- **Jump to heading by number** - Press `1-9` for instant navigation
- **Bookmark system** - Press `m` to mark, `'` to return
- **Enhanced title bar** - Shows heading count
- **Smart status bar** - Displays outline width, bookmark indicator ⚑

### Planned (Phase 4+)

- OFM (Obsidian Flavored Markdown) callouts and wikilinks
- Multiple color themes (Nord, Dracula, Solarized)
- Theme switcher (`t` key)
- Configuration file (`~/.config/treemd/config.toml`)
- Fuzzy search with nucleo
- Tables with box-drawing
- Watch mode (auto-reload on file change)

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
./target/release/treemd
```

## Usage

### TUI Mode (Interactive - Default)

Simply run treemd without flags to launch the interactive interface:

```bash
treemd README.md
```

**Keyboard Shortcuts:**

*Navigation:*
- `j/k` or `↓/↑` - Navigate up/down
- `g/G` - Jump to top/bottom
- `d/u` - Page down/up (in content)
- `Tab` - Switch between outline and content
- `1-9` - Jump to heading 1-9 (instant access)

*Tree Operations:*
- `Enter/Space` - Toggle expand/collapse
- `h/l` or `←/→` - Collapse/expand heading

*UX Features:*
- `w` - Toggle outline visibility (full-width content)
- `[` `]` - Decrease/increase outline width (20%, 30%, 40%)
- `m` - Set bookmark at current position
- `'` - Jump to bookmarked position

*Search & Help:*
- `/` - Search/filter headings (type to filter, Esc to clear)
- `?` - Toggle help overlay
- `q/Esc` - Quit

**Interface Features:**
- **Syntax-highlighted code blocks** - 50+ languages supported
- **Inline formatting** - Bold, italic, inline code with colors
- **Real-time search** - Filter headings as you type (press `/`)
- **Toggle outline** - Hide for full-width reading (press `w`)
- **Adjustable layout** - Resize outline 20%/30%/40% (press `[` `]`)
- **Quick navigation** - Jump to any heading 1-9 instantly
- **Bookmarks** - Mark and return to positions (press `m` and `'`)
- **Color-coded headings** - 5 distinct levels
- **Scrollbars** - Position indicators on both panes
- **Smart status bar** - Shows position, outline width, bookmark status
- **Help overlay** - Always available (press `?`)

### CLI Mode (Non-Interactive)

### List all headings

```bash
treemd -l README.md
```

Output:
```
# treemd
## Features
### Phase 1: CLI Mode
### Phase 2: TUI Mode
## Installation
...
```

### Show heading tree

```bash
treemd --tree README.md
```

Output:
```
└─ # treemd
    ├─ ## Features
    │   ├─ ### Phase 1: CLI Mode
    │   └─ ### Phase 2: TUI Mode
    ├─ ## Installation
    ...
```

### Extract a section

```bash
treemd -s "Installation" README.md
```

Output:
```
## Installation

cargo install --path .
...
```

### Filter headings

```bash
treemd -l --filter "usage" README.md
```

### Show only specific heading level

```bash
treemd -l -L 2 README.md  # Only ## headings
```

### Count headings

```bash
treemd --count README.md
```

Output:
```
Heading counts:
  #: 1
  ##: 5
  ###: 6

Total: 12
```

### JSON output

```bash
treemd -l -o json README.md
```

## Development Roadmap

- [x] **Phase 1: Core Parser & CLI**
  - [x] Markdown parsing with pulldown-cmark
  - [x] Heading tree extraction
  - [x] CLI commands (`-l`, `--tree`, `-s`, `--count`)
  - [x] Filtering and level selection
  - [x] JSON output

- [x] **Phase 2: TUI Foundation**
  - [x] Ratatui dual-pane layout
  - [x] Outline view with collapsible tree
  - [x] Content view with markdown rendering
  - [x] Vim-style navigation keybindings
  - [x] Scrollbars for both panes
  - [x] Help overlay popup
  - [x] Rich styling and colors
  - [x] Status bar with progress indicators

- [x] **Phase 3: Enhanced Rendering**
  - [x] Syntax highlighting for code blocks (syntect)
  - [x] Inline formatting (bold, italic, inline code)
  - [x] Search/filter overlay with real-time filtering
  - [x] Enhanced markdown rendering pipeline
  - [x] Theme integration (Base16 Ocean Dark)

- [ ] **Phase 4: Advanced Features**
  - [ ] OFM callouts and wikilinks
  - [ ] OFM callouts with icons
  - [ ] Wikilink styling

- [ ] Phase 4: Search & UX Polish
  - [ ] Fuzzy search overlay
  - [ ] Multiple color themes
  - [ ] Configuration file support
  - [ ] Help overlay

- [ ] Phase 5: Advanced Features
  - [ ] Multiple file tabs
  - [ ] Link following (open [[linked]] files)
  - [ ] Watch mode (auto-reload on file change)

## Architecture

```
treemd/
├── src/
│   ├── main.rs          # CLI entry, mode selection
│   ├── parser/
│   │   ├── mod.rs       # Markdown parsing facade
│   │   └── document.rs  # Document model (headings tree)
│   ├── cli/
│   │   ├── mod.rs
│   │   └── commands.rs  # CLI argument definitions
│   └── tui/
│       ├── mod.rs
│       ├── app.rs       # TUI application state
│       ├── ui.rs        # Rendering logic
│       ├── syntax.rs    # Syntax highlighting
│       └── theme.rs     # Color theme configuration
└── Cargo.toml
```

## Why treemd?

- **Tree-based navigation**: Unlike `less` or `cat`, treemd understands document structure and lets you explore it like a file tree
- **Expandable outline**: Drill down into sections by collapsing/expanding headings—just like `tree` command
- **Interactive TUI**: Beautiful dual-pane interface with vim-style navigation and synchronized scrolling
- **CLI and TUI modes**: Use interactively for reading or in scripts for extraction/filtering
- **Fast**: Built in Rust, optimized binary with syntax highlighting
- **Rich rendering**: Color-coded headings, syntax-highlighted code blocks (50+ languages), styled inline formatting
- **User-friendly**: Scrollbars, help overlays, bookmarks, and fuzzy search

## Similar Tools

- `tree` - File tree explorer (inspiration for outline navigation)
- `glow` - Beautiful markdown rendering (presentation-focused, not interactive)
- `mdcat` - Markdown rendering to terminal (no navigation)
- `bat` - Syntax highlighting pager (not markdown-aware)
- `less` - Classic pager (no structure awareness)

treemd combines the best of these: **tree-based exploration** + interactive navigation + comfortable reading + CLI scriptability.

## License

MIT
