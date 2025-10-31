//! Table formatting and display functionality
//!
//! This module provides utilities for creating beautifully formatted tables
//! with automatic alignment, borders, and styling support.
//!
//! # Examples
//!
//! ```
//! use zfish::table::{Table, BoxStyle};
//! use zfish::style::Color;
//!
//! let mut table = Table::new(vec!["Name", "Age", "City"]);
//! table.add_row(vec!["Alice", "25", "NYC"]);
//! table.add_row(vec!["Bob", "30", "LA"]);
//! table.print();
//! ```
//!
//! # Manual Table Drawing
//!
//! If the automated `Table` API doesn't fit your specific needs (e.g., complex
//! multi-level layouts, custom cell spanning, dynamic row structures), you can
//! manually draw tables using Unicode box-drawing characters.
//!
//! See `examples/18_manual_table_drawing.rs` for complete manual drawing examples
//! including:
//! - Simple manual tables
//! - Colored manual tables
//! - Complex multi-level layouts
//! - Unicode-aware manual drawing with proper width calculation
//! - Hybrid approach mixing automated and manual techniques

use crate::style::{Color, Style};
use crate::unicode::display_width;

/// Box drawing styles for tables
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxStyle {
    /// Single-line box drawing characters: ┌─┐│└─┘
    Single,
    /// Double-line box drawing characters: ╔═╗║╚═╝
    Double,
    /// Heavy-line box drawing characters: ┏━┓┃┗━┛
    Heavy,
    /// Rounded corners with single lines: ╭─╮│╰─╯
    Rounded,
    /// ASCII-only characters: +-+|+-+
    Ascii,
}

impl BoxStyle {
    /// Get the box drawing characters for this style
    fn chars(&self) -> BoxChars {
        match self {
            BoxStyle::Single => BoxChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
                cross: '┼',
                t_down: '┬',
                t_up: '┴',
                t_right: '├',
                t_left: '┤',
            },
            BoxStyle::Double => BoxChars {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
                cross: '╬',
                t_down: '╦',
                t_up: '╩',
                t_right: '╠',
                t_left: '╣',
            },
            BoxStyle::Heavy => BoxChars {
                top_left: '┏',
                top_right: '┓',
                bottom_left: '┗',
                bottom_right: '┛',
                horizontal: '━',
                vertical: '┃',
                cross: '╋',
                t_down: '┳',
                t_up: '┻',
                t_right: '┣',
                t_left: '┫',
            },
            BoxStyle::Rounded => BoxChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
                cross: '┼',
                t_down: '┬',
                t_up: '┴',
                t_right: '├',
                t_left: '┤',
            },
            BoxStyle::Ascii => BoxChars {
                top_left: '+',
                top_right: '+',
                bottom_left: '+',
                bottom_right: '+',
                horizontal: '-',
                vertical: '|',
                cross: '+',
                t_down: '+',
                t_up: '+',
                t_right: '+',
                t_left: '+',
            },
        }
    }
}

/// Box drawing characters
#[derive(Debug, Clone, Copy)]
struct BoxChars {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
    cross: char,
    t_down: char,
    t_up: char,
    t_right: char,
    t_left: char,
}

/// Text alignment within table cells
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Left-aligned text
    Left,
    /// Right-aligned text
    Right,
    /// Center-aligned text
    Center,
}
// (Width logic moved to crate::unicode)

/// A formatted table with automatic column width calculation and borders
#[derive(Debug)]
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    col_widths: Vec<usize>,
    col_alignments: Vec<Alignment>,
    box_style: BoxStyle,
    indent: usize,
    has_header_separator: bool,
    has_footer_separator: bool,
}

impl Table {
    /// Creates a new table with the given headers
    ///
    /// # Examples
    ///
    /// ```
    /// use zfish::table::Table;
    ///
    /// let table = Table::new(vec!["Name", "Age", "City"]);
    /// ```
    pub fn new(headers: Vec<&str>) -> Self {
        let col_widths: Vec<usize> = headers.iter().map(|h| display_width(h)).collect();
        let headers: Vec<String> = headers.into_iter().map(|h| h.to_string()).collect();
        let col_count = col_widths.len();

        Table {
            headers,
            rows: Vec::new(),
            col_widths,
            col_alignments: vec![Alignment::Left; col_count],
            box_style: BoxStyle::Single,
            indent: 3,
            has_header_separator: true,
            has_footer_separator: false,
        }
    }

