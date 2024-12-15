use csslsrs::{
    css_data::CssCustomData,
    service::{LanguageService, LanguageServiceOptions},
};
use lsp_types::{
    Command, CompletionItem, CompletionItemKind, CompletionList, CompletionTextEdit, Documentation,
    InsertTextFormat, Position,
};

#[derive(Debug)]
pub struct ItemDescription {
    label: String,
    detail: Option<String>,
    documentation: Option<Documentation>,
    documentation_includes: Option<String>,
    kind: Option<CompletionItemKind>,
    insert_text_format: Option<InsertTextFormat>,
    result_text: Option<String>,
    not_available: Option<bool>,
    command: Option<Command>,
    sort_text: Option<String>,
}

pub fn assert_completion(completions: &CompletionList, expected: &ItemDescription, document: &str) {
    let matches: Vec<&CompletionItem> = completions
        .items
        .iter()
        .filter(|completion| completion.label == expected.label)
        .collect();

    if expected.not_available.is_some() {
        assert_eq!(matches.len(), 0, "{} should not be present", expected.label);
    } else {
        assert_eq!(
            matches.len(),
            1,
            "{} should only exist once: Actual: {}",
            expected.label,
            completions
                .items
                .iter()
                .map(|c| c.label.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }

    let match_item = matches[0];
    if let Some(detail) = &expected.detail {
        assert_eq!(match_item.detail, Some(detail.clone()));
    }
    if let Some(documentation) = &expected.documentation {
        assert_eq!(match_item.documentation, Some(documentation.clone()));
    }
    if let Some(documentation_includes) = &expected.documentation_includes {
        if let Some(doc) = &match_item.documentation {
            match doc {
                Documentation::String(doc) => {
                    assert!(doc.contains(documentation_includes));
                }
                Documentation::MarkupContent(doc) => {
                    assert!(doc.value.contains(documentation_includes));
                }
            }
        }
    }
    if let Some(kind) = &expected.kind {
        assert_eq!(match_item.kind, Some(*kind));
    }
    if let Some(result_text) = &expected.result_text {
        if let Some(text_edit) = &match_item.text_edit {
            let edited_text = apply_text_edit(document, text_edit);
            assert_eq!(edited_text, *result_text);
        }
    }
    if let Some(insert_text_format) = &expected.insert_text_format {
        assert_eq!(match_item.insert_text_format, Some(*insert_text_format));
    }
    if let Some(command) = &expected.command {
        assert_eq!(match_item.command, Some(command.clone()));
    }
    if let Some(sort_text) = &expected.sort_text {
        assert_eq!(match_item.sort_text, Some(sort_text.clone()));
    }
}

fn apply_text_edit(document: &str, text_edit: &CompletionTextEdit) -> String {
    let mut lines: Vec<&str> = document.lines().collect();

    match text_edit {
        CompletionTextEdit::Edit(edit) => {
            let start_line = edit.range.start.line as usize;
            let start_char = edit.range.start.character as usize;
            let end_line = edit.range.end.line as usize;
            let end_char = edit.range.end.character as usize;

            let start_line_content = &lines[start_line][..start_char];
            let end_line_content = &lines[end_line][end_char..];

            let formatted_line = format!(
                "{}{}{}",
                start_line_content, edit.new_text, end_line_content
            );
            lines[start_line] = formatted_line.as_str();
            lines[start_line + 1..=end_line]
                .iter_mut()
                .for_each(|line| *line = "");

            lines.join("\n")
        }
        CompletionTextEdit::InsertAndReplace(_) => unreachable!(),
    }
}

pub async fn test_completion_for(
    content: String,
    expected: ExpectedCompetions,
    test_uri: &str,
    workspace_folder_uri: &str,
    custom_data: Vec<CssCustomData>,
) {
    let offset = content.find('|').expect("| missing in value");
    let value = content.replace("|", "");

    let mut ls = LanguageService::new(LanguageServiceOptions {
        include_base_css_custom_data: true,
        ..Default::default()
    });

    for data in custom_data {
        ls.add_css_custom_data(data);
    }

    let document = TextDocument::create(test_uri, lang, 0, value);
    let position = Position::new(0, offset as u32);

    let list = ls.get_completions(document, position);

    if let Some(count) = expected.count {
        assert_eq!(list.items.len(), count);
    }
    if let Some(items) = expected.items {
        for item in items {
            assert_completion(&list, &item, &document);
        }
    }
}
