pub struct Element {
    element: Box<dyn El>,
    classes: Vec<String>,
    styles: Vec<Style>,
    attributes: Vec<Attribute>,
}

impl Element {
    pub fn new(element: impl El + 'static) -> Self {
        Self {
            element: Box::new(element),
            classes: Vec::new(),
            styles: Vec::new(),
            attributes: Vec::new(),
            //change class to automatically use a uuid or something
        }
    }

    fn to_html(&self) -> String {
        self.element.to_html()
    }
}

impl<T> From<T> for Element
where
    T: El + 'static,
{
    fn from(other: T) -> Self {
        Self {
            element: Box::new(other),
            attributes: Vec::new(),
            styles: Vec::new(),
            classes: Vec::new(), //change class to automatically use a uuid or something
        }
    }
}

#[derive(Clone)]
pub enum Attribute {
    Height(u32),
    Width(u32),
}

impl Attribute {
    fn to_string(&self) -> String {
        match self {
            Attribute::Height(px) => format!("height={}px", px),
            Attribute::Width(px) => format!("width={}px", px),
        }
    }
}

#[derive(Clone)]
pub enum Style {
    Rounded(u32),
    RoundedEach(Corners),
    Margin(u32),
    MarginEach(Sides),
    Padding(u32),
    PaddingEach(Sides),
    BackgroundColor(Color),
    TextColor(Color),
}
//Obviously make this better, make it rgba if possible
#[derive(Clone)]
pub enum Color {
    Red,
    Blue,
}

#[derive(Clone)]
pub struct Sides {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

#[derive(Clone)]
pub struct Corners {
    top_left: u32,
    top_right: u32,
    bottom_left: u32,
    bottom_right: u32,
}

pub struct Row {
    elements: Vec<Element>,
}

impl El for Row {
    fn to_html(&self) -> String {
        let elements = self
            .elements
            .iter()
            .fold("".to_string(), |acc, element| acc + &element.to_html());
        format!("<div class={{classes}}>{elements}</div>")
    }
}

pub struct Column {
    elements: Vec<Element>,
}

impl Column {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push(mut self, element: impl Into<Element>) -> Self {
        self.elements.push(element.into());
        self
    }
}

impl El for Column {
    fn to_html(&self) -> String {
        let elements = self
            .elements
            .iter()
            .fold("".to_string(), |acc, element| acc + &element.to_html());
        format!("<div class={{classes}}>{}</div>", elements)
    }
}

/// Creates a [`Column`] with the given children.
///
/// [`Column`]: widget::Column
//code from rust discord user, don't know how it works
#[macro_export]
macro_rules! column {
    () => (
        $crate::widget::Column::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::widget::Column::with_children(vec![$($crate::Element::from($x)),+])
    );
}

pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

impl El for Text {
    fn to_html(&self) -> String {
        format!("<span class={{classes}}>{}</span>", self.content).to_string()
    }
}
pub struct Button {
    label: Element,
    on_press: String, //change this to a Message
}

impl El for Button {
    fn to_html(&self) -> String {
        format!(
            "<button href=\"{}\" class={{classes}}>{}</button>",
            self.on_press,
            self.label.to_html()
        )
        .to_string()
    }
}
pub struct Link {
    label: Box<Element>,
    target: String, //change this to a url, can be relative or absolute
}

// You should read about traits and see if they can have fields not just functions
pub trait El {
    fn to_html(&self) -> String;
}

impl El for Link {
    fn to_html(&self) -> String {
        format!(
            "<a href=\"{}\" class={{classes}}>{}</a>",
            self.target,
            self.label.to_html()
        )
        .to_string()
    }
}
