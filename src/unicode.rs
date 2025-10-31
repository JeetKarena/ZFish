//! Unicode display width utilities (zero-dependency)
//!
//! This module implements a pragmatic subset of Unicode rules to compute
//! terminal column width for strings. It handles:
//! - ASCII (1-cell)
//! - Combining marks, variation selectors, ZWJ/ZWNJ (0-cell)
//! - East Asian Wide/Fullwidth (2-cell)
//! - Emoji (2-cell) including skin tones and ZWJ sequences
//! - Regional indicator pairs (flags) (2-cell)
//! - Keycap sequences (2-cell)
//! - Text-default emoji that become emoji with VS-16 (plain=1, with VS-16=2)
//!
//! This logic purposefully has no external dependencies.

/// Public API: compute display width of a string in terminal cells.
pub fn display_width(s: &str) -> usize {
    let mut width = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let cp = c as u32;

        if is_zero_width(cp) {
            i += 1;
            continue;
        }

        // Text-default emoji special-case (plain 1, with VS-16 => 2)
        if is_text_default_emoji(cp) {
            if i + 1 < chars.len() && (chars[i + 1] as u32) == 0xFE0F {
                width += 2;
                i += 2;
            } else {
                width += 1;
                i += 1;
            }
            continue;
        }

        // Regional indicator pairs (flags)
        if is_regional_indicator(cp) && i + 1 < chars.len() {
            let next_cp = chars[i + 1] as u32;
            if is_regional_indicator(next_cp) {
                width += 2;
                i += 2;
                continue;
            }
        }

        // Emoji or East Asian wide
        if is_emoji_base(cp) || is_wide_character(cp) {
            width += 2;
            i += 1;
            // Consume trailing parts of the cluster
            while i < chars.len() {
                let next_cp = chars[i] as u32;
                if next_cp == 0x200D {
                    // ZWJ
                    i += 1; // consume ZWJ
                    if i < chars.len() {
                        i += 1; // consume the joined emoji base
                        continue;
                    }
                    break;
                }
                if is_zero_width(next_cp) || is_emoji_modifier(next_cp) {
                    i += 1;
                    continue;
                }
                break;
            }
            continue;
        }

        // Keycap sequences: [0-9#*] + FE0F + 20E3
        if ((0x30..=0x39).contains(&cp) || cp == 0x23 || cp == 0x2A) && i + 2 < chars.len() {
            let vs = chars[i + 1] as u32;
            let combining = chars[i + 2] as u32;
            if vs == 0xFE0F && combining == 0x20E3 {
                width += 2;
                i += 3;
                continue;
            }
        }

        // Regular character (1-cell)
        width += 1;
        i += 1;
        // Consume combining marks (0-cell)
        while i < chars.len() {
            let next_cp = chars[i] as u32;
            if is_combining_mark(next_cp) {
                i += 1;
            } else {
                break;
            }
        }
    }

    width
}

#[inline]
fn is_zero_width(cp: u32) -> bool {
    matches!(cp,
        0xFE00..=0xFE0F | // VS
        0xE0100..=0xE01EF | // VS Sup
        0x200D | // ZWJ
        0x200C | // ZWNJ
        0x200B | // ZW space
        0xFEFF | // ZW no-break space
        0x0000..=0x001F | 0x007F..=0x009F // controls
    )
}

#[inline]
fn is_combining_mark(cp: u32) -> bool {
    matches!(cp,
        0x0300..=0x036F | // Combining Diacritics
        0x1AB0..=0x1AFF | // Extended
        0x1DC0..=0x1DFF | // Supplement
        0x20D0..=0x20FF | // Symbols (inc 20E3)
        0xFE20..=0xFE2F   // Half Marks
    )
}

#[inline]
fn is_regional_indicator(cp: u32) -> bool {
    matches!(cp, 0x1F1E6..=0x1F1FF)
}

#[inline]
fn is_emoji_base(cp: u32) -> bool {
    matches!(cp,
        0x1F600..=0x1F64F | // Emoticons
        0x1F300..=0x1F5FF | // Symbols & Pictographs
        0x1F680..=0x1F6FF | // Transport & Map
        0x1F900..=0x1F9FF | // Supplemental Symbols & Pictographs
        0x1FA70..=0x1FAFF | // Symbols & Pictographs Extended-A
        0x1F100..=0x1F1FF | // Enclosed Alphanumeric Supplement
        0x1F200..=0x1F2FF | // Enclosed Ideographic Supplement
        0x2600..=0x26FF   | // Misc Symbols (many emoji)
        0x2700..=0x27BF   | // Dingbats
        0x1F0A0..=0x1F0FF | // Playing Cards
        0x1F000..=0x1F02F | // Mahjong Tiles
        0x1F030..=0x1F09F | // Domino Tiles
        0x25A0..=0x25FF   | // Geometric Shapes
        0x2300..=0x23FF     // Misc Technical
    )
}

#[inline]
fn is_emoji_modifier(cp: u32) -> bool {
    matches!(cp, 0x1F3FB..=0x1F3FF)
}

#[inline]
fn is_wide_character(cp: u32) -> bool {
    matches!(cp,
        0x4E00..=0x9FFF  | 0x3400..=0x4DBF | // CJK Unified Ideographs
        0x20000..=0x2EBEF | // CJK Ext B-G
        0xF900..=0xFAFF   | 0x2F800..=0x2FA1F | // CJK Compatibility
        0xAC00..=0xD7AF   | 0xA960..=0xA97F | 0xD7B0..=0xD7FF | // Hangul
        0xFF01..=0xFF60   | 0xFFE0..=0xFFE6 | 0xFF00..=0xFFEF | // Fullwidth forms
        0x3040..=0x309F   | 0x30A0..=0x30FF | 0x31F0..=0x31FF | // Hiragana/Katakana
        0x3100..=0x312F   | 0x31A0..=0x31BF | 0x3190..=0x319F | // Bopomofo/Kanbun
        0x3200..=0x32FF   | 0x3300..=0x33FF | // Enclosed/Compatibility
        0xA000..=0xA48F   | 0xA490..=0xA4CF   // Yi Syllables/Radicals
    )
}

#[inline]
fn is_text_default_emoji(cp: u32) -> bool {
    matches!(cp, 0x26A0 | 0x263A | 0x2709 | 0x260E | 0x270F)
}
