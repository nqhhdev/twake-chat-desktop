//! Helpers that reproduce JavaScript string semantics the web app relies on.

/// `\s` of a JavaScript regex (ECMAScript WhiteSpace + LineTerminator).
/// Differs from `char::is_whitespace`: includes U+FEFF, excludes U+0085.
pub fn is_js_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r' | ' ' | '\u{A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202F}'
                | '\u{205F}'
                | '\u{3000}'
                | '\u{FEFF}'
    )
}

/// `String.prototype.trim`.
pub fn trim(s: &str) -> &str {
    s.trim_matches(is_js_space)
}

/// `s.slice(0, max)` where `max` counts UTF-16 code units, as JavaScript does.
/// When the cut falls inside a surrogate pair, JavaScript keeps the lone high
/// surrogate; Rust strings cannot hold one, so it is dropped here. Callers that
/// strip a trailing high surrogate afterwards (as `toPreviewText` does) get the
/// same visible result.
pub fn slice_utf16(s: &str, max: usize) -> &str {
    let mut units = 0;
    for (index, c) in s.char_indices() {
        let width = c.len_utf16();
        if units + width > max {
            return &s[..index];
        }
        units += width;
    }
    s
}

/// `Math.round`: nearest integer, ties toward +∞ (Rust's `round` ties away from 0).
pub fn math_round(x: f64) -> f64 {
    if !x.is_finite() {
        return x;
    }
    let floor = x.floor();
    let r = if x - floor >= 0.5 { floor + 1.0 } else { floor };
    // JavaScript keeps the sign of zero: Math.round(-0.4) is -0.
    if r == 0.0 && x.is_sign_negative() {
        -0.0
    } else {
        r
    }
}

/// `Math.min(...)`: NaN when any argument is NaN (Rust's `f64::min` ignores NaN), and -0
/// below +0 (`f64::min` may return either).
pub fn math_min(values: &[f64]) -> f64 {
    values
        .iter()
        .try_fold(f64::INFINITY, |acc, &v| {
            if v.is_nan() {
                None
            } else if v < acc || (v == acc && v.is_sign_negative()) {
                Some(v)
            } else {
                Some(acc)
            }
        })
        .unwrap_or(f64::NAN)
}

/// `Math.max(...)`: NaN when any argument is NaN, and +0 above -0.
pub fn math_max(values: &[f64]) -> f64 {
    values
        .iter()
        .try_fold(f64::NEG_INFINITY, |acc, &v| {
            if v.is_nan() {
                None
            } else if v > acc || (v == acc && v.is_sign_positive()) {
                Some(v)
            } else {
                Some(acc)
            }
        })
        .unwrap_or(f64::NAN)
}

/// `String(number)` for the values the domain formats: integers, NaN, ±Infinity,
/// and exponent form from 1e21 as JavaScript writes it (`1e+21`).
pub fn number_to_string(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x > 0.0 {
            "Infinity".into()
        } else {
            "-Infinity".into()
        };
    }
    if x == 0.0 {
        return "0".into(); // String(-0) is "0"
    }
    let abs = x.abs();
    if (1e-6..1e21).contains(&abs) {
        return format!("{x}");
    }
    let s = format!("{x:e}");
    match s.split_once('e') {
        Some((mantissa, exp)) if !exp.starts_with('-') => format!("{mantissa}e+{exp}"),
        Some((mantissa, exp)) => format!("{mantissa}e{exp}"),
        None => s,
    }
}

/// Length in UTF-16 code units (`string.length`).
pub fn len_utf16(s: &str) -> usize {
    s.chars().map(char::len_utf16).sum()
}

/// The byte offset of the UTF-16 offset `units` (clamped to the end).
pub fn byte_of_utf16(s: &str, units: usize) -> usize {
    slice_utf16(s, units).len()
}

/// The UTF-16 offset of the byte offset `byte` (a char boundary, clamped to the end).
pub fn utf16_of_byte(s: &str, byte: usize) -> usize {
    len_utf16(s.get(..byte.min(s.len())).unwrap_or(s))
}

/// Approximates `a.localeCompare(b)` of the root locale (ICU): letters compared
/// without accents or case first, then accents, then lower case before upper case;
/// spaces and punctuation before digits, digits before letters. Exact ICU collation
/// would need ICU's tables; names of people and rooms sort the same in practice.
pub fn locale_compare(a: &str, b: &str) -> std::cmp::Ordering {
    fn class(c: char) -> u8 {
        if c.is_alphabetic() {
            2
        } else if c.is_numeric() {
            1
        } else {
            0
        }
    }
    let primary = |s: &str| -> Vec<(u8, char)> {
        crate::search::normalize_for_search(s)
            .chars()
            .map(|c| (class(c), c))
            .collect()
    };
    let accents = |s: &str| s.to_lowercase();
    let case = |s: &str| -> Vec<bool> { s.chars().map(char::is_uppercase).collect() };
    primary(a)
        .cmp(&primary(b))
        .then_with(|| accents(a).cmp(&accents(b)))
        .then_with(|| case(a).cmp(&case(b)))
        .then_with(|| a.cmp(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices_by_utf16_units() {
        assert_eq!(slice_utf16("abc", 2), "ab");
        assert_eq!(slice_utf16("a🎉b", 2), "a"); // the cut would split the emoji
        assert_eq!(slice_utf16("a🎉b", 3), "a🎉");
        assert_eq!(len_utf16("a🎉"), 3);
    }

    #[test]
    fn trims_js_spaces() {
        assert_eq!(trim("\u{FEFF} x \u{3000}"), "x");
        assert_eq!(trim("\u{85}x"), "\u{85}x");
    }
}
