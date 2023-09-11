use std::fmt::Debug;

#[derive(Debug)]
pub struct PushError;

#[derive(Debug, Clone)]
pub struct Element {
    content: ElementContent,
    meta: ElementMetaData,
}
impl Element {
    pub fn to_html(&self) -> String {
        println!("{}", &self.get_inline_style_string());
        self.content
            .to_html()
            .replace("~~styles~~", &self.get_inline_style_string())
    }

    fn get_inline_style_string(&self) -> String {
        self.meta
            .styles
            .iter()
            .fold(String::new(), |style_string, style| {
                style_string + &style.to_string()
            })
    }

    pub fn push(&mut self, element: Element) -> Self {
        match &mut self.content {
            ElementContent::Column(column) => {
                column.elements.push(element);
                self.clone()
            }
            ElementContent::Row(row) => {
                row.elements.push(element);
                self.clone()
            }
            _ => panic!("Tried to push to an unpushable element. {:?}", self),
        }
    }

    pub fn add_style(mut self, style: Style) -> Self {
        self.meta.add_style(style);
        self
    }

    pub fn add_styles(mut self, styles: Vec<Style>) -> Self {
        self.meta.add_styles(styles);
        self
    }
}
#[derive(Debug, Clone)]
pub enum ElementContent {
    Column(Column),
    Row(Row),
    Text(Text),
    Link(Link),
}

impl ElementContent {
    fn to_html(&self) -> String {
        match self {
            Self::Column(column) => column.to_html(),
            Self::Row(row) => row.to_html(),
            Self::Text(text) => text.to_html(),
            Self::Link(link) => link.to_html(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElementMetaData {
    classes: Vec<String>,
    styles: Vec<Style>,
    attributes: Vec<Attribute>,
}

impl ElementMetaData {
    fn new() -> Self {
        Self {
            classes: Vec::new(),
            styles: Vec::new(),
            attributes: Vec::new(),
        }
    }

    // Add a single style
    pub fn add_style(&mut self, style: Style) -> &mut Self {
        self.styles.push(style);
        self
    }

    // Add multiple styles at once
    pub fn add_styles(&mut self, styles: Vec<Style>) -> &mut Self {
        self.styles.extend(styles);
        self
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Style {
    Rounded(Unit),
    RoundedEach(Corners),
    Margin(Unit),
    MarginEach(Sides),
    Padding(Unit),
    PaddingEach(Sides),
    BackgroundColor(Color),
    TextColor(Color),
    Center,
    Width(Unit),
    Height(Unit),
}

impl Style {
    pub fn to_string(&self) -> String {
        match self{
            Self::Rounded(unit) => format!("border-radius:{unit};"),
            Self::RoundedEach(corners) => format!("border-top-left-radius:{};border-top-right-radius:{};border-bottom-left-radius:{};border-bottom-right-radius:{};", corners.top_left, corners.top_right, corners.bottom_left, corners.bottom_right),
            Self::Margin(unit) => format!("margin:{};", unit),
            Self::MarginEach(sides) => format!("margin-top:{};margin-bottom:{};margin-right:{};margin-left:{};", sides.top, sides.bottom, sides.right, sides.left),
            Self::Padding(unit) => format!("padding:{unit};"),
            Self::PaddingEach(sides) => format!("padding-top:{};padding-bottom:{};padding-right:{};padding-left:{};", sides.top, sides.bottom, sides.right, sides.left),
            Self::BackgroundColor(color) => format!("background-color:{};", color.to_string()),
            Self::TextColor(color) => format!("color:{};", color.to_string()),
            Self::Center => format!("margin:auto;"),
            Self::Height(unit) => format!("height:{unit};" ),
            Self::Width(unit) => format!("width:{unit};" )
        }
    }
}
//Obviously make this better, make it rgba if possible
#[derive(Clone, Debug)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Orange,
    Purple,
    Black,
    White,
    Grey,
}

impl Color {
    fn to_string(&self) -> String {
        match self {
            Self::Red => "red".to_owned(),
            Self::Blue => "blue".to_owned(),
            Self::Yellow => "yellow".to_owned(),
            Self::Green => "green".to_owned(),
            Self::Orange => "orange".to_owned(),
            Self::Purple => "purple".to_owned(),
            Self::Black => "black".to_owned(),
            Self::White => "white".to_owned(),
            Self::Grey => "grey".to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sides {
    top: Unit,
    bottom: Unit,
    left: Unit,
    right: Unit,
}

#[derive(Clone, Debug)]
pub struct Corners {
    top_left: Unit,
    top_right: Unit,
    bottom_left: Unit,
    bottom_right: Unit,
}

#[derive(Debug, Clone)]
pub struct Row {
    elements: Vec<Element>,
}

impl Row {
    pub fn new() -> Element {
        Element {
            content: ElementContent::Row(Self {
                elements: Vec::new(),
            }),
            meta: ElementMetaData::new(),
        }
    }

    pub fn push(mut self, element: impl Into<Element>) -> Self {
        self.elements.push(element.into());
        self
    }
}

impl El for Row {
    fn to_html(&self) -> String {
        let elements = self
            .elements
            .iter()
            .fold("".to_string(), |acc, element| acc + &element.to_html());
        format!("<div {{attributes}} class={{classes}} style=~~styles~~>{elements}</div>")
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    elements: Vec<Element>,
}

impl Column {
    pub fn new() -> Element {
        Element {
            content: ElementContent::Column(Self {
                elements: Vec::new(),
            }),
            meta: ElementMetaData::new(),
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
        format!("<div {{attributes}} class={{classes}} style=~~styles~~>{elements}</div>")
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: &str) -> Element {
        Element {
            content: ElementContent::Text(Self {
                content: content.to_string(),
            }),
            meta: ElementMetaData::new(),
        }
    }
}

impl El for Text {
    fn to_html(&self) -> String {
        format!(
            "<span {{attributes}} class={{classes}} style=~~styles~~>{}</span>",
            self.content
        )
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Button {
    label: Element,
    on_press: String, //change this to a Message
}

impl El for Button {
    fn to_html(&self) -> String {
        format!(
            "<button href=\"{}\" {{attributes}} class={{classes}} style=~~styles~~>{}</button>",
            self.on_press,
            self.label.to_html()
        )
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Link {
    label: Box<Element>,
    target: String, //change this to a url, can be relative or absolute
}

// You should read about traits and see if they can have fields not just functions
pub trait El: Debug {
    fn to_html(&self) -> String;
}

impl El for Link {
    fn to_html(&self) -> String {
        format!(
            "<a href=\"{}\" class={{classes}} style=~~styles~~ {{attributes}}>{}</a>",
            self.target,
            self.label.to_html()
        )
        .to_string()
    }
}

pub fn column() -> Element {
    Column::new()
}

pub fn row() -> Element {
    Row::new()
}

pub fn text(text: &str) -> Element {
    Text::new(text)
}

#[derive(Debug, Clone)]
pub enum Unit {
    Px(u32),
    Em(f32),
    Rem(f32),
    Percent(f32),
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Px(px) => write!(f, "{}px", px),
            Unit::Em(em) => write!(f, "{}em", em),
            Unit::Rem(rem) => write!(f, "{}rem", rem),
            Unit::Percent(percent) => write!(f, "{}%", percent),
        }
    }
}
impl Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Px(px) => format!("{px}px"),
            Unit::Em(em) => format!("{em}em"),
            Unit::Rem(rem) => format!("{rem}rem"),
            Unit::Percent(percent) => format!("{percent}%"),
        }
    }
}
