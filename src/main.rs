mod custom;
mod html;
use custom::*;
use html::*;
mod ui;
use ui::*;
mod id;
fn main() {
    let mut pages = Pages::new();

    let home = (Document::new("Sashin Dev", "index.html"))
        .add_style(Style::BackgroundColor(colors::RICH_BLACK))
        .add_style(Style::Margin(Unit::Px(0)))
        .add_style(Style::PaddingEach(Sides::new(
            Unit::Px(10),
            Unit::Px(20),
            Unit::Px(20),
            Unit::Px(20),
        )))
        .add_style(Style::Width(Unit::Percent(100.0)))
        .add_style(Style::Font("Ubuntu".to_string()))
        .add_style(Style::TextColor(colors::DARK_MEDIUM_GRAY));

    let page_title: Element = heading(HeadingLevel::H1, "Sashin Dev")
        .add_style(Style::FontWeight(FontWeight::Light))
        .add_style(Style::TextColor(colors::MIDDLE_GREEN))
        .add_style(Style::FontSize(Unit::Px(36)));
    let mut main = column().add_style(Style::Center);
    let page_header: Element = row()
        .add_style(Style::SpaceBetween)
        .add_style(Style::Height(Unit::Px(50)))
        .add_style(Style::Width(Unit::Percent(100.0)))
        .add_style(Style::Padding(Unit::Px(20)))
        .push(page_title)
        .push(
            row()
                .push(header_link("Past Work", "/past-work"))
                .push(header_link("Skills", "/skills"))
                .push(header_link("Testimonials", "/testimonials"))
                .push(header_link("Writing", "/writing"))
                .push(header_link("Now", "/now")),
        )
        .add_style(Style::Height(Unit::Percent(100.0)))
        .add_style(Style::FontSize(Unit::Px(13)));

    let content = column()
        .add_style(Style::Width(Unit::Px(900)))
        .add_style(Style::Center)
        .add_style(Style::BackgroundColor(colors::EERIE_BLACK))
        .add_style(Style::TextColor(colors::DARK_MEDIUM_GRAY))
        .add_style(Style::Rounded(Unit::Px(10)))
        .add_style(Style::Padding(Unit::Px(30)))
        .push(row().push(text("hello").add_style(Style::TextColor(Color::new(255, 255, 0, 1.0)))))
        .push(row().push(text("my").add_style(Style::BackgroundColor(Color::new(200, 0, 0, 1.0)))))
        .push(row().push(text("name")))
        .push(row().push(text("is").add_style(Style::BackgroundColor(Color::new(0, 0, 200, 1.0)))))
        .push(row().push(text("chara").add_style(Style::TextColor(Color::new(250, 0, 0, 1.0)))));

    let main = main.push(page_header).push(content).push(column());

    let home = home.push(main);
    pages.add(home);
    pages.write_html();
    pages.write_css();
    dbg!(pages);
}