    /// Adds a row to the table
    ///
    /// # Examples
    ///
    /// ```
    /// use zfish::table::Table;
    ///
    /// let mut table = Table::new(vec!["Name", "Age"]);
    /// table.add_row(vec!["Alice", "25"]);
    /// table.add_row(vec!["Bob", "30"]);
    /// ```
    pub fn add_row(&mut self, row: Vec<&str>) {
        // Update column widths based on display width (not byte length)
        for (i, cell) in row.iter().enumerate() {
            if i < self.col_widths.len() {
                self.col_widths[i] = self.col_widths[i].max(display_width(cell));
            }
        }
        self.rows
            .push(row.into_iter().map(|s| s.to_string()).collect());
    }

    /// Sets the box drawing style for the table
    ///
    /// # Examples
    ///
    /// ```
    /// use zfish::table::{Table, BoxStyle};
    ///
    /// let mut table = Table::new(vec!["Name", "Age"]);
    /// table.set_box_style(BoxStyle::Double);
    /// ```
    pub fn set_box_style(&mut self, style: BoxStyle) -> &mut Self {
        self.box_style = style;
        self
    }

    /// Sets the indentation level (number of spaces before the table)
    ///
    /// # Examples
    ///
    /// ```
    /// use zfish::table::Table;
    ///
    /// let mut table = Table::new(vec!["Name", "Age"]);
    /// table.set_indent(5);
    /// ```
    pub fn set_indent(&mut self, indent: usize) -> &mut Self {
        self.indent = indent;
        self
    }

    /// Sets the alignment for a specific column
    ///
    /// # Examples
    ///
    /// ```
    /// use zfish::table::{Table, Alignment};
    ///
    /// let mut table = Table::new(vec!["Name", "Age", "Salary"]);
    /// table.set_column_alignment(1, Alignment::Right);  // Right-align "Age"
    /// table.set_column_alignment(2, Alignment::Right);  // Right-align "Salary"
    /// ```
    pub fn set_column_alignment(&mut self, col_index: usize, alignment: Alignment) -> &mut Self {
        if col_index < self.col_alignments.len() {
            self.col_alignments[col_index] = alignment;
        }
        self
    }

    /// Enables or disables the separator line after the header row
    pub fn set_header_separator(&mut self, enabled: bool) -> &mut Self {
        self.has_header_separator = enabled;
        self
    }

    /// Enables or disables the separator line before the last row
    pub fn set_footer_separator(&mut self, enabled: bool) -> &mut Self {
        self.has_footer_separator = enabled;
        self
    }

    /// Formats a cell according to the column's alignment
    fn format_cell(&self, text: &str, width: usize, alignment: Alignment) -> String {
        let text_width = display_width(text);
        let padding = width.saturating_sub(text_width);

        match alignment {
            Alignment::Left => {
                format!("{}{}", text, " ".repeat(padding))
            }
            Alignment::Right => {
                format!("{}{}", " ".repeat(padding), text)
            }
            Alignment::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
            }
        }
    }

    /// Prints a horizontal line with the given junction characters
    fn print_line(&self, left: char, _mid: char, right: char, junction: char) {
        let chars = self.box_style.chars();
        print!("{}", " ".repeat(self.indent));
        print!("{}", left);
        for (i, width) in self.col_widths.iter().enumerate() {
            print!("{}", chars.horizontal.to_string().repeat(width + 2));
            if i < self.col_widths.len() - 1 {
                print!("{}", junction);
            }
        }
        println!("{}", right);
    }

    /// Prints the table to stdout
    ///
    /// # Examples
    ///
    /// ```
    /// use zfish::table::Table;
    ///
    /// let mut table = Table::new(vec!["Name", "Age"]);
    /// table.add_row(vec!["Alice", "25"]);
    /// table.print();
    /// ```
    pub fn print(&self) {
        let chars = self.box_style.chars();

        // Print top border
        self.print_line(chars.top_left, chars.t_down, chars.top_right, chars.t_down);

        // Print headers
        print!("{}", " ".repeat(self.indent));
        print!("{}", chars.vertical);
        for (i, (header, width)) in self.headers.iter().zip(&self.col_widths).enumerate() {
            let formatted = self.format_cell(header, *width, self.col_alignments[i]);
            print!(" {} ", formatted);
            print!("{}", chars.vertical);
        }
        println!();

        // Print header separator
        if self.has_header_separator {
            self.print_line(chars.t_right, chars.cross, chars.t_left, chars.cross);
        }

        // Print rows
        for (idx, row) in self.rows.iter().enumerate() {
            print!("{}", " ".repeat(self.indent));
            print!("{}", chars.vertical);
            for (i, (cell, width)) in row.iter().zip(&self.col_widths).enumerate() {
                let formatted = self.format_cell(cell, *width, self.col_alignments[i]);
                print!(" {} ", formatted);
                print!("{}", chars.vertical);
            }
            println!();

            // Print footer separator before last row if enabled
            if self.has_footer_separator && idx == self.rows.len() - 2 {
                self.print_line(chars.t_right, chars.cross, chars.t_left, chars.cross);
            }
        }

        // Print bottom border
        self.print_line(
            chars.bottom_left,
            chars.t_up,
            chars.bottom_right,
            chars.t_up,
        );
    }
}

