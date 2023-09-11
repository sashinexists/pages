use std::fs;
mod ui;
use ui::*;
fn main() {
    let view = Column::new()
        .push(Row::new().push(Text::new("Hello, ")))
        .push(Row::new().push(Text::new("my ")))
        .push(Row::new().push(Text::new("name ")))
        .push(Row::new().push(Text::new("is ")))
        .push(Row::new().push(Text::new("Chara ")));
    let output = view.to_html();
    dbg!(view);
    fs::write("index.html", output).expect("Failed to write to index.html");
}
