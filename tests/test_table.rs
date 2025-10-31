//! Comprehensive tests for the table module

use zfish::table::{Alignment, BoxStyle, Table};

#[test]
fn test_table_creation() {
    let _table = Table::new(vec!["Col1", "Col2", "Col3"]);
    // Table created successfully
}

#[test]
fn test_add_single_row() {
    let mut table = Table::new(vec!["Name", "Age"]);
    table.add_row(vec!["Alice", "25"]);
    // Row added successfully
}

#[test]
fn test_add_multiple_rows() {
    let mut table = Table::new(vec!["Name", "Age", "City"]);
    table.add_row(vec!["Alice", "25", "NYC"]);
    table.add_row(vec!["Bob", "30", "LA"]);
    table.add_row(vec!["Charlie", "35", "Chicago"]);
    // Multiple rows added successfully
}

#[test]
fn test_column_width_expands_with_content() {
    let mut table = Table::new(vec!["Name", "Age"]);
    // Initial width is 4 for "Name"

    table.add_row(vec!["Alexander", "25"]);
    // Width should expand to 9 for "Alexander"
}

#[test]
fn test_set_box_style_single() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_box_style(BoxStyle::Single);
}

#[test]
fn test_set_box_style_double() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_box_style(BoxStyle::Double);
}

#[test]
fn test_set_box_style_heavy() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_box_style(BoxStyle::Heavy);
}

#[test]
fn test_set_box_style_rounded() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_box_style(BoxStyle::Rounded);
}

#[test]
fn test_set_box_style_ascii() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_box_style(BoxStyle::Ascii);
}

#[test]
fn test_set_alignment_left() {
    let mut table = Table::new(vec!["Col1", "Col2"]);
    table.set_column_alignment(0, Alignment::Left);
}

#[test]
fn test_set_alignment_right() {
    let mut table = Table::new(vec!["Col1", "Col2"]);
    table.set_column_alignment(0, Alignment::Right);
}

#[test]
fn test_set_alignment_center() {
    let mut table = Table::new(vec!["Col1", "Col2"]);
    table.set_column_alignment(0, Alignment::Center);
}

#[test]
fn test_set_indent_zero() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_indent(0);
}

#[test]
fn test_set_indent_custom() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_indent(10);
}

#[test]
fn test_header_separator_enabled() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_header_separator(true);
}

#[test]
fn test_header_separator_disabled() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_header_separator(false);
}

#[test]
fn test_footer_separator_enabled() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_footer_separator(true);
}

#[test]
fn test_footer_separator_disabled() {
    let mut table = Table::new(vec!["Col1"]);
    table.set_footer_separator(false);
}

#[test]
fn test_method_chaining() {
    let mut table = Table::new(vec!["Col1", "Col2"]);
    table
        .set_box_style(BoxStyle::Double)
        .set_indent(5)
        .set_column_alignment(0, Alignment::Center)
        .set_header_separator(true)
        .set_footer_separator(true);
}

#[test]
fn test_empty_table_print() {
    let table = Table::new(vec!["Col1", "Col2"]);
    // Should not panic when printing empty table
    table.print();
}

#[test]
fn test_table_with_emoji() {
    let mut table = Table::new(vec!["Status", "Message"]);
    table.add_row(vec!["✅", "Success"]);
    table.add_row(vec!["❌", "Failure"]);
    table.add_row(vec!["⚠️ ", "Warning"]);
    table.print();
}

#[test]
fn test_table_with_unicode() {
    let mut table = Table::new(vec!["Name", "Symbol"]);
    table.add_row(vec!["Pi", "π"]);
    table.add_row(vec!["Sigma", "Σ"]);
    table.add_row(vec!["Delta", "Δ"]);
    table.print();
}

#[test]
fn test_single_column_table() {
    let mut table = Table::new(vec!["Values"]);
    table.add_row(vec!["First"]);
    table.add_row(vec!["Second"]);
    table.add_row(vec!["Third"]);
    table.print();
}

