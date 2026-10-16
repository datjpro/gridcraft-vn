//! Vietnamese language utilities: collation, name comparison, case normalization, and TCVN3 conversion.

use std::cmp::Ordering;

/// Primary (base letter) and secondary (tone) weights for Vietnamese dictionary sorting.
///
/// Base letter ordering:
/// A < Ă < Â < B < C < D < Đ < E < Ê < F < G < H < I < J < K < L < M < N < O < Ô < Ơ < P < Q < R < S < T < U < Ư < V < W < X < Y < Z
///
/// Tone ordering:
/// Không dấu (0) < Huyền (1) < Hỏi (2) < Ngã (3) < Sắc (4) < Nặng (5)
pub fn vietnamese_char_weights(c: char) -> (u16, u8) {
    let lower = match c.to_lowercase().next() {
        Some(l) => l,
        None => c,
    };

    match lower {
        // A
        'a' => (10, 0),
        'à' => (10, 1),
        'ả' => (10, 2),
        'ã' => (10, 3),
        'á' => (10, 4),
        'ạ' => (10, 5),

        // Ă
        'ă' => (20, 0),
        'ằ' => (20, 1),
        'ẳ' => (20, 2),
        'ẵ' => (20, 3),
        'ắ' => (20, 4),
        'ặ' => (20, 5),

        // Â
        'â' => (30, 0),
        'ầ' => (30, 1),
        'ẩ' => (30, 2),
        'ẫ' => (30, 3),
        'ấ' => (30, 4),
        'ậ' => (30, 5),

        'b' => (40, 0),
        'c' => (50, 0),
        'd' => (60, 0),

        // Đ (comes right after D and before E in Vietnamese alphabet)
        'đ' => (70, 0),

        // E
        'e' => (80, 0),
        'è' => (80, 1),
        'ẻ' => (80, 2),
        'ẽ' => (80, 3),
        'é' => (80, 4),
        'ẹ' => (80, 5),

        // Ê
        'ê' => (90, 0),
        'ề' => (90, 1),
        'ể' => (90, 2),
        'ễ' => (90, 3),
        'ế' => (90, 4),
        'ệ' => (90, 5),

        'f' => (95, 0),
        'g' => (100, 0),
        'h' => (110, 0),

        // I
        'i' => (120, 0),
        'ì' => (120, 1),
        'ỉ' => (120, 2),
        'ĩ' => (120, 3),
        'í' => (120, 4),
        'ị' => (120, 5),

        'j' => (125, 0),
        'k' => (130, 0),
        'l' => (140, 0),
        'm' => (150, 0),
        'n' => (160, 0),

        // O
        'o' => (170, 0),
        'ò' => (170, 1),
        'ỏ' => (170, 2),
        'õ' => (170, 3),
        'ó' => (170, 4),
        'ọ' => (170, 5),

        // Ô
        'ô' => (180, 0),
        'ồ' => (180, 1),
        'ổ' => (180, 2),
        'ỗ' => (180, 3),
        'ố' => (180, 4),
        'ộ' => (180, 5),

        // Ơ
        'ơ' => (190, 0),
        'ờ' => (190, 1),
        'ở' => (190, 2),
        'ỡ' => (190, 3),
        'ớ' => (190, 4),
        'ợ' => (190, 5),

        'p' => (200, 0),
        'q' => (210, 0),
        'r' => (220, 0),
        's' => (230, 0),
        't' => (240, 0),

        // U
        'u' => (250, 0),
        'ù' => (250, 1),
        'ủ' => (250, 2),
        'ũ' => (250, 3),
        'ú' => (250, 4),
        'ụ' => (250, 5),

        // Ư
        'ư' => (260, 0),
        'ừ' => (260, 1),
        'ử' => (260, 2),
        'ữ' => (260, 3),
        'ứ' => (260, 4),
        'ự' => (260, 5),

        'v' => (270, 0),
        'w' => (275, 0),
        'x' => (280, 0),

        // Y
        'y' => (290, 0),
        'ỳ' => (290, 1),
        'ỷ' => (290, 2),
        'ỹ' => (290, 3),
        'ý' => (290, 4),
        'ỵ' => (290, 5),

        'z' => (300, 0),

        other => {
            let code = other as u32;
            let p = if code < 1000 { 1000 + code as u16 } else { 2000 };
            (p, 0)
        }
    }
}

