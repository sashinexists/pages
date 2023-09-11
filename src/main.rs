use std::fs;
mod ui;
use ui::*;
fn main() {
    let document = (Document::new("Test Page")).add_style(Style::BackgroundColor(Color::Black));

    let content = column()
        .add_style(Style::Width(Unit::Percent(90.0)))
        .add_style(Style::Center)
        .add_style(Style::BackgroundColor(Color::Grey))
        .add_style(Style::TextColor(Color::White))
        .add_style(Style::Rounded(Unit::Px(15)))
        .add_style(Style::Padding(Unit::Px(30)))
        .push(row().push(text("hello").add_style(Style::TextColor(Color::Yellow))))
        .push(row().push(text("my").add_style(Style::BackgroundColor(Color::Orange))))
        .push(row().push(text("name")))
        .push(row().push(text("is").add_style(Style::BackgroundColor(Color::Blue))))
        .push(row().push(text("chara").add_style(Style::TextColor(Color::Red))));

    let document = document.push(content);

    let output = document.to_html();
    dbg!(document);
    fs::write("index.html", output).expect("Failed to write to index.html");
}

#[derive(Debug, Clone)]
struct Document {
    title: String,
    styles: Vec<Style>,
    content: Vec<Element>,
}

impl Document {
    fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            styles: Vec::new(),
            content: Vec::new(),
        }
    }

    fn push(mut self, element: Element) -> Self {
        self.content.push(element);
        self
    }

    fn get_inline_style_string(&self) -> String {
        self.styles
            .iter()
            .fold(String::new(), |style_string, style| {
                style_string + &style.to_string()
            })
    }

    fn get_elements_html(&self) -> String {
        self.content.iter().fold("".to_string(), |output, element| {
            output + &element.to_html()
        })
    }

    pub fn add_style(mut self, style: Style) -> Self {
        self.styles.push(style);
        self
    }

    pub fn add_styles(mut self, styles: Vec<Style>) -> Self {
        self.styles.extend(styles);
        self
    }

    fn to_html(&self) -> String {
        format!(
            "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>{}</title>
</head>
<body style=\"{}\">
{}
</body>
</html>",
            self.title,
            self.get_inline_style_string(),
            self.get_elements_html()
        )
    }
}
