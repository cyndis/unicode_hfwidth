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

/// Returns the half-width form for `ch`. If no half-width form for `ch` exists,
/// or `ch` is already in half-width form, returns `None`.
///
/// # Example
/// ```rust
/// assert_eq!(unicode_hfwidth::to_halfwidth('カ'), Some('ｶ'));
/// assert_eq!(unicode_hfwidth::to_halfwidth('a'), None);
/// ```
pub fn to_halfwidth(ch: char) -> Option<char> {
    let ch = ch as u32;
    unsafe {
        match ch {
            /* Full-width variant characters */
            0xff01...0xff5e => Some(transmute(ch - 0xff01 + 0x0021)),
            0xff5f...0xff60 => Some(transmute(ch - 0xff5f + 0x2985)),
            0xffe0...0xffe1 => Some(transmute(ch - 0xffe0 + 0x00a2)),
            0xffe2          => Some(transmute(0x00acu32)),
            0xffe3          => Some(transmute(0x00afu32)),
            0xffe4          => Some(transmute(0x00a6u32)),
            0xffe5          => Some(transmute(0x00a5u32)),
            0xffe6          => Some(transmute(0x20a9u32)),

            /* Natural full-width characters */
            0x3002 => Some(transmute(0xff61u32)),
            0x300c => Some(transmute(0xff62u32)),
            0x300d => Some(transmute(0xff63u32)),
            0x3001 => Some(transmute(0xff64u32)),
            0x30fb => Some(transmute(0xff65u32)),
            0x30f2 => Some(transmute(0xff66u32)),
            0x30a1 => Some(transmute(0xff67u32)),
            0x30a3 => Some(transmute(0xff68u32)),
            0x30a5 => Some(transmute(0xff69u32)),
            0x30a7 => Some(transmute(0xff6au32)),
            0x30a9 => Some(transmute(0xff6bu32)),
            0x30e3 => Some(transmute(0xff6cu32)),
            0x30e5 => Some(transmute(0xff6du32)),
            0x30e7 => Some(transmute(0xff6eu32)),
            0x30c3 => Some(transmute(0xff6fu32)),
            0x30fc => Some(transmute(0xff70u32)),
            0x30a2 => Some(transmute(0xff71u32)),
            0x30a4 => Some(transmute(0xff72u32)),
            0x30a6 => Some(transmute(0xff73u32)),
            0x30a8 => Some(transmute(0xff74u32)),
            0x30aa => Some(transmute(0xff75u32)),
            0x30ab => Some(transmute(0xff76u32)),
            0x30ad => Some(transmute(0xff77u32)),
            0x30af => Some(transmute(0xff78u32)),
            0x30b1 => Some(transmute(0xff79u32)),
            0x30b3 => Some(transmute(0xff7au32)),
            0x30b5 => Some(transmute(0xff7bu32)),
            0x30b7 => Some(transmute(0xff7cu32)),
            0x30b9 => Some(transmute(0xff7du32)),
            0x30bb => Some(transmute(0xff7eu32)),
            0x30bd => Some(transmute(0xff7fu32)),
            0x30bf => Some(transmute(0xff80u32)),
            0x30c1 => Some(transmute(0xff81u32)),
            0x30c4 => Some(transmute(0xff82u32)),
            0x30c6 => Some(transmute(0xff83u32)),
            0x30c8 => Some(transmute(0xff84u32)),
            0x30ca => Some(transmute(0xff85u32)),
            0x30cb => Some(transmute(0xff86u32)),
            0x30cc => Some(transmute(0xff87u32)),
            0x30cd => Some(transmute(0xff88u32)),
            0x30ce => Some(transmute(0xff89u32)),
            0x30cf => Some(transmute(0xff8au32)),
            0x30d2 => Some(transmute(0xff8bu32)),
            0x30d5 => Some(transmute(0xff8cu32)),
            0x30d8 => Some(transmute(0xff8du32)),
            0x30db => Some(transmute(0xff8eu32)),
            0x30de => Some(transmute(0xff8fu32)),
            0x30df => Some(transmute(0xff90u32)),
            0x30e0 => Some(transmute(0xff91u32)),
            0x30e1 => Some(transmute(0xff92u32)),
            0x30e2 => Some(transmute(0xff93u32)),
            0x30e4 => Some(transmute(0xff94u32)),
            0x30e6 => Some(transmute(0xff95u32)),
            0x30e8 => Some(transmute(0xff96u32)),
            0x30e9 => Some(transmute(0xff97u32)),
            0x30ea => Some(transmute(0xff98u32)),
            0x30eb => Some(transmute(0xff99u32)),
            0x30ec => Some(transmute(0xff9au32)),
            0x30ed => Some(transmute(0xff9bu32)),
            0x30ef => Some(transmute(0xff9cu32)),
            0x30f3 => Some(transmute(0xff9du32)),
            0x3099 => Some(transmute(0xff9eu32)),
            0x309a => Some(transmute(0xff9fu32)),
            0x3164 => Some(transmute(0xffa0u32)),
            0x3131 => Some(transmute(0xffa1u32)),
            0x3132 => Some(transmute(0xffa2u32)),
            0x3133 => Some(transmute(0xffa3u32)),
            0x3134 => Some(transmute(0xffa4u32)),
            0x3135 => Some(transmute(0xffa5u32)),
            0x3136 => Some(transmute(0xffa6u32)),
            0x3137 => Some(transmute(0xffa7u32)),
            0x3138 => Some(transmute(0xffa8u32)),
            0x3139 => Some(transmute(0xffa9u32)),
            0x313a => Some(transmute(0xffaau32)),
            0x313b => Some(transmute(0xffabu32)),
            0x313c => Some(transmute(0xffacu32)),
            0x313d => Some(transmute(0xffadu32)),
            0x313e => Some(transmute(0xffaeu32)),
            0x313f => Some(transmute(0xffafu32)),
            0x3140 => Some(transmute(0xffb0u32)),
            0x3141 => Some(transmute(0xffb1u32)),
            0x3142 => Some(transmute(0xffb2u32)),
            0x3143 => Some(transmute(0xffb3u32)),
            0x3144 => Some(transmute(0xffb4u32)),
            0x3145 => Some(transmute(0xffb5u32)),
            0x3146 => Some(transmute(0xffb6u32)),
            0x3147 => Some(transmute(0xffb7u32)),
            0x3148 => Some(transmute(0xffb8u32)),
            0x3149 => Some(transmute(0xffb9u32)),
            0x314a => Some(transmute(0xffbau32)),
            0x314b => Some(transmute(0xffbbu32)),
            0x314c => Some(transmute(0xffbcu32)),
            0x314d => Some(transmute(0xffbdu32)),
            0x314e => Some(transmute(0xffbeu32)),
            0x314f => Some(transmute(0xffc2u32)),
            0x3150 => Some(transmute(0xffc3u32)),
            0x3151 => Some(transmute(0xffc4u32)),
            0x3152 => Some(transmute(0xffc5u32)),
            0x3153 => Some(transmute(0xffc6u32)),
            0x3154 => Some(transmute(0xffc7u32)),
            0x3155 => Some(transmute(0xffcau32)),
            0x3156 => Some(transmute(0xffcbu32)),
            0x3157 => Some(transmute(0xffccu32)),
            0x3158 => Some(transmute(0xffcdu32)),
            0x3159 => Some(transmute(0xffceu32)),
            0x315a => Some(transmute(0xffcfu32)),
            0x315b => Some(transmute(0xffd2u32)),
            0x315c => Some(transmute(0xffd3u32)),
            0x315d => Some(transmute(0xffd4u32)),
            0x315e => Some(transmute(0xffd5u32)),
            0x315f => Some(transmute(0xffd6u32)),
            0x3160 => Some(transmute(0xffd7u32)),
            0x3161 => Some(transmute(0xffdau32)),
            0x3162 => Some(transmute(0xffdbu32)),
            0x3163 => Some(transmute(0xffdcu32)),
            0x2502 => Some(transmute(0xffe8u32)),
            0x2190 => Some(transmute(0xffe9u32)),
            0x2191 => Some(transmute(0xffeau32)),
            0x2192 => Some(transmute(0xffebu32)),
            0x2193 => Some(transmute(0xffecu32)),
            0x25a0 => Some(transmute(0xffedu32)),
            0x25cb => Some(transmute(0xffeeu32)),

            _ => None
        }
    }
}

