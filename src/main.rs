use std::fs;
use std::path::PathBuf;
mod html;
use html::*;
mod ui;
use ui::*;
fn main() {
    let mut pages = Pages::new();

    let home = (Document::new("Test Page", "index.html"))
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

    let page_title: Element = heading(HeadingLevel::H1, "Sashin Dev")
        .add_style(Style::FontWeight(FontWeight::Light))
        .add_style(Style::TextColor(colors::MIDDLE_GREEN))
        .add_style(Style::FontSize(Unit::Px(36)));
    let mut main = column().add_style(Style::Center);
    let page_header: Element = row()
        .add_style(Style::SpaceBetween)
        .add_style(Style::Height(Unit::Px(50)))
        .push(page_title)
        .push(
            row()
                .push(header_link("Past Work", "/past-work"))
                .push(header_link("Skills", "/skills"))
                .push(header_link("Testimonials", "/testimonials"))
                .push(header_link("Writing", "/writing"))
                .push(header_link("Now", "/now")),
        )
        .add_style(Style::Height(Unit::Percent(100.0)))
        .add_style(Style::FontSize(Unit::Px(13)));

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

    let home = home.push(main);
    pages.add(home);
    pages.write_html();
    pages.write_css();
}

#[derive(Debug, Clone)]
struct Pages(Vec<Document>);

impl Pages {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, page: Document) -> Self {
        self.0.push(page);
        self.clone()
    }

    pub fn write_html(&mut self) {
        self.0.iter().for_each(|page| page.write_html());
    }

    //this runs for each page, you will need to fix this when you have more pages
    pub fn write_css(&mut self) {
        self.0.iter().for_each(|page| page.write_css());
    }
}

#[derive(Debug, Clone)]
struct Document {
    title: String,
    styles: Vec<Style>,
    content: Vec<Element>,
    path: PathBuf,
}

impl Document {
    fn new(title: &str, path: &str) -> Self {
        Self {
            title: title.to_string(),
            styles: Vec::new(),
            content: Vec::new(),
            path: PathBuf::from(path),
        }
    }

    fn write_html(&self) {
        fs::write(&self.path, self.to_html()).expect(&format!(
            "Failed to write document to {}",
            &self.path.display()
        ));
    }

    fn write_css(&self) {
        let stylesheet: Stylesheet = Stylesheet::from_document(self);
        let css = stylesheet.to_css();
        fs::write("style.css", css).expect(&format!("Failed to write stylesheet"));
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
        // self.content.iter().fold("".to_string(), |output, element| {
        //     output + &element.to_html()
        // })
        self.content.iter().fold("".to_string(), |output, element| {
            let tag: Tag = match &element.content {
                ElementContent::Column(_) => Tag::Div,
                ElementContent::Row(_) => Tag::Div,
                ElementContent::Text(_) => Tag::Span,
                ElementContent::Link(_) => Tag::A,
                ElementContent::Heading(heading) => match heading.level {
                    HeadingLevel::H1 => Tag::H1,
                    HeadingLevel::H2 => Tag::H2,
                    HeadingLevel::H3 => Tag::H3,
                    HeadingLevel::H4 => Tag::H4,
                    HeadingLevel::H5 => Tag::H5,
                    HeadingLevel::H6 => Tag::H6,
                },
            };
            output + &HtmlElement::from_element(element, tag).write_html()
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
    <link rel=\"stylesheet\" type=\"text/css\" href=\"style.css\">
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

#[derive(Debug, Clone)]
struct CSSRuleSet {
    selector: String,
    styles: Vec<Style>,
}

impl CSSRuleSet {
    fn to_css(&self) -> String {
        format!(
            "{}{{{}}}",
            ".".to_string() + &self.selector,
            self.styles
                .iter()
                .map(|style| style.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }

    fn from_style(style: &Style) -> Self {
        Self {
            selector: style.to_utility_class(),
            styles: vec![style.clone()],
        }
    }
}

#[derive(Debug, Clone)]
struct Stylesheet(Vec<CSSRuleSet>);

impl Stylesheet {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn to_css(&self) -> String {
        self.0
            .iter()
            .map(|rule| rule.to_css())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn from_document(document: &Document) -> Self {
        document
            .content
            .iter()
            .fold(Self::new(), |mut stylesheet, element| {
                stylesheet
                    .0
                    .extend(Stylesheet::from_element(element, Stylesheet::new()).0);
                stylesheet
            })
    }

    fn from_element(element: &Element, mut reducer: Self) -> Self {
        reducer.0.extend(
            element
                .meta
                .styles
                .iter()
                .map(|style| CSSRuleSet::from_style(style)),
        );
        match element.content.clone() {
            ElementContent::Column(column) => {
                let element_stylesheet = column
                    .elements
                    .iter()
                    .fold(Stylesheet::new(), |stylesheet, element| {
                        Self::from_element(element, stylesheet)
                    });
                reducer.0.extend(element_stylesheet.0);
                reducer
            }
            ElementContent::Row(row) => {
                let element_stylesheet = row
                    .elements
                    .iter()
                    .fold(Stylesheet::new(), |stylesheet, element| {
                        Self::from_element(element, stylesheet)
                    });
                reducer.0.extend(element_stylesheet.0);
                reducer
            }
            _ => reducer,
        }
    }
}

// This is not a part of the actual framework, it's an example of the framework being used

fn header_link(label: &str, target: &str) -> Element {
    link(text(label), target)
        .add_style(Style::Padding(Unit::Px(15)))
        .add_style(Style::Height(Unit::Percent(100.0)))
        .add_style(Style::TextColor(colors::DARK_MEDIUM_GRAY))
}

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