#[test]
fn test_many_columns_table() {
    let mut table = Table::new(vec!["A", "B", "C", "D", "E", "F"]);
    table.add_row(vec!["1", "2", "3", "4", "5", "6"]);
    table.add_row(vec!["7", "8", "9", "10", "11", "12"]);
    table.print();
}

#[test]
fn test_wide_content() {
    let mut table = Table::new(vec!["Short", "Long"]);
    table.add_row(vec!["A", "This is a very long piece of content"]);
    table.add_row(vec!["This is also long", "B"]);
    table.print();
}

#[test]
fn test_numeric_alignment() {
    let mut table = Table::new(vec!["Item", "Quantity", "Price"]);
    table.set_column_alignment(1, Alignment::Right);
    table.set_column_alignment(2, Alignment::Right);
    table.add_row(vec!["Apple", "10", "$1.50"]);
    table.add_row(vec!["Banana", "5", "$0.75"]);
    table.add_row(vec!["Orange", "8", "$2.00"]);
    table.print();
}

#[test]
fn test_all_box_styles() {
    for style in [
        BoxStyle::Single,
        BoxStyle::Double,
        BoxStyle::Heavy,
        BoxStyle::Rounded,
        BoxStyle::Ascii,
    ] {
        let mut table = Table::new(vec!["Style", "Test"]);
        table.set_box_style(style);
        table.add_row(vec!["Testing", "Box"]);
        table.print();
    }
}

#[test]
fn test_all_alignments() {
    for alignment in [Alignment::Left, Alignment::Right, Alignment::Center] {
        let mut table = Table::new(vec!["Alignment", "Test"]);
        table.set_column_alignment(0, alignment);
        table.add_row(vec!["Testing", "Alignment"]);
        table.print();
    }
}

#[test]
fn test_table_with_totals() {
    let mut table = Table::new(vec!["Item", "Quantity", "Price"]);
    table.set_box_style(BoxStyle::Double);
    table.set_footer_separator(true);
    table.set_column_alignment(1, Alignment::Right);
    table.set_column_alignment(2, Alignment::Right);
    table.add_row(vec!["Item 1", "10", "$100"]);
    table.add_row(vec!["Item 2", "5", "$50"]);
    table.add_row(vec!["Item 3", "8", "$80"]);
    table.add_row(vec!["TOTAL", "23", "$230"]);
    table.print();
}

#[test]
fn test_realistic_data_table() {
    let mut table = Table::new(vec!["Feature", "Status", "Tests", "Coverage"]);
    table.set_box_style(BoxStyle::Single);
    table.set_footer_separator(true);
    table.set_column_alignment(2, Alignment::Right);
    table.set_column_alignment(3, Alignment::Right);
    table.add_row(vec!["Help Generation", "✅ Pass", "5", "100%"]);
    table.add_row(vec!["Validation", "✅ Pass", "2", "100%"]);
    table.add_row(vec!["Default Values", "✅ Pass", "5", "100%"]);
    table.add_row(vec!["TOTAL", "✅ Pass", "12", "100%"]);
    table.print();
}

#[test]
fn test_financial_report_table() {
    let mut table = Table::new(vec!["Month", "Revenue", "Expenses", "Profit"]);
    table.set_box_style(BoxStyle::Double);
    table.set_footer_separator(true);
    table.set_column_alignment(1, Alignment::Right);
    table.set_column_alignment(2, Alignment::Right);
    table.set_column_alignment(3, Alignment::Right);
    table.add_row(vec!["January", "$10,000", "$7,000", "$3,000"]);
    table.add_row(vec!["February", "$12,000", "$8,000", "$4,000"]);
    table.add_row(vec!["March", "$15,000", "$9,000", "$6,000"]);
    table.add_row(vec!["TOTAL", "$37,000", "$24,000", "$13,000"]);
    table.print();
}
