//! Utilities for handling characters in the Unicode "Halfwidth and Fullwidth Forms" block.

use std::mem::transmute;

/// Checks if `ch` is in the Unicode "Halfwidth and Fullwidth Forms" block.
///
/// # Example
/// ```rust
/// assert_eq!(unicode_hfwidth::is_nonstandard_width('カ'), false);
/// assert_eq!(unicode_hfwidth::is_nonstandard_width('ｶ'), true);
/// ```
pub fn is_nonstandard_width(ch: char) -> bool {
    match ch as u32 {
        0xff00...0xffee => true,
        _               => false,
    }
}

/// Returns the standard-width form for `ch`. If `ch` is not in the Unicode
/// "Halfwidth and Fullwidth forms" block, returns `None`.
///
/// # Example
/// ```rust
/// assert_eq!(unicode_hfwidth::to_standard_width('カ'), None);
/// assert_eq!(unicode_hfwidth::to_standard_width('ｶ'), Some('カ'));
/// assert_eq!(unicode_hfwidth::to_standard_width('ａ'), Some('a'));
/// ```
pub fn to_standard_width(ch: char) -> Option<char> {
    match ch as u32 {
        0xff01...0xff60 => to_halfwidth(ch),
        0xff61...0xffdc => to_fullwidth(ch),
        0xffe0...0xffe6 => to_halfwidth(ch),
        0xffe8...0xffee => to_fullwidth(ch),
        _               => None
    }
}

// Left private because conversion from standard full-width characters not supported yet
fn to_halfwidth(ch: char) -> Option<char> {
    let ch = ch as u32;
    unsafe {
        match ch {
            0xff01...0xff5e => Some(transmute((ch - 0xff01 + 0x0021))),
            0xff5f...0xff60 => Some(transmute((ch - 0xff5f + 0x2985))),
            0xffe0...0xffe1 => Some(transmute((ch - 0xffe0 + 0x00a2))),
            0xffe2          => Some(transmute(0x00acu32)),
            0xffe3          => Some(transmute(0x00afu32)),
            0xffe4          => Some(transmute(0x00a6u32)),
            0xffe5          => Some(transmute(0x00a5u32)),
            0xffe6          => Some(transmute(0x20a9u32)),
            _ => None
        }
    }
}

