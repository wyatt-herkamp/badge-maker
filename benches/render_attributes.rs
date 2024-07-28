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

pub fn render_attributes_string(links: &Links, accessible_text: &str) -> String {
    if !links.any() {
        let escaped = escape_xml(accessible_text);
        let mut build = String::with_capacity(30 + escaped.len());
        build.push_str(r##"role="img" aria-label=""##);
        build.push_str(&escaped);
        build.push('"');
        build
    } else {
        "".to_string()
    }
}

pub fn render_attributes_format(links: &Links, accessible_text: &str) -> String {
    if links.any() {
        "".to_string()
    } else {
        format!(
            r##"role="img" aria-label="{}""##,
            escape_xml(accessible_text)
        )
    }
}
