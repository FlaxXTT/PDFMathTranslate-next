use std::ops::Range;

fn markers(text: &str) -> impl Iterator<Item = Range<usize>> + '_ {
    text.match_indices("[[BD_").filter_map(|(start, _)| {
        let bytes = text.as_bytes().get(start + 5..)?;
        if !matches!(bytes, [b'F' | b'L' | b'R', b'_', b'0'..=b'9', ..]) {
            return None;
        }
        let digits = bytes[2..]
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        bytes[2 + digits..]
            .starts_with(b"]]")
            .then_some(start..start + 9 + digits)
    })
}

struct Character {
    offset: usize,
    value: char,
    in_marker: bool,
    inside_marker: bool,
}

fn characters(text: &str) -> impl Iterator<Item = Character> + '_ {
    let mut markers = markers(text).peekable();
    text.char_indices().map(move |(offset, value)| {
        if markers.peek().is_some_and(|marker| offset >= marker.end) {
            markers.next();
        }
        let marker = markers.peek().filter(|marker| marker.contains(&offset));
        Character {
            offset,
            value,
            in_marker: marker.is_some(),
            inside_marker: marker.is_some_and(|marker| marker.start < offset),
        }
    })
}

fn sentence_end(ch: char) -> bool {
    matches!(ch, '.' | '!' | '?' | '。' | '！' | '？' | '؟' | '।' | '…')
}

fn closing_punctuation(ch: char) -> bool {
    matches!(
        ch,
        '"' | '\'' | '”' | '’' | '»' | '」' | '』' | '）' | '】' | '》' | ')' | ']' | '}'
    )
}

fn grapheme_extension(ch: char) -> bool {
    matches!(ch as u32, 0x0300..=0x036f | 0x1ab0..=0x1aff | 0x1dc0..=0x1dff
        | 0x200c..=0x200d | 0x20d0..=0x20ff | 0xfe00..=0xfe0f | 0xfe20..=0xfe2f
        | 0x1f3fb..=0x1f3ff | 0xe0100..=0xe01ef)
}

fn safe_between(left: char, right: char) -> bool {
    let regional = |ch: char| matches!(ch as u32, 0x1f1e6..=0x1f1ff);
    left != '\u{200d}' && !grapheme_extension(right) && !(regional(left) && regional(right))
}

fn ends_sentence_before(chars: &[Character], boundary: usize, right: char) -> bool {
    let mut tail = chars[..boundary].iter().rev().map(|ch| ch.value);
    let Some(last) = tail.find(|ch| !closing_punctuation(*ch)) else {
        return false;
    };
    sentence_end(last)
        && !(last == '.'
            && right.is_ascii_digit()
            && tail.next().is_some_and(|ch| ch.is_ascii_digit()))
}

/// Divide at a balanced, Unicode-safe boundary. The middle slice is any whitespace
/// removed from the two requests, so joining these three slices gives the original.
pub(super) fn split(text: &str) -> Option<(&str, &str, &str)> {
    let chars: Vec<_> = characters(text).collect();
    let is_content = |ch: &Character| !ch.in_marker && ch.value.is_alphanumeric();
    let first_content = chars.iter().position(is_content)?;
    let last_content = chars.iter().rposition(is_content)?;
    if first_content == last_content {
        return None;
    }

    let mut best = None;
    for left in first_content + 1..=last_content {
        if chars[left].inside_marker || chars[left - 1].value.is_whitespace() {
            continue;
        }
        let right = left
            + chars[left..]
                .iter()
                .position(|ch| !ch.value.is_whitespace())?;
        if right > last_content || !safe_between(chars[left - 1].value, chars[right].value) {
            continue;
        }
        let distance = (left + right).abs_diff(chars.len());
        let rank = if distance <= chars.len() / 3
            && ends_sentence_before(&chars, left, chars[right].value)
        {
            0
        } else if distance <= chars.len() * 2 / 5 && left != right {
            1
        } else {
            2
        };
        let score = (rank, distance);
        if best.is_none_or(|(previous, _, _)| score < previous) {
            best = Some((score, chars[left].offset, chars[right].offset));
        }
    }

    best.map(|(_, left, right)| (&text[..left], &text[left..right], &text[right..]))
}

fn units(ch: char) -> usize {
    if !ch.is_alphanumeric() {
        return 0;
    }
    // One ideograph often carries about as much content as several Latin letters.
    // Kana, Hangul and common Southeast Asian scripts need a smaller adjustment.
    match ch as u32 {
        0x3400..=0x9fff | 0xf900..=0xfaff | 0x20000..=0x323af => 3,
        0x0e00..=0x0eff
        | 0x1000..=0x109f
        | 0x1780..=0x17ff
        | 0x3040..=0x30ff
        | 0x31f0..=0x31ff
        | 0x1100..=0x11ff
        | 0x3130..=0x318f
        | 0xac00..=0xd7af => 2,
        _ => 1,
    }
}

struct TextShape {
    value: usize,
    end: Option<char>,
    last_word: String,
}

