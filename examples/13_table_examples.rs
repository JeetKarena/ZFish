//! Table Examples - Comprehensive demonstration of table functionality
//!
//! This example shows all the features of the zfish table module:
//! - Different box styles (Single, Double, Heavy, Rounded, ASCII)
//! - Column alignment (Left, Right, Center)
//! - Header and footer separators
//! - Custom indentation
//! - Helper functions for boxes and separators

use zfish::style::Color;
use zfish::table::{Alignment, BoxStyle, Table, draw_box, draw_separator};

fn main() {
    println!();
    draw_box("✨ ZFISH TABLE EXAMPLES ✨", BoxStyle::Double, Color::Cyan);
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 1: Basic Table with Default Settings
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 1: Basic Table (Single-line style, left-aligned)");
    println!();

    let mut table1 = Table::new(vec!["Name", "Age", "City"]);
    table1.add_row(vec!["Alice", "25", "New York"]);
    table1.add_row(vec!["Bob", "30", "Los Angeles"]);
    table1.add_row(vec!["Charlie", "35", "Chicago"]);
    table1.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 2: Double-line Box Style
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 2: Double-line Box Style");
    println!();

    let mut table2 = Table::new(vec!["Product", "Price", "Stock"]);
    table2.set_box_style(BoxStyle::Double);
    table2.add_row(vec!["Laptop", "$999", "15"]);
    table2.add_row(vec!["Mouse", "$25", "150"]);
    table2.add_row(vec!["Keyboard", "$75", "80"]);
    table2.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 3: Heavy-line Box Style
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 3: Heavy-line Box Style");
    println!();

    let mut table3 = Table::new(vec!["Status", "Count", "Percentage"]);
    table3.set_box_style(BoxStyle::Heavy);
    table3.add_row(vec!["✅ Pass", "67", "100%"]);
    table3.add_row(vec!["⚠️  Warn", "0", "0%"]);
    table3.add_row(vec!["❌ Fail", "0", "0%"]);
    table3.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 4: Rounded Corners
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 4: Rounded Corner Style");
    println!();

    let mut table4 = Table::new(vec!["Feature", "Status"]);
    table4.set_box_style(BoxStyle::Rounded);
    table4.add_row(vec!["Help Generation", "✅ Implemented"]);
    table4.add_row(vec!["Validation", "✅ Implemented"]);
    table4.add_row(vec!["Default Values", "✅ Implemented"]);
    table4.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 5: ASCII-only (for compatibility)
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 5: ASCII-only Style (maximum compatibility)");
    println!();

    let mut table5 = Table::new(vec!["ID", "Name", "Value"]);
    table5.set_box_style(BoxStyle::Ascii);
    table5.add_row(vec!["1", "Alpha", "100"]);
    table5.add_row(vec!["2", "Beta", "200"]);
    table5.add_row(vec!["3", "Gamma", "300"]);
    table5.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 6: Column Alignment (Right and Center)
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 6: Column Alignment (Left, Right, Center)");
    println!();

    let mut table6 = Table::new(vec!["Item", "Quantity", "Price"]);
    table6.set_box_style(BoxStyle::Double);
    table6.set_column_alignment(0, Alignment::Left); // Item: left
    table6.set_column_alignment(1, Alignment::Center); // Quantity: center
    table6.set_column_alignment(2, Alignment::Right); // Price: right
    table6.add_row(vec!["Apple", "10", "$1.50"]);
    table6.add_row(vec!["Banana", "5", "$0.75"]);
    table6.add_row(vec!["Orange", "8", "$2.00"]);
    table6.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 7: Footer Separator (for totals/summaries)
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 7: Footer Separator for Totals");
    println!();

    let mut table7 = Table::new(vec!["Feature", "Tests", "Status"]);
    table7.set_box_style(BoxStyle::Single);
    table7.set_footer_separator(true); // Enable separator before last row
    table7.add_row(vec!["Help Generation", "5", "✅ Pass"]);
    table7.add_row(vec!["Validation", "2", "✅ Pass"]);
    table7.add_row(vec!["Default Values", "5", "✅ Pass"]);
    table7.add_row(vec!["TOTAL", "12", "✅ Pass"]); // Last row as footer
    table7.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 8: No Header Separator
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 8: Table Without Header Separator");
    println!();

    let mut table8 = Table::new(vec!["Type", "Description"]);
    table8.set_box_style(BoxStyle::Single);
    table8.set_header_separator(false); // Disable header separator
    table8.add_row(vec!["Info", "This is an informational message"]);
    table8.add_row(vec!["Warning", "This is a warning message"]);
    table8.add_row(vec!["Error", "This is an error message"]);
    table8.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 9: Custom Indentation
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 9: Custom Indentation Levels");
    println!();

    println!("Default indentation (3 spaces):");
    let mut table9a = Table::new(vec!["A", "B", "C"]);
    table9a.add_row(vec!["1", "2", "3"]);
    table9a.print();
    println!();

    println!("No indentation (0 spaces):");
    let mut table9b = Table::new(vec!["A", "B", "C"]);
    table9b.set_indent(0);
    table9b.add_row(vec!["1", "2", "3"]);
    table9b.print();
    println!();

    println!("Large indentation (10 spaces):");
    let mut table9c = Table::new(vec!["A", "B", "C"]);
    table9c.set_indent(10);
    table9c.add_row(vec!["1", "2", "3"]);
    table9c.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 10: Complex Table (All Features Combined)
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 10: Complex Table (All Features)");
    println!();

    let mut table10 = Table::new(vec!["Month", "Revenue", "Expenses", "Profit"]);
    table10.set_box_style(BoxStyle::Double);
    table10.set_column_alignment(0, Alignment::Left);
    table10.set_column_alignment(1, Alignment::Right);
    table10.set_column_alignment(2, Alignment::Right);
    table10.set_column_alignment(3, Alignment::Right);
    table10.set_footer_separator(true);
    table10.add_row(vec!["January", "$10,000", "$7,000", "$3,000"]);
    table10.add_row(vec!["February", "$12,000", "$8,000", "$4,000"]);
    table10.add_row(vec!["March", "$15,000", "$9,000", "$6,000"]);
    table10.add_row(vec!["TOTAL", "$37,000", "$24,000", "$13,000"]);
    table10.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 11: Helper Functions - draw_box()
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📦 EXAMPLE 11: Helper Function - draw_box()");
    println!();

    println!("Single-line box:");
    draw_box("This is a single-line box", BoxStyle::Single, Color::Cyan);
    println!();

    println!("Double-line box:");
    draw_box("This is a double-line box", BoxStyle::Double, Color::Green);
    println!();

    println!("Heavy-line box:");
    draw_box("This is a heavy-line box", BoxStyle::Heavy, Color::Magenta);
    println!();

    println!("Rounded box:");
    draw_box("This is a rounded box", BoxStyle::Rounded, Color::Yellow);
    println!();

    // ═══════════════════════════════════════════════════════════════
    // EXAMPLE 12: Wide Table with Auto-width Calculation
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    println!("📊 EXAMPLE 12: Auto-width Calculation");
    println!();

    let mut table12 = Table::new(vec!["Short", "Medium Length", "Very Very Long Header"]);
    table12.set_box_style(BoxStyle::Single);
    table12.add_row(vec!["A", "B", "C"]);
    table12.add_row(vec!["Very long content here", "Medium", "Short"]);
    table12.add_row(vec!["X", "This is also quite long", "Y"]);
    table12.print();
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Final Summary
    // ═══════════════════════════════════════════════════════════════

    draw_separator(70, "═", Color::Yellow);
    println!();
    draw_box(
        "✅ All table examples completed!",
        BoxStyle::Double,
        Color::Green,
    );
    println!();

    println!("{}Features demonstrated:", Color::White.paint("📋 "));
    println!("   ✅ 5 box styles (Single, Double, Heavy, Rounded, ASCII)");
    println!("   ✅ 3 alignment types (Left, Right, Center)");
    println!("   ✅ Header and footer separators");
    println!("   ✅ Custom indentation");
    println!("   ✅ Auto-width calculation");
    println!("   ✅ Helper functions (draw_box, draw_separator)");
    println!();
}
