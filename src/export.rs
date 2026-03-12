use std::path::Path;

use genpdf::elements::{Break, Paragraph};
use genpdf::fonts;
use genpdf::style::{self, Style, StyledString};
use genpdf::{Document, Margins};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

const FONT_SIZE_BODY: u8 = 11;
const FONT_SIZE_H1: u8 = 24;
const FONT_SIZE_H2: u8 = 20;
const FONT_SIZE_H3: u8 = 16;
const FONT_SIZE_H4: u8 = 14;
const CODE_FONT_SIZE: u8 = 9;
const PAGE_MARGIN: f64 = 25.0;

pub fn export_to_pdf(title: &str, markdown: &str, output_path: &Path) -> Result<(), String> {
    let font_family = default_font_family();

    let mut doc = Document::new(font_family);
    doc.set_title(title);
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.4);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(Margins::trbl(PAGE_MARGIN, PAGE_MARGIN, PAGE_MARGIN, PAGE_MARGIN));
    doc.set_page_decorator(decorator);

    // Title
    let title_style = style::Style::new().bold().with_font_size(FONT_SIZE_H1);
    doc.push(Paragraph::new(StyledString::new(title.to_string(), title_style)));
    doc.push(Break::new(0.5));

    // Parse markdown and render
    let parser = Parser::new_ext(markdown, Options::all());
    render_events(&mut doc, parser);

    doc.render_to_file(output_path)
        .map_err(|e| format!("Failed to write PDF: {e}"))
}

fn default_font_family() -> fonts::FontFamily<fonts::FontData> {
    let regular = fonts::FontData::new(
        include_bytes!("../fonts/Inter-Regular.ttf").to_vec(),
        None,
    )
    .expect("Failed to load Inter Regular");

    let bold = fonts::FontData::new(
        include_bytes!("../fonts/Inter-Bold.ttf").to_vec(),
        None,
    )
    .expect("Failed to load Inter Bold");

    let italic = fonts::FontData::new(
        include_bytes!("../fonts/Inter-Italic.ttf").to_vec(),
        None,
    )
    .expect("Failed to load Inter Italic");

    let bold_italic = fonts::FontData::new(
        include_bytes!("../fonts/Inter-BoldItalic.ttf").to_vec(),
        None,
    )
    .expect("Failed to load Inter Bold Italic");

    fonts::FontFamily {
        regular,
        bold,
        italic,
        bold_italic,
    }
}

struct TextSpan {
    text: String,
    style: Style,
}

fn render_events(doc: &mut Document, parser: Parser) {
    let mut current_spans: Vec<TextSpan> = Vec::new();
    let mut style_stack: Vec<Style> = vec![Style::new().with_font_size(FONT_SIZE_BODY)];
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut _in_heading = false;
    let mut list_depth: u32 = 0;
    let mut ordered_list_index: Option<u64> = None;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    flush_paragraph(doc, &mut current_spans);
                    let size = match level {
                        HeadingLevel::H1 => FONT_SIZE_H1,
                        HeadingLevel::H2 => FONT_SIZE_H2,
                        HeadingLevel::H3 => FONT_SIZE_H3,
                        _ => FONT_SIZE_H4,
                    };
                    style_stack.push(Style::new().bold().with_font_size(size));
                    _in_heading = true;
                }
                Tag::Paragraph => {
                    style_stack.push(Style::new().with_font_size(FONT_SIZE_BODY));
                }
                Tag::Strong => {
                    let base = current_style(&style_stack);
                    style_stack.push(base.bold());
                }
                Tag::Emphasis => {
                    let base = current_style(&style_stack);
                    style_stack.push(base.italic());
                }
                Tag::CodeBlock(_) => {
                    flush_paragraph(doc, &mut current_spans);
                    in_code_block = true;
                    code_block_content.clear();
                }
                Tag::List(start) => {
                    flush_paragraph(doc, &mut current_spans);
                    list_depth += 1;
                    ordered_list_index = start;
                }
                Tag::Item => {
                    flush_paragraph(doc, &mut current_spans);
                    let prefix = if let Some(ref mut idx) = ordered_list_index {
                        let s = format!("{}. ", idx);
                        *idx += 1;
                        s
                    } else {
                        "- ".to_string()
                    };
                    let indent = "    ".repeat(list_depth.saturating_sub(1) as usize);
                    current_spans.push(TextSpan {
                        text: format!("{indent}{prefix}"),
                        style: current_style(&style_stack),
                    });
                }
                Tag::BlockQuote(_) => {
                    flush_paragraph(doc, &mut current_spans);
                    let base = current_style(&style_stack);
                    style_stack.push(base.italic());
                }
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) => {
                    flush_paragraph(doc, &mut current_spans);
                    doc.push(Break::new(0.3));
                    style_stack.pop();
                    _in_heading = false;
                }
                TagEnd::Paragraph => {
                    flush_paragraph(doc, &mut current_spans);
                    if list_depth == 0 {
                        doc.push(Break::new(0.2));
                    }
                    style_stack.pop();
                }
                TagEnd::Strong | TagEnd::Emphasis => {
                    style_stack.pop();
                }
                TagEnd::CodeBlock => {
                    let code_style = Style::new().with_font_size(CODE_FONT_SIZE);
                    for line in code_block_content.lines() {
                        doc.push(Paragraph::new(StyledString::new(
                            line.to_string(),
                            code_style,
                        )));
                    }
                    doc.push(Break::new(0.2));
                    in_code_block = false;
                    code_block_content.clear();
                }
                TagEnd::List(_) => {
                    flush_paragraph(doc, &mut current_spans);
                    list_depth = list_depth.saturating_sub(1);
                    if list_depth == 0 {
                        ordered_list_index = None;
                        doc.push(Break::new(0.2));
                    }
                }
                TagEnd::Item => {
                    flush_paragraph(doc, &mut current_spans);
                }
                TagEnd::BlockQuote(_) => {
                    flush_paragraph(doc, &mut current_spans);
                    style_stack.pop();
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else {
                    current_spans.push(TextSpan {
                        text: text.to_string(),
                        style: current_style(&style_stack),
                    });
                }
            }
            Event::Code(code) => {
                let code_style = current_style(&style_stack).with_font_size(CODE_FONT_SIZE);
                current_spans.push(TextSpan {
                    text: code.to_string(),
                    style: code_style,
                });
            }
            Event::SoftBreak | Event::HardBreak => {
                flush_paragraph(doc, &mut current_spans);
            }
            Event::Rule => {
                flush_paragraph(doc, &mut current_spans);
                doc.push(Break::new(0.3));
                doc.push(Paragraph::new("---"));
                doc.push(Break::new(0.3));
            }
            _ => {}
        }
    }

    flush_paragraph(doc, &mut current_spans);
}

fn current_style(stack: &[Style]) -> Style {
    stack.last().copied().unwrap_or_else(|| Style::new().with_font_size(FONT_SIZE_BODY))
}

fn flush_paragraph(doc: &mut Document, spans: &mut Vec<TextSpan>) {
    if spans.is_empty() {
        return;
    }

    let mut para = Paragraph::default();
    for span in spans.drain(..) {
        para.push(StyledString::new(span.text, span.style));
    }

    doc.push(para);
}
