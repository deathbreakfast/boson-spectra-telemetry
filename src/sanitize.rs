//! Sanitize operator-visible error strings before they are persisted in events.

/// Maximum length of persisted / logged handler error messages.
pub const MAX_ERROR_MESSAGE_CHARS: usize = 512;

fn redact_credentials_in_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < text.len() {
        if let Some(rel) = text[i..].find("://") {
            let abs = i + rel;
            let scheme_start = text[..abs]
                .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '+' || c == '.' || c == '-'))
                .map_or(i, |j| j + 1);
            out.push_str(&text[i..scheme_start]);
            let endpoint = &text[scheme_start..];
            let after_scheme = endpoint.find("://").map_or(0, |j| j + 3);
            let authority = &endpoint[after_scheme..];
            if let Some(userinfo_end) = authority.find('@') {
                if !authority[..userinfo_end].contains(['/', '?', '#']) {
                    let host_start = after_scheme + userinfo_end + 1;
                    out.push_str(&endpoint[..after_scheme]);
                    out.push_str("***@");
                    let rest = &endpoint[host_start..];
                    let end_rel = rest
                        .find(|c: char| {
                            c.is_whitespace() || c == '"' || c == '\'' || c == ')' || c == ']'
                        })
                        .unwrap_or(rest.len());
                    out.push_str(&rest[..end_rel]);
                    i = scheme_start + host_start + end_rel;
                    continue;
                }
            }
            let end_rel = endpoint[after_scheme..]
                .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ')' || c == ']')
                .unwrap_or_else(|| endpoint.len().saturating_sub(after_scheme));
            out.push_str(&endpoint[..after_scheme + end_rel]);
            i = scheme_start + after_scheme + end_rel;
        } else {
            out.push_str(&text[i..]);
            break;
        }
    }
    out
}

/// Truncate and strip obvious secret-looking substrings from an error message.
#[must_use]
pub fn sanitize_error_message(raw: &str) -> String {
    let mut out = redact_credentials_in_text(&raw.replace('\0', ""));
    for needle in [
        "password=",
        "Password=",
        "secret=",
        "Secret=",
        "token=",
        "Token=",
        "Bearer ",
        "authorization:",
    ] {
        if let Some(idx) = out.find(needle) {
            let end = (idx + needle.len() + 8).min(out.len());
            out.replace_range(idx..end, &format!("{needle}[redacted]"));
        }
    }
    if out.chars().count() > MAX_ERROR_MESSAGE_CHARS {
        out = out.chars().take(MAX_ERROR_MESSAGE_CHARS).collect();
        out.push('…');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_long_messages_sad() {
        let long = "x".repeat(800);
        let s = sanitize_error_message(&long);
        assert!(s.chars().count() <= MAX_ERROR_MESSAGE_CHARS + 1);
        assert!(s.ends_with('…'));
    }

    #[test]
    fn redacts_password_prefix_sad() {
        let s = sanitize_error_message("db failed password=hunter2 more");
        assert!(s.contains("[redacted]"));
        assert!(!s.contains("hunter2"));
    }
}
