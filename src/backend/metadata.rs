use crate::env::config::CONFIG;

pub fn set_index_metadata(body: &[u8]) -> Vec<u8> {
    let domain = CONFIG.domains.first().cloned().expect("no domains");

    set_metadata(
        body,
        domain,
        "",
        CONFIG.name,
        "Break free from the algorithm.",
        "website",
    )
}

fn truncate(s: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    if s.chars().count() <= max_chars {
        return s.to_string();
    }

    let keep = max_chars.saturating_sub(1);
    let mut truncated = String::with_capacity(max_chars);
    truncated.extend(s.chars().take(keep));
    truncated.push('…');
    truncated
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn set_metadata(
    body: &[u8],
    host: &str,
    path: &str,
    title: &str,
    desc: &str,
    page_type: &str,
) -> Vec<u8> {
    let norm_path = path.trim_start_matches('/');
    let base_url = if norm_path.is_empty() {
        format!("https://{}/", host)
    } else {
        format!("https://{}/{}", host, norm_path)
    };

    let desc_clean = desc.replace('\n', " ");
    let desc = escape_attr(&truncate(&desc_clean, 160));
    let title = escape_attr(title);

    let metadata = format!(
        r#"<meta content="{base_url}" property="og:url" />
            <link href="{base_url}" rel="canonical" />
            <title>{title}</title>
            <meta content="{desc}" name="description" />
            <meta content="{title}" property="og:title" />
            <meta content="{desc}" property="og:description" />
            <meta content="https://{host}/_/raw/social-image.jpg" property="og:image" />
            <meta content="image/jpeg" property="og:image:type" />
            <meta content="1200" property="og:image:width" />
            <meta content="630" property="og:image:height" />
            <meta content="{title}" name="twitter:title" />
            <meta content="{desc}" name="twitter:description" />
            <meta content="summary_large_image" name="twitter:card" />
            <meta content="https://{host}/_/raw/social-image.jpg" name="twitter:image" />
            <meta content="{page_type}" property="og:type" />"#,
        base_url = base_url,
        title = title,
        desc = desc,
        host = host,
        page_type = page_type
    )
    .replace('\n', "");

    let html = String::from_utf8_lossy(body);
    // Assets built locally may contain either `<meta .../>` or `<meta ... />`
    // depending on the minifier, so replace both variants.
    html.replace(r#"<meta name="mark" content="OG"/>"#, &metadata)
        .replace(r#"<meta name="mark" content="OG" />"#, &metadata)
        .as_bytes()
        .to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_metadata_uses_clean_canonical_urls() {
        let body = br#"<html><head><meta name="mark" content="OG"/></head></html>"#;
        let result = set_metadata(body, "example.com", "post/42", "Title", "Desc", "article");
        let output = String::from_utf8(result).expect("valid utf8");
        assert!(output.contains(
            r#"<meta content="https://example.com/post/42" property="og:url" />"#
        ));
        assert!(output.contains(
            r#"<link href="https://example.com/post/42" rel="canonical" />"#
        ));
        assert!(
            !output.contains("#/"),
            "metadata should not emit hash-based canonical URLs"
        );
    }

    #[test]
    fn set_metadata_replaces_placeholder_with_space_variant() {
        let body = br#"<html><head><meta name="mark" content="OG" /></head></html>"#;
        let result = set_metadata(body, "example.com", "", "Home", "Desc", "website");
        let output = String::from_utf8(result).expect("valid utf8");
        assert!(
            output.contains(r#"<meta content="https://example.com/" property="og:url" />"#),
            "placeholder with space before slash should also be replaced"
        );
        assert!(
            !output.contains(r#"<meta name="mark" content="OG" />"#),
            "original placeholder should be removed"
        );
    }

    #[test]
    fn truncate_adds_ellipsis_when_needed() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello", 5), "hello");
        assert_eq!(truncate("hello", 4), "hel…");
        assert_eq!(truncate("hello", 1), "…");
        assert_eq!(truncate("hello", 0), "");
    }

    #[test]
    fn set_metadata_escapes_attributes_and_normalizes_path() {
        let body = br#"<html><head><meta name="mark" content="OG"/></head></html>"#;
        let result = set_metadata(
            body,
            "example.com",
            "/post/42",
            r#"<Title & co>"#,
            "Line 1\nLine 2 & more",
            "article",
        );
        let output = String::from_utf8(result).expect("valid utf8");
        assert!(
            output.contains(r#"<meta content="https://example.com/post/42" property="og:url" />"#),
            "path should be normalized without double slash"
        );
        assert!(
            output.contains(r#"<title>&lt;Title &amp; co&gt;</title>"#),
            "title must be HTML-escaped"
        );
        assert!(
            output.contains(r#"<meta content="Line 1 Line 2 &amp; more" property="og:description" />"#),
            "description must be sanitized and newlines replaced"
        );
        assert!(
            output.contains(r#"<meta content="&lt;Title &amp; co&gt;" name="twitter:title" />"#),
            "twitter meta tags should use name= attributes with escaped values"
        );
    }
}
