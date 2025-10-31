use zfish::table::Table;

fn main() {
    // Debug: Check the actual bytes and chars in these strings
    let emojis = vec!["✅", "❌", "⚠️", "⚠"];

    println!("Emoji Analysis:");
    println!("{:-<60}", "");
    for emoji in &emojis {
        println!("Emoji: '{}' ", emoji);
        println!("  Byte length: {}", emoji.len());
        println!("  Char count: {}", emoji.chars().count());
        println!("  Chars: {:?}", emoji.chars().collect::<Vec<_>>());
        println!(
            "  Codepoints: {:?}",
            emoji
                .chars()
                .map(|c| format!("U+{:04X}", c as u32))
                .collect::<Vec<_>>()
        );
        println!();
    }

    println!("\n{:-<60}", "");
    println!("Creating test table with emojis:");
    println!();

    let mut table = Table::new(vec!["Emoji", "Name", "Bytes", "Chars"]);
    table.add_row(vec!["✅", "Check Mark", "4", "1"]);
    table.add_row(vec!["❌", "Cross Mark", "4", "1"]);
    table.add_row(vec!["⚠️", "Warning (VS)", "6", "2"]);
    table.add_row(vec!["⚠", "Warning (plain)", "3", "1"]);
    table.print();
}