/// Helper function to draw a simple box around text
///
/// # Examples
///
/// ```
/// use zfish::table::{draw_box, BoxStyle};
/// use zfish::style::Color;
///
/// draw_box("Hello World", BoxStyle::Double, Color::Green);
/// ```
pub fn draw_box(text: &str, style: BoxStyle, color: Color) {
    let chars = style.chars();
    let width = display_width(text);

    let top = format!(
        "{}{}{}",
        chars.top_left,
        chars.horizontal.to_string().repeat(width + 2),
        chars.top_right
    );
    let middle = format!("{} {} {}", chars.vertical, text, chars.vertical);
    let bottom = format!(
        "{}{}{}",
        chars.bottom_left,
        chars.horizontal.to_string().repeat(width + 2),
        chars.bottom_right
    );

    println!("{}", color.paint(&top).style(Style::Bold));
    println!("{}", color.paint(&middle).style(Style::Bold));
    println!("{}", color.paint(&bottom).style(Style::Bold));
}

/// Helper function to draw a horizontal separator line
///
/// # Examples
///
/// ```
/// use zfish::table::draw_separator;
/// use zfish::style::Color;
///
/// draw_separator(50, "═", Color::Yellow);
/// ```
pub fn draw_separator(width: usize, char: &str, color: Color) {
    println!("{}", color.paint(char.repeat(width)).style(Style::Bold));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = Table::new(vec!["Col1", "Col2", "Col3"]);
        assert_eq!(table.headers.len(), 3);
        assert_eq!(table.col_widths.len(), 3);
        assert_eq!(table.rows.len(), 0);
    }

    #[test]
    fn test_add_row() {
        let mut table = Table::new(vec!["Name", "Age"]);
        table.add_row(vec!["Alice", "25"]);
        table.add_row(vec!["Bob", "30"]);
        assert_eq!(table.rows.len(), 2);
    }

    #[test]
    fn test_column_width_calculation() {
        let mut table = Table::new(vec!["Name", "Age"]);
        assert_eq!(table.col_widths[0], 4); // "Name".len()

        table.add_row(vec!["Alexander", "25"]);
        assert_eq!(table.col_widths[0], 9); // "Alexander".len()
    }

    #[test]
    fn test_box_style_setting() {
        let mut table = Table::new(vec!["Col1"]);
        table.set_box_style(BoxStyle::Double);
        assert_eq!(table.box_style, BoxStyle::Double);
    }

    #[test]
    fn test_alignment_setting() {
        let mut table = Table::new(vec!["Name", "Age", "Salary"]);
        table.set_column_alignment(1, Alignment::Right);
        assert_eq!(table.col_alignments[1], Alignment::Right);
    }

    #[test]
    fn test_cell_formatting_left() {
        let table = Table::new(vec!["Test"]);
        let result = table.format_cell("Hi", 5, Alignment::Left);
        assert_eq!(result, "Hi   ");
    }

    #[test]
    fn test_cell_formatting_right() {
        let table = Table::new(vec!["Test"]);
        let result = table.format_cell("Hi", 5, Alignment::Right);
        assert_eq!(result, "   Hi");
    }

    #[test]
    fn test_cell_formatting_center() {
        let table = Table::new(vec!["Test"]);
        let result = table.format_cell("Hi", 6, Alignment::Center);
        assert_eq!(result, "  Hi  ");
    }

    #[test]
    fn test_box_chars_single() {
        let chars = BoxStyle::Single.chars();
        assert_eq!(chars.top_left, '┌');
        assert_eq!(chars.horizontal, '─');
    }

    #[test]
    fn test_box_chars_double() {
        let chars = BoxStyle::Double.chars();
        assert_eq!(chars.top_left, '╔');
        assert_eq!(chars.horizontal, '═');
    }

    #[test]
    fn test_box_chars_ascii() {
        let chars = BoxStyle::Ascii.chars();
        assert_eq!(chars.top_left, '+');
        assert_eq!(chars.horizontal, '-');
    }
}
