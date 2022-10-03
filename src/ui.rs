#[derive(Clone)]
pub enum Element {
    Row(Row),
    Column(Column),
    Text(Text),
    Link(Link),
    Button(Button),
}

impl El for Element {
    fn to_html(&self) -> String {
        match self {
            Element::Row(row) => row.to_html(),
            Element::Column(column) => column.to_html(),
            Element::Text(text) => text.to_html(),
            Element::Link(link) => link.to_html(),
            Element::Button(button) => button.to_html(),
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

#[derive(Clone)]
pub struct Row {
    elements: Vec<Element>,
    attributes: Vec<Attribute>,
    class: Option<String>,
    style: Vec<Style>,
}

impl El for Row {
    fn to_html(&self) -> String {
        let class = match self.class.clone() {
            Some(class) => class,
            None => "".to_string(),
        };

        format!("<div class=\"row {}\"></div>", class)
    }
}

#[derive(Clone)]
pub struct Column {
    elements: Vec<Element>,
    attributes: Vec<Attribute>,
    class: Option<String>,
    styles: Vec<Style>,
}

impl Column {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            attributes: Vec::new(),
            class: None,
            styles: Vec::new(),
        }
    }

    pub fn push(self, element: Element) -> Self {
        let mut elements = self.elements.clone();
        elements.push(element);
        Self { elements, ..self }
    }
    pub fn to_html(&self) -> String {
        let class = match self.class.clone() {
            Some(class) => class,
            None => "".to_string(),
        };

        let elements = self
            .elements
            .iter()
            .fold("".to_string(), |acc, element| acc + &element.to_html());

        format!("<div class=\"column {}\">{}</div>", class, elements)
    }
}

impl El for Column {
    fn to_html(&self) -> String {
        let class = match self.class.clone() {
            Some(class) => class,
            None => "".to_string(),
        };

        format!("<div class=\"column {}\"></div>", class)
    }
}

#[derive(Clone)]
pub struct Text {
    content: String,
    attributes: Vec<Attribute>,
    class: Option<String>,
    styles: Vec<Style>,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            attributes: Vec::new(),
            class: None,
            styles: Vec::new(),
        }
    }
}

impl El for Text {
    fn to_html(&self) -> String {
        let class = match self.class.clone() {
            Some(class) => class,
            None => "".to_string(),
        };
        format!("<span class=\"{}\">{}</span>", class, self.content).to_string()
    }
}
#[derive(Clone)]
pub struct Button {
    label: Box<Element>,
    on_press: String, //change this to a Message
    class: Option<String>,
}

impl El for Button {
    fn to_html(&self) -> String {
        let class = match self.class.clone() {
            Some(class) => class,
            None => "".to_string(),
        };
        format!(
            "<button href=\"{}\" class=\"{}\">{}</button>",
            self.on_press,
            class,
            self.label.to_html()
        )
        .to_string()
    }
}
#[derive(Clone)]
pub struct Link {
    label: Box<Element>,
    target: String, //change this to a url, can be relative or absolute
    class: Option<String>,
}

// You should read about traits and see if they can have fields not just functions
pub trait El {
    fn to_html(&self) -> String;
}

impl El for Link {
    fn to_html(&self) -> String {
        let class = match self.class.clone() {
            Some(class) => class,
            None => "".to_string(),
        };
        format!(
            "<a href=\"{}\" class=\"{}\">{}</a>",
            self.target,
            class,
            self.label.to_html()
        )
        .to_string()
    }
}
