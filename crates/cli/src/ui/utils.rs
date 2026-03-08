//! Terminal utilities for handling width detection, text wrapping, and formatting.

use std::cmp::min;
use std::io::IsTerminal;

/// Detect the current terminal width.
///
/// Uses `terminal_size` crate to detect the terminal width.
/// Defaults to 80 columns if detection fails or is not available.
///
/// # Returns
/// The terminal width in columns, minimum 60, maximum 200.
pub fn term_width() -> usize {
    match terminal_size::terminal_size() {
        Some((terminal_size::Width(w), _)) => {
            let w = w as usize;
            // Clamp between reasonable bounds
            w.clamp(60, 200)
        }
        None => 80,
    }
}

/// Check if stdout is a TTY (interactive terminal).
///
/// # Returns
/// `true` if stdout is a TTY, `false` otherwise.
pub fn is_tty() -> bool {
    std::io::stdout().is_terminal()
}

/// Check if stderr is a TTY.
///
/// Useful for knowing if we should write spinner output to stderr.
///
/// # Returns
/// `true` if stderr is a TTY, `false` otherwise.
#[allow(dead_code)]
pub fn is_tty_stderr() -> bool {
    std::io::stderr().is_terminal()
}

/// Produce a horizontal divider line.
///
/// Creates a line of `─` characters padded to the given width.
///
/// # Arguments
/// * `width` - The desired width of the divider
///
/// # Examples
/// ```ignore
/// let line = divider(80);
/// assert_eq!(line.len(), 80);
/// ```
pub fn divider(width: usize) -> String {
    "─".repeat(width)
}

/// Right-pad a string to a specified width with spaces.
///
/// If the string is longer than the desired width, returns the string unchanged.
///
/// # Arguments
/// * `s` - The string to pad
/// * `width` - The target width
///
/// # Examples
/// ```ignore
/// assert_eq!(pad_right("hi", 5), "hi   ");
/// ```
pub fn pad_right(s: &str, width: usize) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - s.len()))
    }
}

/// Left-pad a string to a specified width with spaces.
///
/// If the string is longer than the desired width, returns the string unchanged.
///
/// # Arguments
/// * `s` - The string to pad
/// * `width` - The target width
#[allow(dead_code)]
pub fn pad_left(s: &str, width: usize) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        format!("{}{}", " ".repeat(width - s.len()), s)
    }
}

