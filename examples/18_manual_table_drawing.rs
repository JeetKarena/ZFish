//! Manual Table Drawing - Fallback for Custom Scenarios
//!
//! This example shows how to manually draw tables using box-drawing characters
//! if the automated Table API doesn't meet your specific needs.
//!
//! Use cases for manual drawing:
//! - Complex multi-level tables
//! - Custom cell spanning
//! - Dynamic layouts that change per row
//! - Special formatting requirements

use zfish::style::{Color, Style};
use zfish::unicode::display_width;

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  Manual Table Drawing - Custom Layouts & Fallback       ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Example 1: Simple Manual Table
    example_1_simple_manual();

    // Example 2: Manual Table with Colors
    example_2_colored_manual();

    // Example 3: Complex Multi-level Table
    example_3_complex_layout();

    // Example 4: Manual Table with Unicode/Emoji
    example_4_unicode_aware();

    // Example 5: Mixing Automated and Manual
    example_5_hybrid_approach();

    // Example 6: Character Reference
    print_box_drawing_reference();

    println!("\n✅ All manual table drawing examples completed!\n");
}

/// Example 1: Basic manual table drawing
fn example_1_simple_manual() {
    println!("📊 Example 1: Simple Manual Table");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Top border
    println!("┌─────────────┬────────┬──────────┐");

    // Header
    println!("│ Product     │ Price  │ Stock    │");

    // Header separator
    println!("├─────────────┼────────┼──────────┤");

    // Data rows
    println!("│ Laptop      │ $999   │ 15       │");
    println!("│ Mouse       │ $25    │ 150      │");
    println!("│ Keyboard    │ $75    │ 80       │");

    // Bottom border
    println!("└─────────────┴────────┴──────────┘");
    println!();
}

/// Example 2: Manual table with colors
fn example_2_colored_manual() {
    println!("📊 Example 2: Colored Manual Table");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Top border (yellow)
    println!(
        "{}",
        Color::Yellow.paint("╔═══════════════╦═══════╦══════════╗")
    );

    // Header (cyan)
    println!(
        "{}",
        Color::Cyan
            .paint("║ Service       ║ Status║ Uptime   ║")
            .style(Style::Bold)
    );

    // Header separator (yellow)
    println!(
        "{}",
        Color::Yellow.paint("╠═══════════════╬═══════╬══════════╣")
    );

    // Data rows with colored status
    print!("{}", Color::White.paint("║ Web Server    ║ "));
    print!("{}", Color::Green.paint("✅ Up "));
    println!("{}", Color::White.paint("║ 99.9%    ║"));

    print!("{}", Color::White.paint("║ Database      ║ "));
    print!("{}", Color::Green.paint("✅ Up "));
    println!("{}", Color::White.paint("║ 100%     ║"));

    print!("{}", Color::White.paint("║ Cache         ║ "));
    print!("{}", Color::Red.paint("❌ Down"));
    println!("{}", Color::White.paint("║ 95.2%    ║"));

    // Bottom border (yellow)
    println!(
        "{}",
        Color::Yellow.paint("╚═══════════════╩═══════╩══════════╝")
    );
    println!();
}

/// Example 3: Complex multi-level table
fn example_3_complex_layout() {
    println!("📊 Example 3: Complex Multi-Level Layout");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("╔═══════════════════════════════════════════════════╗");
    println!("║              Q4 2025 Report                       ║");
    println!("╠════════════════╦══════════════╦═══════════════════╣");
    println!("║ Department     ║ Revenue      ║ Growth            ║");
    println!("╠════════════════╬══════════════╬═══════════════════╣");

    // Engineering section with sub-rows
    println!("║ Engineering    ║              ║                   ║");
    println!("║   ├─ Backend  ║ $1.2M        ║ ↗ +15%            ║");
    println!("║   ├─ Frontend ║ $800K        ║ ↗ +12%            ║");
    println!("║   └─ DevOps   ║ $400K        ║ ↗ +8%             ║");
    println!("╟────────────────╫──────────────╫───────────────────╢");

    // Sales section
    println!("║ Sales          ║              ║                   ║");
    println!("║   ├─ B2B      ║ $2.5M        ║ ↗ +25%            ║");
    println!("║   └─ B2C      ║ $1.8M        ║ ↗ +18%            ║");
    println!("╟────────────────╫──────────────╫───────────────────╢");

    // Total
    println!("║ TOTAL          ║ $6.7M        ║ ↗ +17%            ║");
    println!("╚════════════════╩══════════════╩═══════════════════╝");
    println!();
}

