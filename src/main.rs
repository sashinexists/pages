use std::fs;
mod ui;
use ui::{Element, *};
fn main() {
    let view = Column::new()
        .push(Element::Text(Text::new("hello")))
        .push(Element::Text(Text::new("world,")))
        .push(Element::Text(Text::new("it's")))
        .push(Element::Text(Text::new("me,")))
        .push(Element::Text(Text::new("Chara")));

    let output = view.to_html();
    fs::write("index.html", output).expect("Failed to write to index.html");
}
