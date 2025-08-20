//! Unicode utility functions

/// Convert a Unicode code point string (e.g., "U+4E00") to a char
pub fn code_point_to_char(code_point: &str) -> Option<char> {
    if code_point.starts_with("U+") {
        let hex = &code_point[2..];
        if let Ok(value) = u32::from_str_radix(hex, 16) {
            return char::from_u32(value);
        }
    }
    None
}

/// Convert a char to a Unicode code point string (e.g., "U+4E00")
pub fn char_to_code_point(ch: char) -> String {
    format!("U+{:04X}", ch as u32)
}

/// Check if a character is a CJK Unified Ideograph
pub fn is_cjk_unified_ideograph(ch: char) -> bool {
    let code_point = ch as u32;
    (0x4E00..=0x9FFF).contains(&code_point) || // CJK Unified Ideographs
    (0x3400..=0x4DBF).contains(&code_point) || // CJK Unified Ideographs Extension A
    (0x20000..=0x2A6DF).contains(&code_point) || // CJK Unified Ideographs Extension B
    (0x2A700..=0x2B73F).contains(&code_point) || // CJK Unified Ideographs Extension C
    (0x2B740..=0x2B81F).contains(&code_point) || // CJK Unified Ideographs Extension D
    (0x2B820..=0x2CEAF).contains(&code_point) || // CJK Unified Ideographs Extension E
    (0x2CEB0..=0x2EBEF).contains(&code_point) || // CJK Unified Ideographs Extension F
    (0x30000..=0x3134F).contains(&code_point) // CJK Unified Ideographs Extension G
}

/// Check if a character is a Kangxi radical
pub fn is_kangxi_radical(ch: char) -> bool {
    let code_point = ch as u32;
    (0x2F00..=0x2FDF).contains(&code_point)
}

/// Check if a character is a compatibility character
pub fn is_compatibility_character(ch: char) -> bool {
    let code_point = ch as u32;
    (0xF900..=0xFAFF).contains(&code_point) || // CJK Compatibility Ideographs
    (0x2F800..=0x2FA1F).contains(&code_point) || // CJK Compatibility Ideographs Supplement
    (0x3300..=0x33FF).contains(&code_point) || // CJK Compatibility
    (0xFE30..=0xFE4F).contains(&code_point) || // CJK Compatibility Forms
    (0xFF00..=0xFFEF).contains(&code_point) // Halfwidth and Fullwidth Forms
}

/// Check if a character is a Hiragana
pub fn is_hiragana(ch: char) -> bool {
    let code_point = ch as u32;
    (0x3040..=0x309F).contains(&code_point)
}

/// Check if a character is a Katakana
pub fn is_katakana(ch: char) -> bool {
    let code_point = ch as u32;
    (0x30A0..=0x30FF).contains(&code_point)
}

/// Check if a character is a Hangul
pub fn is_hangul(ch: char) -> bool {
    let code_point = ch as u32;
    (0xAC00..=0xD7AF).contains(&code_point) || // Hangul Syllables
    (0x1100..=0x11FF).contains(&code_point) || // Hangul Jamo
    (0x3130..=0x318F).contains(&code_point) // Hangul Compatibility Jamo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_point_conversion() {
        assert_eq!(code_point_to_char("U+4E00"), Some('一'));
        assert_eq!(code_point_to_char("U+9FFF"), Some('鿿'));
        assert_eq!(code_point_to_char("invalid"), None);

        assert_eq!(char_to_code_point('一'), "U+4E00");
        assert_eq!(char_to_code_point('A'), "U+0041");
    }

    #[test]
    fn test_cjk_detection() {
        assert!(is_cjk_unified_ideograph('一'));
        assert!(is_cjk_unified_ideograph('國'));
        assert!(!is_cjk_unified_ideograph('A'));
    }

    #[test]
    fn test_kangxi_detection() {
        assert!(is_kangxi_radical('⽅')); // U+2F45
        assert!(is_kangxi_radical('⾯')); // U+2FAF
        assert!(!is_kangxi_radical('一'));
    }

    #[test]
    fn test_script_detection() {
        assert!(is_hiragana('あ'));
        assert!(is_katakana('ア'));
        assert!(is_hangul('안'));
        assert!(!is_hiragana('一'));
    }
}