/// Example 4: Unicode-aware manual table
fn example_4_unicode_aware() {
    println!("📊 Example 4: Unicode-Aware Manual Drawing");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Helper function to pad text based on display width
    fn pad_text(text: &str, total_width: usize) -> String {
        let text_width = display_width(text);
        let padding = total_width.saturating_sub(text_width);
        format!("{}{}", text, " ".repeat(padding))
    }

    println!("┌──────────────────┬────────────┬──────────┐");
    println!("│ User             │ Status     │ Country  │");
    println!("├──────────────────┼────────────┼──────────┤");

    // Row 1: Emoji and Unicode
    print!("│ ");
    print!("{}", pad_text("田中さん", 16));
    print!(" │ ");
    print!("{}", pad_text("✅ Online", 10));
    print!(" │ ");
    print!("{}", pad_text("🇯🇵", 8));
    println!(" │");

    // Row 2: More emoji
    print!("│ ");
    print!("{}", pad_text("José García", 16));
    print!(" │ ");
    print!("{}", pad_text("❌ Offline", 10));
    print!(" │ ");
    print!("{}", pad_text("🇪🇸", 8));
    println!(" │");

    // Row 3: Skin tone modifiers
    print!("│ ");
    print!("{}", pad_text("Alex 👋🏽", 16));
    print!(" │ ");
    print!("{}", pad_text("⚠️ Away", 10));
    print!(" │ ");
    print!("{}", pad_text("🇺🇸", 8));
    println!(" │");

    println!("└──────────────────┴────────────┴──────────┘");
    println!();
}

/// Example 5: Hybrid approach - mixing automated and manual
fn example_5_hybrid_approach() {
    use zfish::table::{BoxStyle, Table, draw_box};

    println!("📊 Example 5: Hybrid Approach (Automated + Manual)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Use automated table for standard data
    println!("Standard Table (Automated):");
    let mut table = Table::new(vec!["ID", "Name", "Value"]);
    table.set_box_style(BoxStyle::Single);
    table.add_row(vec!["1", "Alpha", "100"]);
    table.add_row(vec!["2", "Beta", "200"]);
    table.print();

    println!();

    // Add manual section for special formatting
    println!("Custom Section (Manual):");
    println!("┌────────────────────────────────────────────┐");
    println!("│  📝 Notes:                                 │");
    println!("│     • Data validated on 2025-10-31         │");
    println!("│     • Source: Production Database          │");
    println!("│     • Next update: 2025-11-01 00:00 UTC    │");
    println!("└────────────────────────────────────────────┘");

    println!();

    // Use helper function for special boxes
    println!("Helper Function (draw_box):");
    draw_box(
        "✨ Summary: All systems operational!",
        BoxStyle::Double,
        Color::Green,
    );

    println!();
}

/// Example 6: Box drawing character reference
fn print_box_drawing_reference() {
    println!("\n📊 Example 6: Box Drawing Character Reference");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n📐 Box Drawing Character Reference");
    println!("══════════════════════════════════════════════════");

    println!("\nSingle Line:");
    println!("  Corners: ┌ ┐ └ ┘");
    println!("  Lines:   ─ │");
    println!("  T-joints: ┬ ┴ ├ ┤");
    println!("  Cross:   ┼");

    println!("\nDouble Line:");
    println!("  Corners: ╔ ╗ ╚ ╝");
    println!("  Lines:   ═ ║");
    println!("  T-joints: ╦ ╩ ╠ ╣");
    println!("  Cross:   ╬");

    println!("\nHeavy Line:");
    println!("  Corners: ┏ ┓ ┗ ┛");
    println!("  Lines:   ━ ┃");
    println!("  T-joints: ┳ ┻ ┣ ┫");
    println!("  Cross:   ╋");

    println!("\nRounded:");
    println!("  Corners: ╭ ╮ ╰ ╯");
    println!("  Lines:   ─ │");

    println!("\nASCII Compatible:");
    println!("  Corners: + + + +");
    println!("  Lines:   - |");
    println!("  T-joints: + + + +");
    println!("  Cross:   +");
}