/// Returns the full-width form for `ch`. If no full-width form for `ch` exists,
/// or `ch` is already in full-width form, returns `None`.
///
/// # Example
/// ```rust
/// assert_eq!(unicode_hfwidth::to_fullwidth('a'), Some('ａ'));
/// assert_eq!(unicode_hfwidth::to_fullwidth('カ'), None);
/// ```
pub fn to_fullwidth(ch: char) -> Option<char> {
    let ch = ch as u32;
    unsafe {
        match ch {
            /* Half-width variant characters */
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

            /* Natural half-width characters */
            0x0021...0x007e => Some(transmute(ch - 0x0021 + 0xff01)),
            0x2985...0x2986 => Some(transmute(ch - 0x2985 + 0xff5f)),
            0x00a2...0x00a3 => Some(transmute(ch - 0x00a2 + 0xffe0)),
            0x00ac          => Some(transmute(0xffe2u32)),
            0x00af          => Some(transmute(0xffe3u32)),
            0x00a6          => Some(transmute(0xffe4u32)),
            0x00a5          => Some(transmute(0xffe5u32)),
            0x20a9          => Some(transmute(0xffe6u32)),

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

#[test]
fn test_katakana_rev() {
    let full = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワン";
    let half = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ";
    for (f, h) in full.chars().zip(half.chars()) {
        assert_eq!(to_halfwidth(f).unwrap(), h);
    }
}

#[test]
fn test_a() {
    assert_eq!(to_fullwidth('a').unwrap(), 'ａ');
}
