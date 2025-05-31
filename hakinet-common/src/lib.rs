//! Common utilities and types for the Hakinet network tools suite

pub mod network;
pub mod output;
pub mod types;
pub mod utils;

pub use types::*;
pub use network::*;
pub use output::*;
pub use utils::*;

use colored::*;

/// Print the cute cat banner for Hakinet tools
pub fn print_cat_banner(tool_name: &str, description: &str) {
    // Define all the content lines
    let title_line = format!("🐱 Welcome to {}! 🐱", tool_name);
    let cat_line = "    /\\_/\\    Meow! Let's hunt some    ";
    let packet_line = "  ( o.o )   network packets! 📦     ";
    let face_line = "    > ^ <     packets together!      ";

    // Calculate display width accounting for emoji visual width
    let calculate_display_width = |s: &str| -> usize {
        s.chars().map(|c| {
            match c {
                // Common emoji characters take up 2 visual columns
                '🐱' | '📦' => 2,
                // Most other characters take 1 column
                _ => 1,
            }
        }).sum()
    };

    // Find the maximum display width needed
    let max_content_width = [
        calculate_display_width(&title_line),
        calculate_display_width(description),
        calculate_display_width(cat_line),
        calculate_display_width(packet_line),
        calculate_display_width(face_line),
    ].into_iter().max().unwrap_or(40);

    // Ensure minimum width and add padding
    let banner_width = (max_content_width + 8).max(50);

    // Create the border lines
    let top_border = format!("╭{}╮", "─".repeat(banner_width - 2));
    let bottom_border = format!("╰{}╯", "─".repeat(banner_width - 2));

    // Helper function to center text in the banner
    let center_text = |text: &str| -> String {
        let text_display_width = calculate_display_width(text);
        if text_display_width >= banner_width - 2 {
            format!("│{}│", text)
        } else {
            let padding = banner_width - 2 - text_display_width;
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            format!("│{}{text}{}│", " ".repeat(left_pad), " ".repeat(right_pad))
        }
    };

    // Create empty line
    let empty_line = format!("│{}│", " ".repeat(banner_width - 2));

    // Print the banner
    println!("{}", top_border.bright_cyan());
    println!("{}", empty_line.bright_cyan());
    println!("{}", center_text(&title_line).bright_cyan());
    println!("{}", center_text(description).bright_cyan());
    println!("{}", empty_line.bright_cyan());
    println!("{}", center_text(cat_line).bright_cyan());
    println!("{}", center_text(packet_line).bright_cyan());
    println!("{}", center_text(face_line).bright_cyan());
    println!("{}", empty_line.bright_cyan());
    println!("{}", bottom_border.bright_cyan());
}

/// Print working cat animation
pub fn print_cat_working(message: &str) {
    println!(
        "{}",
        format!(
            "
    🐱 {}
       /\\_/\\
      ( ^.^ ) *sniff sniff*
       > ^ <
    ",
            message
        )
        .bright_green()
    );
}

/// Print completion cat animation
pub fn print_cat_done(message: &str) {
    println!(
        "{}",
        format!(
            "
    🐱 {}
       /\\_/\\
      ( -.- ) *yawn*
       > ^ <
    ",
            message
        )
        .bright_yellow()
    );
}

/// Print error cat animation
pub fn print_cat_error(message: &str) {
    println!(
        "{}",
        format!(
            "
    🐱 {}
       /\\_/\\
      ( x.x ) *oops*
       > ^ <
    ",
            message
        )
        .bright_red()
    );
}