// Left private because conversion from standard half-width characters not supported yet
fn to_fullwidth(ch: char) -> Option<char> {
    let ch = ch as u32;
    unsafe {
        match ch {
            0xff61 => Some(transmute(0x3002u32)),
            0xff62 => Some(transmute(0x300cu32)),
            0xff63 => Some(transmute(0x300du32)),
            0xff64 => Some(transmute(0x3001u32)),
            0xff65 => Some(transmute(0x30fbu32)),
            0xff66 => Some(transmute(0x30f2u32)),
            0xff67 => Some(transmute(0x30a1u32)),
            0xff68 => Some(transmute(0x30a3u32)),
            0xff69 => Some(transmute(0x30a5u32)),
            0xff6a => Some(transmute(0x30a7u32)),
            0xff6b => Some(transmute(0x30a9u32)),
            0xff6c => Some(transmute(0x30e3u32)),
            0xff6d => Some(transmute(0x30e5u32)),
            0xff6e => Some(transmute(0x30e7u32)),
            0xff6f => Some(transmute(0x30c3u32)),
            0xff70 => Some(transmute(0x30fcu32)),
            0xff71 => Some(transmute(0x30a2u32)),
            0xff72 => Some(transmute(0x30a4u32)),
            0xff73 => Some(transmute(0x30a6u32)),
            0xff74 => Some(transmute(0x30a8u32)),
            0xff75 => Some(transmute(0x30aau32)),
            0xff76 => Some(transmute(0x30abu32)),
            0xff77 => Some(transmute(0x30adu32)),
            0xff78 => Some(transmute(0x30afu32)),
            0xff79 => Some(transmute(0x30b1u32)),
            0xff7a => Some(transmute(0x30b3u32)),
            0xff7b => Some(transmute(0x30b5u32)),
            0xff7c => Some(transmute(0x30b7u32)),
            0xff7d => Some(transmute(0x30b9u32)),
            0xff7e => Some(transmute(0x30bbu32)),
            0xff7f => Some(transmute(0x30bdu32)),
            0xff80 => Some(transmute(0x30bfu32)),
            0xff81 => Some(transmute(0x30c1u32)),
            0xff82 => Some(transmute(0x30c4u32)),
            0xff83 => Some(transmute(0x30c6u32)),
            0xff84 => Some(transmute(0x30c8u32)),
            0xff85 => Some(transmute(0x30cau32)),
            0xff86 => Some(transmute(0x30cbu32)),
            0xff87 => Some(transmute(0x30ccu32)),
            0xff88 => Some(transmute(0x30cdu32)),
            0xff89 => Some(transmute(0x30ceu32)),
            0xff8a => Some(transmute(0x30cfu32)),
            0xff8b => Some(transmute(0x30d2u32)),
            0xff8c => Some(transmute(0x30d5u32)),
            0xff8d => Some(transmute(0x30d8u32)),
            0xff8e => Some(transmute(0x30dbu32)),
            0xff8f => Some(transmute(0x30deu32)),
            0xff90 => Some(transmute(0x30dfu32)),
            0xff91 => Some(transmute(0x30e0u32)),
            0xff92 => Some(transmute(0x30e1u32)),
            0xff93 => Some(transmute(0x30e2u32)),
            0xff94 => Some(transmute(0x30e4u32)),
            0xff95 => Some(transmute(0x30e6u32)),
            0xff96 => Some(transmute(0x30e8u32)),
            0xff97 => Some(transmute(0x30e9u32)),
            0xff98 => Some(transmute(0x30eau32)),
            0xff99 => Some(transmute(0x30ebu32)),
            0xff9a => Some(transmute(0x30ecu32)),
            0xff9b => Some(transmute(0x30edu32)),
            0xff9c => Some(transmute(0x30efu32)),
            0xff9d => Some(transmute(0x30f3u32)),
            0xff9e => Some(transmute(0x3099u32)),
            0xff9f => Some(transmute(0x309au32)),
            0xffa0 => Some(transmute(0x3164u32)),
            0xffa1 => Some(transmute(0x3131u32)),
            0xffa2 => Some(transmute(0x3132u32)),
            0xffa3 => Some(transmute(0x3133u32)),
            0xffa4 => Some(transmute(0x3134u32)),
            0xffa5 => Some(transmute(0x3135u32)),
            0xffa6 => Some(transmute(0x3136u32)),
            0xffa7 => Some(transmute(0x3137u32)),
            0xffa8 => Some(transmute(0x3138u32)),
            0xffa9 => Some(transmute(0x3139u32)),
            0xffaa => Some(transmute(0x313au32)),
            0xffab => Some(transmute(0x313bu32)),
            0xffac => Some(transmute(0x313cu32)),
            0xffad => Some(transmute(0x313du32)),
            0xffae => Some(transmute(0x313eu32)),
            0xffaf => Some(transmute(0x313fu32)),
            0xffb0 => Some(transmute(0x3140u32)),
            0xffb1 => Some(transmute(0x3141u32)),
            0xffb2 => Some(transmute(0x3142u32)),
            0xffb3 => Some(transmute(0x3143u32)),
            0xffb4 => Some(transmute(0x3144u32)),
            0xffb5 => Some(transmute(0x3145u32)),
            0xffb6 => Some(transmute(0x3146u32)),
            0xffb7 => Some(transmute(0x3147u32)),
            0xffb8 => Some(transmute(0x3148u32)),
            0xffb9 => Some(transmute(0x3149u32)),
            0xffba => Some(transmute(0x314au32)),
            0xffbb => Some(transmute(0x314bu32)),
            0xffbc => Some(transmute(0x314cu32)),
            0xffbd => Some(transmute(0x314du32)),
            0xffbe => Some(transmute(0x314eu32)),
            0xffc2 => Some(transmute(0x314fu32)),
            0xffc3 => Some(transmute(0x3150u32)),
            0xffc4 => Some(transmute(0x3151u32)),
            0xffc5 => Some(transmute(0x3152u32)),
            0xffc6 => Some(transmute(0x3153u32)),
            0xffc7 => Some(transmute(0x3154u32)),
            0xffca => Some(transmute(0x3155u32)),
            0xffcb => Some(transmute(0x3156u32)),
            0xffcc => Some(transmute(0x3157u32)),
            0xffcd => Some(transmute(0x3158u32)),
            0xffce => Some(transmute(0x3159u32)),
            0xffcf => Some(transmute(0x315au32)),
            0xffd2 => Some(transmute(0x315bu32)),
            0xffd3 => Some(transmute(0x315cu32)),
            0xffd4 => Some(transmute(0x315du32)),
            0xffd5 => Some(transmute(0x315eu32)),
            0xffd6 => Some(transmute(0x315fu32)),
            0xffd7 => Some(transmute(0x3160u32)),
            0xffda => Some(transmute(0x3161u32)),
            0xffdb => Some(transmute(0x3162u32)),
            0xffdc => Some(transmute(0x3163u32)),
            0xffe8 => Some(transmute(0x2502u32)),
            0xffe9 => Some(transmute(0x2190u32)),
            0xffea => Some(transmute(0x2191u32)),
            0xffeb => Some(transmute(0x2192u32)),
            0xffec => Some(transmute(0x2193u32)),
            0xffed => Some(transmute(0x25a0u32)),
            0xffee => Some(transmute(0x25cbu32)),
            _ => None,
        }
    }
}

#[test]
fn test_katakana() {
    let full = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワン";
    let half = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ";
    for (f, h) in full.chars().zip(half.chars()) {
        assert_eq!(to_fullwidth(h).unwrap(), f);
    }
}
