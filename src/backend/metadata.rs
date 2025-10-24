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
    match s.char_indices().nth(max_chars) {
        None => s.to_string(),
        Some((idx, _)) => {
            let mut truncated_string = s[..idx - 3].to_string();
            truncated_string.push_str("...");
            truncated_string
        }
    }
}

pub fn set_metadata(
    body: &[u8],
    host: &str,
    path: &str,
    title: &str,
    desc: &str,
    page_type: &str,
) -> Vec<u8> {
    let desc = truncate(desc, 160).replace('\n', " ");

    let base_url = if path.is_empty() {
        format!("https://{}/", host)
    } else {
        format!("https://{}/{}", host, path)
    };

    let metadata = format!(
        r#"<meta content="{0}" property="og:url" />
            <link href="{0}" rel="canonical" />
            <title>{1}</title>
            <meta content="{2}" name="description" />
            <meta content="{1}" property="og:title" />
            <meta content="{2}" property="og:description" />
            <meta content="https://{3}/_/raw/social-image.jpg" property="og:image" />
            <meta content="image/jpeg" property="og:image:type" />
            <meta content="1200" property="og:image:width" />
            <meta content="630" property="og:image:height" />
            <meta content="{1}" property="twitter:title" />
            <meta content="{2}" property="twitter:description" />
            <meta content="summary_large_image" property="twitter:card" />
            <meta content="https://{3}/_/raw/social-image.jpg" property="twitter:image" />
            <meta content="{4}" property="og:type" />"#,
        base_url, title, desc, host, page_type
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
}
