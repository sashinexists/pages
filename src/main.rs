use std::fs;
mod ui;
use ui::*;
fn main() {
    let document = (Document::new("Test Page", "index.html"))
        .add_style(Style::BackgroundColor(colors::RICH_BLACK))
        .add_style(Style::Margin(Unit::Px(0)))
        .add_style(Style::PaddingEach(Sides::new(
            Unit::Px(10),
            Unit::Px(20),
            Unit::Px(20),
            Unit::Px(20),
        )))
        .add_style(Style::Width(Unit::Percent(100.0)))
        .add_style(Style::Font("Ubuntu".to_string()))
        .add_style(Style::TextColor(colors::DARK_MEDIUM_GRAY));

    let page_title: Element = heading(HeadingLevel::H1, "Sashin Exists")
        .add_style(Style::FontWeight(FontWeight::Light))
        .add_style(Style::TextColor(colors::MIDDLE_GREEN))
        .add_style(Style::FontSize(Unit::Px(36)));
    let mut main = column().add_style(Style::Center);
    let page_header: Element = row()
        .add_style(Style::SpaceBetween)
        .add_style(Style::Padding(Unit::Px(15)))
        .add_style(Style::Height(Unit::Px(50)))
        .push(page_title)
        .push(row().push(Text::new("About")));

    let content = column()
        .add_style(Style::Width(Unit::Px(900)))
        .add_style(Style::Center)
        .add_style(Style::BackgroundColor(colors::EERIE_BLACK))
        .add_style(Style::TextColor(colors::DARK_MEDIUM_GRAY))
        .add_style(Style::Rounded(Unit::Px(10)))
        .add_style(Style::Padding(Unit::Px(30)))
        .push(row().push(text("hello").add_style(Style::TextColor(Color::new(255, 255, 0, 1.0)))))
        .push(row().push(text("my").add_style(Style::BackgroundColor(Color::new(200, 0, 0, 1.0)))))
        .push(row().push(text("name")))
        .push(row().push(text("is").add_style(Style::BackgroundColor(Color::new(0, 0, 200, 1.0)))))
        .push(row().push(text("chara").add_style(Style::TextColor(Color::new(250, 0, 0, 1.0)))));

    let main = main.push(page_header).push(content).push(column());

    let document = document.push(main);

    dbg!(&document);
    document.write();
}

//change this so the path isn't stored in a string but something more strict
#[derive(Debug, Clone)]
struct Document {
    title: String,
    styles: Vec<Style>,
    content: Vec<Element>,
    path: String,
}

impl Document {
    fn new(title: &str, path: &str) -> Self {
        Self {
            title: title.to_string(),
            styles: Vec::new(),
            content: Vec::new(),
            path: path.to_string(),
        }
    }

    fn write(&self) {
        fs::write(&self.path, self.to_html())
            .expect(&format!("Failed to write document to {}", &self.path));
    }

    fn push(mut self, element: Element) -> Self {
        self.content.push(element);
        self
    }

    pub fn push_elements(mut self, elements: Vec<Element>) -> Self {
        self.content.extend(elements);
        self
    }

    fn get_inline_style_string(&self) -> String {
        self.styles
            .iter()
            .map(|style| style.to_string())
            .collect::<Vec<String>>()
            .join("")
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
<body style=\"box-sizing:border-box;{}\">
{}
</body>
</html>",
            self.title,
            self.get_inline_style_string(),
            self.get_elements_html()
        )
    }
}

// This is not a part of the actual framework, it's an example of the framework being used
mod colors {

    use crate::Color; // if Color is defined in another module but in the same crate
    pub const RICH_BLACK: Color = Color::new(3, 3, 3, 1.0);
    pub const EERIE_BLACK: Color = Color::new(23, 23, 23, 1.0);
    pub const EERIE_BLACK_LIGHTEST: Color = Color::new(45, 45, 45, 1.0);
    pub const EERIE_BLACK_LIGHTEST_TRANSPARENT: Color = Color::new(45, 45, 45, 0.9);
    pub const EERIE_BLACK_LIGHTER: Color = Color::new(36, 36, 36, 1.0);
    pub const EERIE_BLACK_LIGHTER_TRANSPARENT: Color = Color::new(36, 36, 36, 0.9);
    pub const EERIE_BLACK_DARKER: Color = Color::new(18, 18, 18, 1.0);
    pub const EERIE_BLACK_DARKER_TRANSPARENT: Color = Color::new(18, 18, 18, 0.9);
    pub const CHARLESTON_GREEN: Color = Color::new(44, 44, 44, 1.0);
    pub const DARK_MEDIUM_GRAY: Color = Color::new(170, 170, 170, 1.0);
    pub const PLATINUM: Color = Color::new(233, 233, 233, 1.0);
    pub const MIDDLE_GREEN: Color = Color::new(82, 170, 94, 1.0);
    pub const TURQUOISE_GREEN: Color = Color::new(160, 208, 167, 1.0);
    pub const AMARANTH: Color = Color::new(239, 45, 86, 1.0);
}
