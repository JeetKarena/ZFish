//! Simple test to verify table alignment with emojis

use zfish::table::{BoxStyle, Table};

fn main() {
    println!("\nTest 1: Table with emojis in Status column");
    let mut table = Table::new(vec!["Feature", "Status", "Tests", "Coverage"]);
    table.set_box_style(BoxStyle::Single);
    table.add_row(vec!["Help Generation", "✅ Pass", "5/5", "100%"]);
    table.add_row(vec!["Required Validation", "✅ Pass", "2/2", "100%"]);
    table.add_row(vec!["Default Values", "✅ Pass", "5/5", "100%"]);
    table.print();

    println!("\n\nTest 2: Table with emojis in first column");
    let mut table2 = Table::new(vec!["Icon", "Name", "Value"]);
    table2.set_box_style(BoxStyle::Double);
    table2.add_row(vec!["✅", "Success", "100"]);
    table2.add_row(vec!["❌", "Failure", "0"]);
    table2.add_row(vec!["⚠️", "Warning", "5"]);
    table2.print();

    println!("\n\nTest 3: Table with mixed content");
    let mut table3 = Table::new(vec!["Status", "Count", "Percentage"]);
    table3.set_box_style(BoxStyle::Heavy);
    table3.add_row(vec!["✅ Pass", "67", "100%"]);
    table3.add_row(vec!["⚠️ Warn", "0", "0%"]);
    table3.add_row(vec!["❌ Fail", "0", "0%"]);
    table3.print();

    println!("\n✅ All alignment tests completed!");
}
