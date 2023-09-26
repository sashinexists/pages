use super::Page;
use crate::ui::{Element, ElementContent, HeadingLevel, Style};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct HtmlElement {
    tag: Tag,
    attributes: HashMap<String, String>,
    classes: Vec<String>,
    id: String,
    is_self_closing: bool,
    inner: HtmlInner,
    styles: Vec<Style>,
    hover_styles: Vec<Style>,
}

impl HtmlElement {
    pub fn from_element(element: &Element, tag: Tag) -> Self {
        // Common properties
        let id = element.id.clone();
        let classes = element.meta.classes.clone();
        let styles = element.meta.styles.clone();
        let attributes = element.meta.attributes.clone();
        let hover_styles = element.meta.hover_styles.clone();
        let is_self_closing = match tag {
            Tag::IMG => true,
            _ => false,
        };

        // Specific HtmlInner based on ElementContent
        let inner = match &element.content {
            ElementContent::Column(column) => HtmlInner::Children(
                column
                    .elements
                    .iter()
                    .map(|el| Self::from_element(el, el.get_tag()))
                    .collect(),
            ),
            ElementContent::Row(row) => HtmlInner::Children(
                row.elements
                    .iter()
                    .map(|el| Self::from_element(el, el.get_tag()))
                    .collect(),
            ),
            ElementContent::Text(text) => HtmlInner::Content(text.content.clone()),
            ElementContent::Link(link) => {
                HtmlInner::Children(vec![Self::from_element(&link.label, link.label.get_tag())])
            }
            ElementContent::Heading(heading) => HtmlInner::Content(vec![heading.content.clone()]),
            ElementContent::Image(_) => HtmlInner::None,
        };

        Self {
            id,
            attributes,
            is_self_closing,
            tag,
            classes,
            styles,
            inner,
            hover_styles,
        }
    }

    pub fn write_html(&self) -> String {
        if self.is_self_closing {
            format!(
                "<{} id='{}' {} {}/>",
                self.tag,
                self.id,
                self.get_attribute_string(),
                self.get_class_string()
            )
        } else {
            format!(
                "<{} id='{}' {} {}>{}</{}>",
                self.tag,
                self.id,
                self.get_attribute_string(),
                self.get_class_string(),
                self.inner.write_html(),
                self.tag
            )
        }
    }

    pub fn write_css(&self) -> String {
        self.build_stylesheet().to_css()
    }

    fn build_stylesheet(&self) -> Stylesheet {
        Stylesheet::from_html_element(self)
    }

