use std::fs;
mod ui;
use ui::{Element, *};
fn main() {
    let view = Column::new()
        .push(Text::new("Hello "))
        .push(Text::new("world, "))
        .push(Text::new("it's "))
        .push(Text::new("me. "))
        .push(Text::new("Chara."));

    let output = view.to_html();
    fs::write("index.html", output).expect("Failed to write to index.html");
}
