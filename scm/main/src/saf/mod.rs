use crate::api::{AspectRatio, Deck, OverflowPolicy, PresentationError, Slide, SlideElement};

/// Parses a deterministic Markdown deck. Slides begin with a `---` marker;
/// speaker notes use a `:::notes` block terminated by the next slide marker.
pub fn parse_markdown(source: &str, aspect_ratio: AspectRatio) -> Result<Deck, PresentationError> {
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    let mut slides = Vec::new();
    let mut title = String::new();
    let mut resolved_aspect_ratio = aspect_ratio;
    let mut content = normalized.as_str();
    if let Some(frontmatter) = normalized.strip_prefix("+++\n") {
        if let Some(end) = frontmatter.find("\n+++") {
            let metadata = &frontmatter[..end];
            for line in metadata.lines().filter(|line| !line.trim().is_empty()) {
                apply_toml_metadata_line(line, &mut title, &mut resolved_aspect_ratio)?;
            }
            content = &frontmatter[end + 4..];
        } else {
            return Err(PresentationError::MalformedSource(
                "unterminated TOML presentation front matter".to_string(),
            ));
        }
    }
    if let Some(frontmatter) = normalized.strip_prefix("---\n") {
        if let Some(end) = frontmatter.find("\n---") {
            let metadata = &frontmatter[..end];
            let recognized = metadata.lines().all(|line| {
                let key = line.split_once(':').map(|(key, _)| key.trim());
                matches!(key, Some("title" | "aspect_ratio"))
            });
            if recognized && metadata.lines().any(|line| !line.trim().is_empty()) {
                for line in metadata.lines() {
                    apply_metadata_line(line, &mut title, &mut resolved_aspect_ratio)?;
                }
                content = &frontmatter[end + 4..];
            }
        }
    }
    let mut current = Vec::new();
    let mut in_code = false;
    let mut in_notes = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if slides.is_empty() && current.is_empty() {
            if trimmed.starts_with("title:") {
                apply_metadata_line(trimmed, &mut title, &mut resolved_aspect_ratio)?;
                continue;
            }
            if trimmed.starts_with("aspect_ratio:") {
                apply_metadata_line(trimmed, &mut title, &mut resolved_aspect_ratio)?;
                continue;
            }
        }
        if trimmed == "---" && !in_code && !in_notes {
            if !current.is_empty() {
                slides.push(Slide {
                    elements: std::mem::take(&mut current),
                });
            }
            continue;
        }
        if trimmed.eq_ignore_ascii_case(":::notes") && !in_code {
            in_notes = true;
            continue;
        }
        if in_notes {
            if trimmed == ":::" {
                in_notes = false;
            } else {
                current.push(SlideElement::Notes(line.to_string()));
            }
            continue;
        }
        if trimmed.starts_with("```") {
            in_code = !in_code;
            if !in_code {
                current.push(SlideElement::Code(String::new()));
            }
            continue;
        }
        if in_code {
            match current.last_mut() {
                Some(SlideElement::Code(text)) => {
                    if !text.is_empty() {
                        text.push('\n');
                    }
                    text.push_str(line);
                }
                _ => current.push(SlideElement::Code(line.to_string())),
            }
            continue;
        }
        if let Some(text) = trimmed.strip_prefix("# ") {
            current.push(SlideElement::Heading {
                level: 1,
                text: text.to_string(),
            });
        } else if let Some(text) = trimmed.strip_prefix("## ") {
            current.push(SlideElement::Heading {
                level: 2,
                text: text.to_string(),
            });
        } else if !trimmed.is_empty() {
            current.push(SlideElement::Paragraph(trimmed.to_string()));
        }
    }
    if in_code || in_notes {
        return Err(PresentationError::MalformedSource(
            "unterminated presentation block".to_string(),
        ));
    }
    if !current.is_empty() {
        slides.push(Slide { elements: current });
    }
    if slides.is_empty() {
        return Err(PresentationError::EmptyDeck);
    }
    Ok(Deck {
        title,
        aspect_ratio: resolved_aspect_ratio,
        slides,
        overflow_policy: OverflowPolicy::Reject,
    })
}

/// Validates deterministic fixed-canvas constraints before rendering.
pub fn validate_deck(deck: &Deck) -> Result<(), PresentationError> {
    if deck.slides.is_empty() {
        return Err(PresentationError::EmptyDeck);
    }
    if deck.overflow_policy == OverflowPolicy::Clip {
        return Ok(());
    }
    for (index, slide) in deck.slides.iter().enumerate() {
        let estimated_lines = slide
            .elements
            .iter()
            .map(|element| match element {
                SlideElement::Heading { .. } => 2,
                SlideElement::Paragraph(text) => text.chars().count().div_ceil(90).max(1),
                SlideElement::Code(text) => text.lines().count().max(1),
                SlideElement::Notes(_) => 0,
            })
            .sum::<usize>();
        if estimated_lines > 34 {
            return Err(PresentationError::SlideOverflow {
                slide: index + 1,
                estimated_lines,
            });
        }
    }
    Ok(())
}