/// Wrap text to fit within a maximum width.
///
/// Breaks text at word boundaries to fit within the specified width.
/// Does not hyphenate long words; they will exceed the boundary if needed.
///
/// # Arguments
/// * `text` - The text to wrap
/// * `width` - The maximum width in characters
///
/// # Returns
/// A vector of wrapped lines
///
/// # Examples
/// ```ignore
/// let lines = wrap_text("Hello world this is a test", 10);
/// assert_eq!(lines.len(), 3);
/// ```
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if text.is_empty() || width == 0 {
        return vec![];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            // First word on line
            if word.len() <= width {
                current_line = word.to_string();
            } else {
                // Word is too long for line, put it on its own line
                lines.push(word.to_string());
            }
        } else if current_line.len() + 1 + word.len() <= width {
            // Word fits on current line
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            // Word doesn't fit, start new line
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Create a box line for a violation or summary panel.
///
/// Pads content with spaces to fill the interior width, then adds the border characters.
///
/// # Arguments
/// * `content` - The content to put in the box
/// * `width` - The total box width (including borders and padding)
///
/// # Returns
/// A line with format: `│ content                           │`
pub fn box_line(content: &str, width: usize) -> String {
    let interior_width = width.saturating_sub(4); // Account for `│ ` and ` │`
    let padded = pad_right(content, interior_width);
    format!("│ {} │", padded)
}

/// Create an empty box line (just borders with padding).
///
/// # Arguments
/// * `width` - The total box width
pub fn empty_box_line(width: usize) -> String {
    let interior_width = width.saturating_sub(4);
    format!("│{}│", " ".repeat(interior_width + 2))
}

/// Create a border line with optional separator in middle.
///
/// # Arguments
/// * `left_char` - Left corner character
/// * `right_char` - Right corner character
/// * `fill_char` - Character to fill the middle (usually `─`)
/// * `width` - Total width
/// * `middle_char` - Optional character for middle (use `None` for no middle)
/// * `middle_pos` - Position of the middle character (from left)
#[allow(dead_code)]
pub fn border_line(
    left_char: &str,
    right_char: &str,
    fill_char: &str,
    width: usize,
    middle_char: Option<&str>,
    middle_pos: Option<usize>,
) -> String {
    let mut line = String::new();
    line.push_str(left_char);

    let fill_width = width.saturating_sub(2); // Account for left + right

    if let (Some(mid_char), Some(pos)) = (middle_char, middle_pos) {
        let left_fill = pos.saturating_sub(1);
        let right_fill = fill_width.saturating_sub(left_fill + mid_char.len());

        line.push_str(&fill_char.repeat(left_fill));
        line.push_str(mid_char);
        line.push_str(&fill_char.repeat(right_fill));
    } else {
        line.push_str(&fill_char.repeat(fill_width));
    }

    line.push_str(right_char);
    line
}

/// Create a severity bar for the summary dashboard.
///
/// Creates a bar like `██████░░░░` to show proportional count.
///
/// # Arguments
/// * `count` - The current count
/// * `max` - The maximum count for scaling
/// * `filled_char` - Character for filled portion
/// * `empty_char` - Character for empty portion
/// * `bar_width` - Width of the bar (default 10)
pub fn severity_bar(count: usize, max: usize, filled_char: &str, empty_char: &str) -> String {
    let bar_width = 10;
    if max == 0 {
        return empty_char.repeat(bar_width);
    }

    let filled = (count * bar_width) / max;
    let filled = min(filled, bar_width);
    let empty = bar_width - filled;

    format!("{}{}", filled_char.repeat(filled), empty_char.repeat(empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_empty() {
        let result = wrap_text("", 10);
        assert!(result.is_empty());
    }

    #[test]
    fn test_wrap_text_single_word() {
        let result = wrap_text("hello", 10);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "hello");
    }

    #[test]
    fn test_wrap_text_multiple_words() {
        let result = wrap_text("hello world test", 10);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "hello");
        assert_eq!(result[1], "world test");
    }

    #[test]
    fn test_wrap_text_exact_boundary() {
        let result = wrap_text("hello world", 11);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "hello world");
    }

    #[test]
    fn test_wrap_text_long_word() {
        let result = wrap_text("supercalifragilisticexpialidocious short", 10);
        assert_eq!(result[0], "supercalifragilisticexpialidocious");
        assert_eq!(result[1], "short");
    }

    #[test]
    fn test_pad_right() {
        assert_eq!(pad_right("hi", 5), "hi   ");
        assert_eq!(pad_right("hello", 5), "hello");
        assert_eq!(pad_right("hello world", 5), "hello world");
    }

    #[test]
    fn test_pad_left() {
        assert_eq!(pad_left("hi", 5), "   hi");
        assert_eq!(pad_left("hello", 5), "hello");
    }

    #[test]
    fn test_divider() {
        let div = divider(5);
        assert_eq!(div.chars().count(), 5);
        assert_eq!(div, "─────");
    }

    #[test]
    fn test_severity_bar_zero_max() {
        let bar = severity_bar(0, 0, "█", "░");
        assert_eq!(bar.chars().count(), 10);
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn test_severity_bar_proportional() {
        let bar = severity_bar(5, 10, "█", "░");
        assert_eq!(bar, "█████░░░░░");
    }

    #[test]
    fn test_severity_bar_max() {
        let bar = severity_bar(10, 10, "█", "░");
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn test_box_line() {
        let line = box_line("test", 20);
        assert!(line.starts_with("│"));
        assert!(line.ends_with("│"));
        assert!(line.contains("test"));
    }

    #[test]
    fn test_empty_box_line() {
        let line = empty_box_line(20);
        assert!(line.starts_with("│"));
        assert!(line.ends_with("│"));
        assert_eq!(line.chars().count(), 20);
    }
}