    fn get_attribute_string(&self) -> String {
        self.attributes
            .iter()
            .map(|(k, v)| format!("{k}='{v}'"))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn get_class_string(&self) -> String {
        if &self.classes.len() > &0 {
            "class='".to_string() + &self.classes.join(" ") + "'"
        } else {
            String::new()
        }
    }
}

#[derive(Debug, Clone)]
enum HtmlInner {
    Children(Vec<HtmlElement>),
    Content(Vec<String>),
    None,
}

impl HtmlInner {
    pub fn write_html(&self) -> String {
        match self {
            Self::Children(children) => children
                .iter()
                .map(|child| child.write_html())
                .collect::<Vec<String>>()
                .join("\n"),
            Self::Content(content) => paragraphs_to_html(content.clone()),
            Self::None => "".to_string(),
        }
    }
}

fn paragraphs_to_html(paragraphs: Vec<String>) -> String {
    // inserts the text content into a span if there is only one item, or into paragraphs if there are multiple
    if paragraphs.len() == 1 {
        paragraphs[0].to_string()
    } else {
        paragraphs
            .iter()
            .map(|paragraph| {
                format!("<p {{attributes}} class=~~classes~~ style=~~styles~~>{paragraph}</p>")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
pub enum Tag {
    Div,
    Span,
    A,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    IMG,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::Div => write!(f, "div"),
            Tag::Span => write!(f, "span"),
            Tag::A => write!(f, "a"),
            Tag::H1 => write!(f, "h1"),
            Tag::H2 => write!(f, "h2"),
            Tag::H3 => write!(f, "h3"),
            Tag::H4 => write!(f, "h4"),
            Tag::H5 => write!(f, "h5"),
            Tag::H6 => write!(f, "h6"),
            Tag::IMG => write!(f, "img"),
        }
    }
}

#[derive(Debug, Clone)]
struct CSSRuleSet(String, Vec<Style>);

impl CSSRuleSet {
    fn to_css(&self) -> String {
        let (selector, styles) = (self.0.clone(), self.1.clone());
        format!(
            "{}{{{}}}",
            selector,
            styles
                .iter()
                .map(|style| style.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[derive(Debug, Clone)]
struct Stylesheet(HashMap<String, Vec<Style>>);

impl Stylesheet {
    pub fn new() -> Self {
        Self(HashMap::<String, Vec<Style>>::new())
    }
    //this is what you are up to, you just changed stylesheet to a hashmap and you're dealing with the fallout
    pub fn to_css(&self) -> String {
        self.0
            .iter()
            .map(|(selector, styles)| CSSRuleSet(selector.clone(), styles.clone()).to_css())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn from_page(page: &Page) -> Self {
        page.content
            .iter()
            .fold(Self::new(), |mut stylesheet, element| {
                stylesheet
                    .0
                    .extend(Stylesheet::from_element(element, Stylesheet::new()).0);
                stylesheet
            })
    }
    //This is what you are up to
    fn from_element(element: &Element, mut reducer: Self) -> Self {
        reducer
            .0
            .insert(
                "#".to_string() + &element.id.to_string(),
                element.meta.styles.clone(),
            )
            .expect("Failed to insert ({}, {}) into the stylesheet");
        match element.content.clone() {
            ElementContent::Column(column) => column
                .elements
                .iter()
                .map(|el| Self::from_element(el, reducer.clone()))
                .collect::<Vec<Stylesheet>>()
                .iter()
                .fold(reducer.clone(), |mut output, stylesheet| {
                    stylesheet.0.iter().for_each(|(separator, styles)| {
                        reducer.0.insert(separator.clone(), styles.clone());
                    });
                    reducer.clone()
                }),

            ElementContent::Row(row) => row
                .elements
                .iter()
                .map(|el| Self::from_element(el, reducer.clone()))
                .collect::<Vec<Stylesheet>>()
                .iter()
                .fold(reducer.clone(), |mut output, stylesheet| {
                    stylesheet.0.iter().for_each(|(separator, styles)| {
                        reducer.0.insert(separator.clone(), styles.clone());
                    });
                    reducer.clone()
                }),
            _ => reducer,
        }
    }

    pub fn from_html_element(element: &HtmlElement) -> Self {
        let mut sheet = Stylesheet::new();
        Stylesheet::populate_from_element(&mut sheet, element);
        sheet
    }

    fn populate_from_element(sheet: &mut Stylesheet, element: &HtmlElement) {
        // Generate selector based on id
        let selector = format!("#{}", element.id);

        // Add or merge styles
        let entry = sheet.0.entry(selector).or_insert_with(Vec::new);

        for new_style in &element.styles {
            let is_present = entry
                .iter()
                .any(|existing_style| existing_style.variant_eq(new_style));
            if !is_present {
                entry.push(new_style.clone());
            }
        }

        // Recurse into children if any
        if let HtmlInner::Children(children) = &element.inner {
            for child in children {
                Stylesheet::populate_from_element(sheet, child);
            }
        }
        let selector = format!("#{}:hover", element.id);

        // Add or merge styles
        let entry = sheet.0.entry(selector).or_insert_with(Vec::new);

        for new_style in &element.hover_styles {
            let is_present = entry
                .iter()
                .any(|existing_style| existing_style.variant_eq(new_style));
            if !is_present {
                entry.push(new_style.clone());
            }
        }

        // Recurse into children if any
        if let HtmlInner::Children(children) = &element.inner {
            for child in children {
                Stylesheet::populate_from_element(sheet, child);
            }
        }
    }
}
