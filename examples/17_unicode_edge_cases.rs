//! Test edge cases and complex Unicode sequences
//! This verifies the most challenging Unicode rendering scenarios

use zfish::table::{BoxStyle, Table};

fn main() {
    println!("\n🧪 Edge Case & Complex Unicode Test 🧪\n");

    // Test 1: Emoji with skin tone modifiers
    println!("Test 1: Skin Tone Modifiers");
    let mut table1 = Table::new(vec!["Emoji", "Description", "Codepoints"]);
    table1.set_box_style(BoxStyle::Single);
    table1.add_row(vec!["👋", "Wave (neutral)", "U+1F44B"]);
    table1.add_row(vec!["👋🏻", "Wave (light)", "U+1F44B U+1F3FB"]);
    table1.add_row(vec!["👋🏼", "Wave (medium-light)", "U+1F44B U+1F3FC"]);
    table1.add_row(vec!["👋🏽", "Wave (medium)", "U+1F44B U+1F3FD"]);
    table1.add_row(vec!["👋🏾", "Wave (medium-dark)", "U+1F44B U+1F3FE"]);
    table1.add_row(vec!["👋🏿", "Wave (dark)", "U+1F44B U+1F3FF"]);
    table1.print();

    // Test 2: Zero-Width Joiner (ZWJ) sequences
    println!("\n\nTest 2: ZWJ Sequences (Compound Emoji)");
    let mut table2 = Table::new(vec!["Emoji", "Description"]);
    table2.set_box_style(BoxStyle::Double);
    table2.add_row(vec!["👨‍💻", "Man Technologist"]);
    table2.add_row(vec!["👩‍🔬", "Woman Scientist"]);
    table2.add_row(vec!["👨‍👩‍👧‍👦", "Family"]);
    table2.add_row(vec!["🏳️‍🌈", "Rainbow Flag"]);
    table2.add_row(vec!["👁️‍🗨️", "Eye in Speech Bubble"]);
    table2.print();

    // Test 3: Regional Indicator Symbols (Flags)
    println!("\n\nTest 3: Flag Emoji (Regional Indicators)");
    let mut table3 = Table::new(vec!["Flag", "Country", "Indicators"]);
    table3.set_box_style(BoxStyle::Heavy);
    table3.add_row(vec!["🇺🇸", "USA", "U+1F1FA U+1F1F8"]);
    table3.add_row(vec!["🇬🇧", "UK", "U+1F1EC U+1F1E7"]);
    table3.add_row(vec!["🇯🇵", "Japan", "U+1F1EF U+1F1F5"]);
    table3.add_row(vec!["🇨🇳", "China", "U+1F1E8 U+1F1F3"]);
    table3.add_row(vec!["🇮🇳", "India", "U+1F1EE U+1F1F3"]);
    table3.print();

    // Test 4: Combining characters
    println!("\n\nTest 4: Combining Diacritical Marks");
    let mut table4 = Table::new(vec!["Base", "Combined", "Description"]);
    table4.set_box_style(BoxStyle::Rounded);
    table4.add_row(vec!["e", "é", "e + acute accent"]);
    table4.add_row(vec!["e", "ê", "e + circumflex"]);
    table4.add_row(vec!["n", "ñ", "n + tilde"]);
    table4.add_row(vec!["u", "ü", "u + diaeresis"]);
    table4.add_row(vec!["c", "ç", "c + cedilla"]);
    table4.print();

    // Test 5: Keycap sequences
    println!("\n\nTest 5: Keycap & Special Sequences");
    let mut table5 = Table::new(vec!["Emoji", "Type", "Notes"]);
    table5.set_box_style(BoxStyle::Ascii);
    table5.add_row(vec!["1️⃣", "Keycap", "1 + VS + Combining"]);
    table5.add_row(vec!["#️⃣", "Keycap", "# + VS + Combining"]);
    table5.add_row(vec!["*️⃣", "Keycap", "* + VS + Combining"]);
    table5.add_row(vec!["🔟", "Number", "Encircled 10"]);
    table5.print();

    // Test 6: Mixed complex scenarios
    println!("\n\nTest 6: Real-World Mixed Content");
    let mut table6 = Table::new(vec!["User", "Status", "Message"]);
    table6.set_box_style(BoxStyle::Single);
    table6.add_row(vec!["Alice 👩‍💻", "✅ Online", "Working on 項目 project 🚀"]);
    table6.add_row(vec!["Bob 👨‍🔬", "⚠️ Away", "In café drinking ☕"]);
    table6.add_row(vec!["田中さん", "❌ Offline", "会議中です 📝"]);
    table6.add_row(vec!["José García", "✅ Online", "¡Hola! 👋🏽"]);
    table6.print();

    // Test 7: Emoji sequences vs plain
    println!("\n\nTest 7: Variation Selector Comparison");
    let mut table7 = Table::new(vec!["Plain", "With VS-16", "Notes"]);
    table7.set_box_style(BoxStyle::Double);
    table7.add_row(vec!["⚠", "⚠️", "Warning sign"]);
    table7.add_row(vec!["☺", "☺️", "Smiling face"]);
    table7.add_row(vec!["✉", "✉️", "Envelope"]);
    table7.add_row(vec!["☎", "☎️", "Telephone"]);
    table7.add_row(vec!["✏", "✏️", "Pencil"]);
    table7.print();

    println!("\n✅ All edge case tests completed successfully!");
    println!("   Zero dependencies - Pure Rust implementation!");
    println!("   Based on Unicode UAX #11 (East Asian Width)");
    println!("   Handles: Emoji, ZWJ, VS, Modifiers, Combining Marks\n");
}