/// Compares two strings using Vietnamese collation rules:
/// 1. Character-by-character base letter comparison
/// 2. If equal base letters, compares tone marks
/// 3. If equal tone marks, compares length and character case
pub fn compare_vietnamese_text(a: &str, b: &str) -> Ordering {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let a_weights: Vec<(u16, u8)> = a_chars.iter().map(|&c| vietnamese_char_weights(c)).collect();
    let b_weights: Vec<(u16, u8)> = b_chars.iter().map(|&c| vietnamese_char_weights(c)).collect();

    let min_len = a_weights.len().min(b_weights.len());

    // 1. Level 1: Primary base letter comparison
    for i in 0..min_len {
        let (pa, _) = a_weights.get(i).copied().unwrap_or((0, 0));
        let (pb, _) = b_weights.get(i).copied().unwrap_or((0, 0));
        if pa != pb {
            return pa.cmp(&pb);
        }
    }

    // 2. Level 2: Secondary tone mark comparison
    for i in 0..min_len {
        let (_, ta) = a_weights.get(i).copied().unwrap_or((0, 0));
        let (_, tb) = b_weights.get(i).copied().unwrap_or((0, 0));
        if ta != tb {
            return ta.cmp(&tb);
        }
    }

    // 3. Length comparison
    if a_weights.len() != b_weights.len() {
        return a_weights.len().cmp(&b_weights.len());
    }

    // 4. Exact character case tie-breaker
    a.cmp(b)
}

/// Extracts the Vietnamese given name (the last word in the name).
/// e.g. "Nguyễn Văn An" -> "An", "John" -> "John", "" -> ""
pub fn extract_vietnamese_first_name(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    match words.last() {
        Some(w) => (*w).to_string(),
        None => String::new(),
    }
}

/// Extracts the Vietnamese family and middle name (all words before the last word).
/// e.g. "Nguyễn Văn An" -> "Nguyễn Văn", "John" -> "", "" -> ""
pub fn extract_vietnamese_last_name(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    if words.len() <= 1 {
        return String::new();
    }
    words[..words.len() - 1].join(" ")
}

/// Compares two personal names in Vietnamese tradition:
/// First compares given names (Tên chính - the last word).
/// If given names are identical, compares family and middle names (Họ và đệm).
/// If still identical, compares full strings.
pub fn compare_vietnamese_name(a: &str, b: &str) -> Ordering {
    let first_a = extract_vietnamese_first_name(a);
    let first_b = extract_vietnamese_first_name(b);

    let cmp_first = compare_vietnamese_text(&first_a, &first_b);
    if cmp_first != Ordering::Equal {
        return cmp_first;
    }

    let last_a = extract_vietnamese_last_name(a);
    let last_b = extract_vietnamese_last_name(b);

    let cmp_last = compare_vietnamese_text(&last_a, &last_b);
    if cmp_last != Ordering::Equal {
        return cmp_last;
    }

    compare_vietnamese_text(a, b)
}

/// Capitalizes each word properly for Vietnamese text, preserving inter-word spacing.
/// e.g. "nguyễn văn an" -> "Nguyễn Văn An", "ĐẶNG THỊ THU HÀ" -> "Đặng Thị Thu Hà"
pub fn proper_vietnamese(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut new_word = true;

    for c in s.chars() {
        if c.is_alphabetic() {
            if new_word {
                for u in c.to_uppercase() {
                    out.push(u);
                }
                new_word = false;
            } else {
                for l in c.to_lowercase() {
                    out.push(l);
                }
            }
        } else {
            out.push(c);
            if c.is_whitespace() || c == '-' || c == '/' || c == '(' || c == '[' || c == '.' || c == ',' {
                new_word = true;
            }
        }
    }

    out
}

const TCVN3_CHARS: &[char] = &[
    'µ', '¸', '¶', '·', '¹', '¨', '»', '¾', '¼', '½', 'Æ', '©', 'Ç', 'Ê', 'È', 'É', 'Ë', '®', 'Ì', 'Ð', 'Î', 'Ï', 'Ñ', 'ª', 'Ò', 'Õ', 'Ó', 'Ô',
    'Ö', '×', 'Ý', 'Ø', 'Ü', 'Þ', 'ß', 'ã', 'á', 'â', 'ä', '«', 'å', 'è', 'æ', 'ç', 'é', '¬', 'ê', 'í', 'ë', 'ì', 'î', 'ï', 'ó', 'ñ', 'ò', 'ô',
    '\u{00AD}', 'õ', 'ø', 'ö', '÷', 'ù', 'ú', 'ý', 'û', 'ü', 'þ', '¡', '¢', '§', '£', '¤', '¥', '¦',
];

