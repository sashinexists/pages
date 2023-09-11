use std::fs;
mod ui;
use ui::*;
fn main() {
    let view = column()
        .push(row().push(text("hello").add_style(Style::TextColor(Color::Blue))))
        .push(row().push(text("my").add_style(Style::BackgroundColor(Color::Red))))
        .push(row().push(text("name")))
        .push(row().push(text("is").add_style(Style::BackgroundColor(Color::Blue))))
        .push(row().push(text("chara").add_style(Style::TextColor(Color::Red))));

    let output = view.to_html();
    dbg!(view);
    fs::write("index.html", output).expect("Failed to write to index.html");
}
