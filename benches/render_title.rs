use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use badge_maker::Links;
use once_cell::sync::Lazy;

const XML_ESCAPE_PATTERNS: [&str; 5] = ["&", "<", ">", "\"", "'"];
const XML_ESCAPE_REPLACEMENTS: [&str; 5] = ["&amp;", "&lt;", "&gt;", "&quot;", "&apos;"];

pub fn escape_xml(s: &str) -> String {
    static AC: Lazy<AhoCorasick> = Lazy::new(|| {
        AhoCorasickBuilder::new()
            .kind(Some(aho_corasick::AhoCorasickKind::DFA))
            .build(&XML_ESCAPE_PATTERNS)
            .unwrap()
    });
    AC.replace_all(s, &XML_ESCAPE_REPLACEMENTS)
}

pub fn render_title(links: &Links, accessible_text: &str) -> String {
    if links.any() {
        "".to_string()
    } else {
        format!("<title>{}</title>", escape_xml(accessible_text))
    }
}

pub fn render_title_new(links: &Links, accessible_text: &str) -> String {
    if links.any() {
        "".to_string()
    } else {
        let accessible_text = escape_xml(accessible_text);
        let mut buffer = String::with_capacity(15 + accessible_text.len());
        buffer.push_str(r#"<title>"#);
        buffer.push_str(&accessible_text);
        buffer.push_str(r#"</title>"#);
        buffer
    }
}