fn apply_metadata_line(
    line: &str,
    title: &mut String,
    aspect_ratio: &mut AspectRatio,
) -> Result<(), PresentationError> {
    let Some((key, value)) = line.split_once(':') else {
        return Err(PresentationError::MalformedSource(
            "metadata line must use key: value syntax".to_string(),
        ));
    };
    let value = value.trim().trim_matches(['\"', '\'']);
    match key.trim() {
        "title" => *title = value.to_string(),
        "aspect_ratio" => {
            *aspect_ratio = match value {
                "16:9" | "16x9" => AspectRatio::Widescreen16x9,
                "4:3" | "4x3" => AspectRatio::Standard4x3,
                _ => {
                    return Err(PresentationError::MalformedSource(
                        "aspect_ratio must be 16:9 or 4:3".to_string(),
                    ));
                }
            }
        }
        _ => {
            return Err(PresentationError::MalformedSource(
                "unsupported deck metadata key".to_string(),
            ))
        }
    }
    Ok(())
}

fn apply_toml_metadata_line(
    line: &str,
    title: &mut String,
    aspect_ratio: &mut AspectRatio,
) -> Result<(), PresentationError> {
    let Some((key, value)) = line.split_once('=') else {
        return Err(PresentationError::MalformedSource(
            "TOML metadata line must use key = value syntax".to_string(),
        ));
    };
    apply_metadata_line(
        &format!("{}: {}", key.trim(), value.trim()),
        title,
        aspect_ratio,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_slides_code_and_notes_in_source_order() {
        let deck = parse_markdown(
            "# First\nBody\n:::notes\nsecret\n:::\n---\n# Second\n```rust\nfn main() {}\n```",
            AspectRatio::Widescreen16x9,
        )
        .unwrap_or_else(|error| panic!("valid fixture failed to parse: {error}"));
        assert_eq!(deck.slides.len(), 2);
        assert!(deck.slides[0]
            .elements
            .iter()
            .any(|element| matches!(element, SlideElement::Notes(_))));
        assert!(deck.slides[1]
            .elements
            .iter()
            .any(|element| matches!(element, SlideElement::Code(text) if text == "fn main() {}")));
    }

    #[test]
    fn rejects_unterminated_blocks_and_empty_decks() {
        assert_eq!(
            parse_markdown("", AspectRatio::Standard4x3),
            Err(PresentationError::EmptyDeck)
        );
        assert!(matches!(
            parse_markdown("```\ncode", AspectRatio::Standard4x3),
            Err(PresentationError::MalformedSource(_))
        ));
    }

    #[test]
    fn parses_optional_metadata_without_changing_slide_order() {
        let deck = parse_markdown(
            "title: Quarterly Review\naspect_ratio: 4:3\n---\n# Slide",
            AspectRatio::Widescreen16x9,
        )
        .unwrap_or_else(|error| panic!("valid metadata fixture failed to parse: {error}"));
        assert_eq!(deck.title, "Quarterly Review");
        assert_eq!(deck.aspect_ratio, AspectRatio::Standard4x3);
        assert_eq!(deck.slides.len(), 1);
    }

    #[test]
    fn parses_toml_front_matter_metadata() {
        let deck = parse_markdown(
            "+++\ntitle = 'Quarterly Review'\naspect_ratio = '4:3'\n+++\n# Slide",
            AspectRatio::Widescreen16x9,
        )
        .unwrap_or_else(|error| panic!("valid TOML metadata fixture failed: {error}"));
        assert_eq!(deck.title, "Quarterly Review");
        assert_eq!(deck.aspect_ratio, AspectRatio::Standard4x3);
    }

    #[test]
    fn parses_yaml_style_frontmatter_before_slides() {
        let deck = parse_markdown(
            "---\ntitle: Front Matter\naspect_ratio: 4:3\n---\n# Slide",
            AspectRatio::Widescreen16x9,
        )
        .unwrap_or_else(|error| panic!("frontmatter fixture failed to parse: {error}"));
        assert_eq!(deck.title, "Front Matter");
        assert_eq!(deck.aspect_ratio, AspectRatio::Standard4x3);
        assert_eq!(deck.slides.len(), 1);
    }

    #[test]
    fn rejects_deterministically_overfull_slides() {
        let deck = parse_markdown(
            &format!("# Slide\n{}", "line\n".repeat(40)),
            AspectRatio::Widescreen16x9,
        )
        .unwrap_or_else(|error| panic!("valid overflow fixture failed to parse: {error}"));
        assert!(matches!(
            validate_deck(&deck),
            Err(PresentationError::SlideOverflow { .. })
        ));
    }
}
