pub(crate) fn debug_utf8(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).unwrap_or("[non-utf8]")
}

pub(crate) fn unescape_xml(input: &[u8]) -> String {
    let s = String::from_utf8_lossy(input);
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

pub(crate) fn escape_xml(input: &[u8]) -> (Vec<u8>, u8) {
    let quote = if input.contains(&b'"') { b'\'' } else { b'"' };

    let mut escaped_body = Vec::with_capacity(input.len());
    for c in input {
        match c {
            b'&' => escaped_body.extend(b"&amp;"),
            b'<' => escaped_body.extend(b"&lt;"),
            b'>' => escaped_body.extend(b"&gt;"),
            b'"' if quote == b'"' => escaped_body.extend(b"&quot;"),
            b'\'' if quote == b'\'' => escaped_body.extend(b"&apos;"),
            _ => escaped_body.push(*c),
        }
    }

    (escaped_body, quote)
}

pub(crate) fn leading_whitespaces(s: &[u8]) -> &[u8] {
    let i = s
        .iter()
        .position(|c| !c.is_ascii_whitespace())
        .unwrap_or(s.len());
    &s[..i]
}

pub(crate) fn trailing_whitespaces(s: &[u8]) -> &[u8] {
    let i = s
        .iter()
        .rev()
        .position(|c| !c.is_ascii_whitespace())
        .map(|i| s.len() - 1 - i)
        .unwrap_or(0);
    &s[i..]
}