const UNICODE_CHARS: &[char] = &[
    'à', 'á', 'ả', 'ã', 'ạ', 'ă', 'ằ', 'ắ', 'ẳ', 'ẵ', 'ặ', 'â', 'ầ', 'ấ', 'ẩ', 'ẫ', 'ậ', 'đ', 'è', 'é', 'ẻ', 'ẽ', 'ẹ', 'ê', 'ề', 'ế', 'ể', 'ễ',
    'ệ', 'ì', 'í', 'ỉ', 'ĩ', 'ị', 'ò', 'ó', 'ỏ', 'õ', 'ọ', 'ô', 'ồ', 'ố', 'ổ', 'ỗ', 'ộ', 'ơ', 'ờ', 'ớ', 'ở', 'ỡ', 'ợ', 'ù', 'ú', 'ủ', 'ũ', 'ụ',
    'ư', 'ừ', 'ứ', 'ử', 'ữ', 'ự', 'ỳ', 'ý', 'ỷ', 'ỹ', 'ỵ', 'Ă', 'Â', 'Đ', 'Ê', 'Ô', 'Ơ', 'Ư',
];

/// Converts TCVN3 (ABC/.VnTime) encoded text to standard Unicode.
pub fn tcvn3_to_unicode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if let Some(pos) = TCVN3_CHARS.iter().position(|&x| x == c) {
            if let Some(&u) = UNICODE_CHARS.get(pos) {
                out.push(u);
            } else {
                out.push(c);
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Converts Unicode text to TCVN3 (ABC/.VnTime) encoding.
pub fn unicode_to_tcvn3(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if let Some(pos) = UNICODE_CHARS.iter().position(|&x| x == c) {
            if let Some(&t) = TCVN3_CHARS.get(pos) {
                out.push(t);
            } else {
                out.push(c);
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vietnamese_collation_alphabet() {
        assert_eq!(compare_vietnamese_text("An", "Bình"), Ordering::Less);
        assert_eq!(compare_vietnamese_text("Dũng", "Đạt"), Ordering::Less);
        assert_eq!(compare_vietnamese_text("Đạt", "Giang"), Ordering::Less);
        assert_eq!(compare_vietnamese_text("Vinh", "Xuân"), Ordering::Less);
        assert_eq!(compare_vietnamese_text("Xuân", "Yến"), Ordering::Less);
    }

    #[test]
    fn test_vietnamese_collation_tones() {
        // Không dấu < Huyền < Hỏi < Ngã < Sắc < Nặng
        assert_eq!(compare_vietnamese_text("ba", "bà"), Ordering::Less);
        assert_eq!(compare_vietnamese_text("bà", "bả"), Ordering::Less);
        assert_eq!(compare_vietnamese_text("bả", "bã"), Ordering::Less);
        assert_eq!(compare_vietnamese_text("bã", "bá"), Ordering::Less);
        assert_eq!(compare_vietnamese_text("bá", "bạ"), Ordering::Less);
    }

    #[test]
    fn test_vietnamese_name_sorting() {
        // "Nguyễn Văn An" should come before "Lê Thị Cúc" because An < Cúc
        assert_eq!(compare_vietnamese_name("Nguyễn Văn An", "Lê Thị Cúc"), Ordering::Less);
        // When given names are equal, compare family & middle names
        assert_eq!(compare_vietnamese_name("Lê Thị Cúc", "Nguyễn Thị Cúc"), Ordering::Less);
        assert_eq!(compare_vietnamese_name("Nguyễn Văn An", "Trần Văn An"), Ordering::Less);
    }

    #[test]
    fn test_extract_names() {
        assert_eq!(extract_vietnamese_first_name("Nguyễn Văn An"), "An");
        assert_eq!(extract_vietnamese_last_name("Nguyễn Văn An"), "Nguyễn Văn");
        assert_eq!(extract_vietnamese_first_name("John"), "John");
        assert_eq!(extract_vietnamese_last_name("John"), "");
        assert_eq!(extract_vietnamese_first_name(""), "");
        assert_eq!(extract_vietnamese_last_name(""), "");
    }

    #[test]
    fn test_proper_vietnamese() {
        assert_eq!(proper_vietnamese("nguyễn văn an"), "Nguyễn Văn An");
        assert_eq!(proper_vietnamese("ĐẶNG THỊ THU HÀ"), "Đặng Thị Thu Hà");
        assert_eq!(proper_vietnamese("trần-thị-mai"), "Trần-Thị-Mai");
    }

    #[test]
    fn test_tcvn3_conversion() {
        // "hộp mứt tết" in TCVN3 is "hép møt tÕt"
        assert_eq!(tcvn3_to_unicode("hép møt tÕt"), "hộp mứt tết");
        assert_eq!(unicode_to_tcvn3("hộp mứt tết"), "hép møt tÕt");
        assert_eq!(tcvn3_to_unicode("Céng hoµ x· héi chñ nghÜa ViÖt Nam"), "Cộng hoà xã hội chủ nghĩa Việt Nam");
    }
}
