//! Component for displaying init success message.

use crate::ui::constants::{color_dim, color_label, color_success, ICON_PASS};
use crate::ui::utils::{box_line, divider, empty_box_line, term_width};
use std::path::Path;

/// Render the init success message.
///
/// Displays a boxed message with next steps after initialization.
///
/// # Arguments
/// * `path` - The path where .sentri.toml was created
///
/// # Returns
/// The formatted init output
pub fn render_init_success(path: &Path) -> String {
    let width = term_width();
    let mut output = String::new();

    output.push('\n');

    // Top border
    output.push_str(&format!(
        "{}─ Sentri Init ─{}\n",
        "╭",
        divider(width.saturating_sub(14))
    ));
    output.push_str("╮\n");

    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Success message
    let config_file = path.join(".sentri.toml");
    let success_line = format!(
        "{}  Created .sentri.toml at {}",
        color_success(ICON_PASS),
        config_file.display()
    );
    output.push_str(&format!("{}\n", box_line(&success_line, width)));

    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Next steps header
    output.push_str(&format!("{}\n", box_line("Next steps:", width)));

    // Steps
    let steps = vec![
        "Edit .sentri.toml to configure your chain and invariants",
        "Run: sentri check ./contracts",
        "Add to CI: sentri check ./contracts --fail-on high",
    ];

    for step in steps {
        let step_line = format!("{}  {}", color_dim("·"), step);
        output.push_str(&format!("{}\n", box_line(&step_line, width)));
    }

    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Docs link
    let docs_line = format!(
        "{}  https://docs.sentri.dev/quickstart",
        color_label("Docs:")
    );
    output.push_str(&format!("{}\n", box_line(&docs_line, width)));

    output.push_str(&format!("{}\n", empty_box_line(width)));

    // Bottom border
    output.push_str(&format!("{}{}╯\n", "╰", divider(width.saturating_sub(2))));

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_init_success_message() {
        let path = PathBuf::from("./contracts");
        let output = render_init_success(&path);

        assert!(output.contains("Sentri Init"));
        assert!(output.contains(".sentri.toml"));
        assert!(output.contains("configured"));
        assert!(output.contains("sentri check"));
    }
}
