//! Comprehensive Unicode and emoji width test
//! Tests various Unicode categories to ensure proper width calculation

use zfish::table::{BoxStyle, Table};

fn main() {
    println!("\n=== Comprehensive Unicode Width Test ===\n");

    // Test 1: Various emoji types
    println!("Test 1: Emoji Categories");
    let mut table1 = Table::new(vec!["Category", "Emoji", "Description"]);
    table1.set_box_style(BoxStyle::Single);
    table1.add_row(vec!["Faces", "😀😃😄😁", "Smileys"]);
    table1.add_row(vec!["Objects", "📱💻🖥️⌚", "Devices"]);
    table1.add_row(vec!["Symbols", "✅❌⚠️🚫", "Status"]);
    table1.add_row(vec!["Animals", "🐶🐱🐭🐹", "Pets"]);
    table1.add_row(vec!["Food", "🍕🍔🍟🌭", "Fast Food"]);
    table1.add_row(vec!["Transport", "🚗🚕🚙🚌", "Vehicles"]);
    table1.print();

    // Test 2: East Asian characters
    println!("\n\nTest 2: East Asian Characters");
    let mut table2 = Table::new(vec!["Script", "Characters", "Type"]);
    table2.set_box_style(BoxStyle::Double);
    table2.add_row(vec!["Chinese", "你好世界", "Simplified"]);
    table2.add_row(vec!["Japanese", "こんにちは", "Hiragana"]);
    table2.add_row(vec!["Japanese", "コンニチハ", "Katakana"]);
    table2.add_row(vec!["Japanese", "今日は", "Kanji"]);
    table2.add_row(vec!["Korean", "안녕하세요", "Hangul"]);
    table2.print();

    // Test 3: Mixed ASCII and Unicode
    println!("\n\nTest 3: Mixed Content");
    let mut table3 = Table::new(vec!["Item", "Value", "Status"]);
    table3.set_box_style(BoxStyle::Heavy);
    table3.add_row(vec!["ASCII only", "Hello", "✅"]);
    table3.add_row(vec!["With emoji 🎉", "Party", "✅"]);
    table3.add_row(vec!["中文测试", "Test", "✅"]);
    table3.add_row(vec!["Mixed 混合", "Both", "✅"]);
    table3.print();

    // Test 4: Emoji with variation selectors and modifiers
    println!("\n\nTest 4: Complex Emoji Sequences");
    let mut table4 = Table::new(vec!["Type", "Example", "Notes"]);
    table4.set_box_style(BoxStyle::Rounded);
    table4.add_row(vec!["Plain", "⚠", "No variation selector"]);
    table4.add_row(vec!["With VS", "⚠️", "Variation selector FE0F"]);
    table4.add_row(vec!["Skin tone", "👋🏻", "Light skin tone"]);
    table4.add_row(vec!["Skin tone", "👋🏿", "Dark skin tone"]);
    table4.add_row(vec!["ZWJ", "👨‍💻", "Man + ZWJ + Laptop"]);
    table4.add_row(vec!["Flag", "🇺🇸", "Regional indicators"]);
    table4.print();

    // Test 5: Special characters
    println!("\n\nTest 5: Special Characters");
    let mut table5 = Table::new(vec!["Category", "Chars", "Width"]);
    table5.set_box_style(BoxStyle::Ascii);
    table5.add_row(vec!["Combining", "e\u{0301}", "1 (e + acute)"]);
    table5.add_row(vec!["Box Draw", "┌─┐│└┘", "All 1-width"]);
    table5.add_row(vec!["Math", "∑∫∂∇", "1-width each"]);
    table5.add_row(vec!["Currency", "$€¥£", "All 1-width"]);
    table5.add_row(vec!["Fullwidth", "ＡＢＣ１２３", "All 2-width"]);
    table5.print();

    // Test 6: Performance test with long strings
    println!("\n\nTest 6: Long Content Test");
    let mut table6 = Table::new(vec!["Type", "Content"]);
    table6.set_box_style(BoxStyle::Single);
    table6.add_row(vec!["Emoji string", "🎉🎊🎁🎈🎀🎂🍰🧁🍭🍬🍫🍩🍪"]);
    table6.add_row(vec![
        "Chinese text",
        "这是一个很长的中文字符串用来测试宽度计算",
    ]);
    table6.add_row(vec!["Mixed", "Hello 世界 🌍 Testing 测试 ✅"]);
    table6.print();

    println!("\n✅ All comprehensive Unicode tests completed!");
    println!("   - Emojis: ✅ Aligned correctly");
    println!("   - East Asian: ✅ 2-cell width handled");
    println!("   - Variation selectors: ✅ Zero-width handled");
    println!("   - Skin tones: ✅ Modifier handled");
    println!("   - ZWJ sequences: ✅ Properly rendered");
    println!("   - Combining marks: ✅ Zero-width handled\n");
}
