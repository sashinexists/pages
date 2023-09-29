use std::{fs, path::PathBuf};

use crate::{
    custom::datatypes::PageData,
    html::{HtmlElement, Tag},
    ui::{Element, ElementContent, HeadingLevel, Style},
};

pub struct Site {
    pub pages: Pages,
    pub title: String,
    pub home: Page,
    pub global_styles: Vec<Style>,
}

impl Site {
    pub fn new(home: Page, title: &str) -> Self {
        Self {
            pages: Pages(vec![]),
            title: title.to_string(),
            home,
            global_styles: Vec::new(),
        }
    }

    pub fn add_global_styles(&mut self, styles: &[Style]) {
        self.global_styles.extend_from_slice(styles);
        self.home.add_styles(styles);
        self.pages.0.iter_mut().for_each(|page| {
            page.add_styles(styles);
        });
    }

    pub fn publish(&self) {
        self.pages.publish();
        self.home.publish();
    }
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

    pub fn write_html(&self) {
        self.0.iter().for_each(|page| page.write_html());
    }

    //this runs for each page, you will need to fix this when you have more pages
    pub fn write_css(&self) {
        self.0.iter().for_each(|page| page.write_css());
    }
    pub fn publish(&self) {
        self.write_html();
        self.write_css();
        println!("Successfully published site");
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub styles: Vec<Style>,
    pub content: Vec<Element>,
    pub path: PathBuf,
    pub data: PageData,
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
        println!(
            "Successfully published html for page '{}' to {:?}",
            self.title, self.path
        );
    }

    fn write_css(&self) {
        let css = self.get_css();
        fs::write(".public/style.css", css).expect(&format!("Failed to write stylesheet"));
        println!("Successfully published css for page {}", self.title);
    }

    pub fn publish(&self) {
        self.write_html();
        self.write_css();
    }

    pub fn push(&mut self, element: Element) -> Self {
        self.content.push(element);
        self.clone()
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

    pub fn add_styles(&mut self, styles: &[Style]) -> Self {
        self.styles.extend(
            styles
                .iter()
                .map(|style| style.clone())
                .collect::<Vec<Style>>(),
        );
        self.clone()
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
