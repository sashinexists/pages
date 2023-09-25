mod custom;
use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::PathBuf;
mod html;
use custom::api::*;
use custom::components::*;
use custom::datatypes::PageData;
use custom::theme::*;
use html::*;
mod ui;
use ui::*;

use crate::custom::datatypes::Projects;
use crate::custom::datatypes::View;
mod id;

fn main() {
    dotenv().expect("Failed to read .env file"); // This line loads the .env file into environment variables
    let mut pages = Pages::new();
    let home = (Page::new("Sashin Dev", ".public/index.html", PageData::None))
        .add_style(Style::BackgroundColor(colors::RICH_BLACK))
        .add_style(Style::Margin(Unit::Px(0)))
        .add_style(Style::Width(Unit::Percent(100.0)))
        .add_style(Style::Font("Ubuntu".to_string()))
        .add_style(Style::TextColor(colors::DARK_MEDIUM_GRAY));

    let page_title: Element = heading(HeadingLevel::H1, "Sashin Dev")
        .add_style(Style::FontWeight(FontWeight::Light))
        .add_style(Style::TextColor(colors::MIDDLE_GREEN))
        .add_style(Style::FontSize(Unit::Px(36)))
        .add_hover_style(Style::TextColor(colors::TURQUOISE_GREEN));

    let mut main = column().add_style(Style::Center);

    let page_header: Element = row()
        .add_style(Style::JustifyContent(JustifyContent::SpaceBetween))
        .add_style(Style::Width(Unit::Percent(100.0)))
        .add_style(Style::PaddingEach(Sides::new(
            Unit::Px(5),
            Unit::Px(5),
            Unit::Px(20),
            Unit::Px(20),
        )))
        .push(link(page_title, "https://sashin.dev").add_style(Style::NoUnderline))
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

    let banner = column()
        .add_style(Style::BackgroundImage(Image {
            src: "'assets/images/banner.jpg'".to_string(),
            alt: "banner-image".to_string(),
        }))
        .add_style(Style::Width(Unit::Percent(100.0)))
        .add_style(Style::Height(Unit::Px(300)))
        .add_style(Style::JustifyContent(JustifyContent::End))
        .push(
            row()
                .add_style(Style::Width(Unit::Percent(100.0)))
                .add_style(Style::BackgroundColor(
                    colors::EERIE_BLACK_DARKER_TRANSPARENT,
                ))
                .add_style(Style::PaddingEach(Sides::new(
                    Unit::Px(20),
                    Unit::Px(20),
                    Unit::Px(0),
                    Unit::Px(0),
                )))
                .push(
                    text("Crafting better software for creators and innovators")
                        .add_style(Style::FontSize(Unit::Px(30)))
                        .add_style(Style::FontWeight(FontWeight::ExtraLight))
                        .add_style(Style::TextAlign(TextAlign::Center))
                        .add_style(Style::Width(Unit::Percent(100.0))),
                ),
        );

    const INTRO_TEXT:&'static str = "My name is Sashin, and I help ambitious and creative individuals and organisations design and build their dream websites.\n\nI work directly with clients to bring their vision to life, getting to know them, their mission and brand, and create websites that reflect them.";
    // following three lets are for testing
    let access_token = env::var("CONTENTFUL_CONTENT_DELIVERY_API_ACCESS_TOKEN")
        .expect("CONTENTFUL_ACCESS_TOKEN not found");
    let space_id = env::var("CONTENTFUL_SPACE_ID").expect("CONTENTFUL_SPACE_ID not found");
    let past_projects_data =
        get_past_projects_data(&access_token, &space_id).expect("Failed to get projects data");
    let projects: Projects = Projects::from_items(&access_token, &space_id, past_projects_data);
    let projects_view = projects.view();

    let content = column()
        .add_style(Style::Width(Unit::Px(700)))
        .add_style(Style::Center)
        .add_style(Style::BackgroundColor(colors::EERIE_BLACK))
        .add_style(Style::RoundedEach(Corners::new(
            Unit::Px(0),
            Unit::Px(0),
            Unit::Px(15),
            Unit::Px(15),
        )))
        .add_style(Style::Padding(Unit::Px(15)))
        .add_style(Style::JustifyContent(JustifyContent::Start))
        .push(introduction(
            "Rust Developer at your service",
            &INTRO_TEXT,
            "assets/images/now-banner.jpg",
            "Example Banner",
        ))
        .push(projects_view);

    let main = main
        .push(page_header)
        .push(banner)
        .push(content)
        .push(footer());

    let home = home.push(main);
    pages.add(home);
    pages.write_html();
    pages.write_css();
}

struct Site {
    pages: Pages,
    title: String,
    index: Page,
    global_styles: Vec<Style>,
}

#[derive(Debug, Clone)]
pub struct Pages(Vec<Page>);

impl Pages {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, page: Page) -> Self {
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
pub struct Page {
    title: String,
    styles: Vec<Style>,
    content: Vec<Element>,
    path: PathBuf,
    data: PageData,
}

impl Page {
    pub fn new(title: &str, path: &str, data: PageData) -> Self {
        Self {
            title: title.to_string(),
            styles: Vec::new(),
            content: Vec::new(),
            path: PathBuf::from(path),
            data,
        }
    }

    fn write_html(&self) {
        fs::write(&self.path, self.to_html()).expect(&format!(
            "Failed to write document to {}",
            &self.path.display()
        ));
    }

    fn write_css(&self) {
        let css = self.get_css();
        fs::write(".public/style.css", css).expect(&format!("Failed to write stylesheet"));
    }

    pub fn push(mut self, element: Element) -> Self {
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

    fn get_css(&self) -> String {
        self.content.iter().fold("".to_string(), |output, element| {
            let tag: Tag = match &element.content {
                ElementContent::Column(_) => Tag::Div,
                ElementContent::Row(_) => Tag::Div,
                ElementContent::Text(_) => Tag::Span,
                ElementContent::Link(_) => Tag::A,
                ElementContent::Image(_) => Tag::IMG,
                ElementContent::Heading(heading) => match heading.level {
                    HeadingLevel::H1 => Tag::H1,
                    HeadingLevel::H2 => Tag::H2,
                    HeadingLevel::H3 => Tag::H3,
                    HeadingLevel::H4 => Tag::H4,
                    HeadingLevel::H5 => Tag::H5,
                    HeadingLevel::H6 => Tag::H6,
                },
            };
            output + &HtmlElement::from_element(element, tag).write_css()
        })
    }

    fn get_elements_html(&self) -> String {
        let html = self.content.iter().fold("".to_string(), |output, element| {
            let tag: Tag = match &element.content {
                ElementContent::Column(_) => Tag::Div,
                ElementContent::Row(_) => Tag::Div,
                ElementContent::Text(_) => Tag::Span,
                ElementContent::Link(_) => Tag::A,
                ElementContent::Image(_) => Tag::IMG,
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
        });
        html
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
    <style>* {{box-sizing:border-box;}}</style>
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
