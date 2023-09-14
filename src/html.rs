use uuid::Uuid;

use crate::ui::{Element, ElementContent, Style};
use std::collections::HashMap;

pub struct HtmlElement {
    tag: Tag,
    attributes: HashMap<String, String>,
    classes: Vec<String>,
    id: Uuid,
    is_self_closing: bool,
    inner: HtmlInner,
    styles: Vec<Style>,
}

impl HtmlElement {
    pub fn from_element(element: &Element, tag: Tag) -> Self {
        // Common properties
        let id = element.id;
        let classes = element.meta.classes.clone();
        let styles = element.meta.styles.clone();
        let attributes = element.meta.attributes.clone();
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
                    .map(|el| Self::from_element(el, Tag::Div))
                    .collect(),
            ),
            ElementContent::Row(row) => HtmlInner::Children(
                row.elements
                    .iter()
                    .map(|el| Self::from_element(el, Tag::Div))
                    .collect(),
            ),
            ElementContent::Text(text) => HtmlInner::Content(text.content.clone()),
            ElementContent::Link(link) => {
                HtmlInner::Children(vec![Self::from_element(&link.label, Tag::A)])
            }
            ElementContent::Heading(heading) => HtmlInner::Content(heading.content.clone()),
        };

        Self {
            id,
            attributes,
            is_self_closing,
            tag,
            classes,
            styles,
            inner,
        }
    }

    pub fn write_html(&self) -> String {
        if self.is_self_closing {
            format!(
                "<{} id=\"{}\" {} {}/>",
                self.tag,
                self.id,
                self.get_attribute_string(),
                self.get_class_string()
            )
        } else {
            format!(
                "<{} id=\"{}\" {} {}>{}</{}>",
                self.tag,
                self.id,
                self.get_attribute_string(),
                self.get_class_string(),
                self.inner.write_html(),
                self.tag
            )
        }
    }

    fn get_attribute_string(&self) -> String {
        self.attributes
            .iter()
            .map(|(k, v)| format!("k=\"{v}\""))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn get_class_string(&self) -> String {
        "class=\"".to_string() + &self.classes.join(" ") + "\""
    }
}

enum HtmlInner {
    Children(Vec<HtmlElement>),
    Content(String),
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
            Self::Content(content) => content.clone(),
            Self::None => "".to_string(),
        }
    }
}

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
