mod custom;
use dotenv::dotenv;
use std::env;
mod html;
use custom::components::*;
use custom::datatypes::PageData;
use custom::theme::*;
#[macro_use]
mod ui;
use ui::*;
mod id;
pub mod site;
use site::*;
fn main() {
    dotenv().expect("Failed to read .env file"); // This line loads the .env file into environment variables
    let access_token = env::var("CONTENTFUL_CONTENT_DELIVERY_API_ACCESS_TOKEN")
        .expect("CONTENTFUL_ACCESS_TOKEN not found");
    let space_id = env::var("CONTENTFUL_SPACE_ID").expect("CONTENTFUL_SPACE_ID not found");
    let mut pages = Pages::new();
    let home = (Page::new("Sashin Dev", ".public/index.html", PageData::None)).add_styles(&[
        Style::BackgroundColor(colors::RICH_BLACK),
        Style::Margin(Unit::Px(0)),
        Style::Width(Unit::Percent(100.0)),
        Style::Font("Ubuntu".to_string()),
        Style::TextColor(colors::DARK_MEDIUM_GRAY),
    ]);

    let main = column()
        .add_style(Style::Center)
        .push(page_header())
        .push(banner())
        .push(content(&access_token, &space_id))
        .push(footer());

    let home = home.push(main);
    pages.add(home);
    pages.publish();
}