fn shape(text: &str) -> TextShape {
    let visible: String = characters(text)
        .filter(|ch| !ch.in_marker)
        .map(|ch| ch.value)
        .collect();
    let end = visible
        .chars()
        .rev()
        .find(|&ch| !ch.is_whitespace() && !closing_punctuation(ch));
    let last_word = visible
        .trim_end_matches(|ch: char| !ch.is_alphabetic())
        .rsplit(|ch: char| !ch.is_alphabetic())
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    TextShape {
        value: visible.chars().map(units).sum(),
        end,
        last_word,
    }
}

/// A conservative fallback for services that do not provide a completion reason.
/// It estimates relative content in different scripts and ignores protected anchors.
pub(super) fn looks_incomplete(source: &str, reply: &str, after_split: bool) -> bool {
    let source = shape(source);
    let minimum = if after_split { 72 } else { 220 };
    if source.value < minimum {
        return false;
    }
    let reply = shape(reply);
    let ratio = reply.value as f64 / source.value as f64;
    if ratio < 0.27 {
        return true;
    }

    if source.value < 220
        || !source.end.is_some_and(sentence_end)
        || reply.end.is_some_and(sentence_end)
    {
        return false;
    }
    let dangling = reply.end.is_some_and(|ch| {
        matches!(
            ch,
            ',' | ';' | ':' | '-' | '—' | '(' | '[' | '{' | '，' | '、' | '；' | '：' | '（' | '【'
        )
    }) || matches!(
        reply.last_word.as_str(),
        "and"
            | "or"
            | "but"
            | "because"
            | "the"
            | "a"
            | "to"
            | "of"
            | "in"
            | "with"
            | "for"
            | "from"
            | "as"
            | "which"
            | "that"
    );
    ratio < if dangling { 0.59 } else { 0.47 }
}

#[cfg(test)]
mod tests {
    use super::{looks_incomplete, split};

    #[test]
    fn splits_on_nearby_sentence_and_preserves_separator() {
        let text = "  First sentence ends here. \t\nSecond sentence follows.  ";
        assert_eq!(
            split(text),
            Some((
                "  First sentence ends here.",
                " \t\n",
                "Second sentence follows.  "
            ))
        );

        let text = "The left has more than one word and the right has several words as well";
        let (left, separator, right) = split(text).unwrap();
        assert_eq!(separator, " ");
        assert!((left.len() as f64 / text.len() as f64 - 0.5).abs() < 0.1);
        assert_eq!([left, separator, right].concat(), text);
    }

    #[test]
    fn preserves_every_protected_token_and_requires_content_on_each_side() {
        let text = "甲乙丙丁[[BD_L_123]][[BD_F_9]]辰巳午未[[BD_R_123]]";
        let (left, separator, right) = split(text).unwrap();
        assert_eq!([left, separator, right].concat(), text);
        for token in ["[[BD_L_123]]", "[[BD_F_9]]", "[[BD_R_123]]"] {
            assert!(left.contains(token) || right.contains(token));
        }
        assert_eq!(split("[[BD_L_26]] [[BD_F_205]] 字 [[BD_R_26]]"), None);
        assert_eq!(split(" \t。!  "), None);
    }

    #[test]
    fn splits_unspaced_text_without_breaking_common_graphemes() {
        let text = "春雨渐浓。河岸的灯在远处闪烁。";
        let (left, separator, right) = split(text).unwrap();
        assert_eq!(left, "春雨渐浓。");
        assert_eq!(separator, "");
        assert_eq!([left, separator, right].concat(), text);

        for (text, joined) in [("甲乙丙a\u{301}丁戊己", "a\u{301}"), ("abc👩‍🔬def", "👩‍🔬")]
        {
            let (left, separator, right) = split(text).unwrap();
            assert!(left.contains(joined) || right.contains(joined));
            assert_eq!([left, separator, right].concat(), text);
        }
    }

    #[test]
    fn detects_short_dangling_replies_without_punishing_normal_compression() {
        let sentence =
            "The observation confirmed two separate events and researchers recorded each result. ";
        let english = sentence.repeat(10);
        let chinese = "观测证实两件事各自发生，研究人员记录了每一项结果。".repeat(10);
        assert!(!looks_incomplete(&english, &chinese, false));
        assert!(!looks_incomplete(&chinese, &english, false));
        assert!(!looks_incomplete(&english, &sentence.repeat(4), false));
        assert!(looks_incomplete(&english, "研究人员记录了结果。", false));
        assert!(looks_incomplete(
            &english,
            &(sentence.repeat(5) + "The"),
            false
        ));
        assert!(!looks_incomplete("This is a short label", "短标签", false));
        assert!(!looks_incomplete(
            "English label",
            &"[[BD_F_271]] ".repeat(50),
            false,
        ));
    }

    #[test]
    fn requires_clear_disproportion_in_an_already_split_short_piece() {
        let source = "The team observed three distinct changes in temperature and recorded measurements throughout the afternoon.";
        assert!(looks_incomplete(source, "温", true));
        assert!(!looks_incomplete(source, "温", false));
        assert!(!looks_incomplete(
            source,
            "Measurements were recorded and",
            true
        ));
    }
}
