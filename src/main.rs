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

use crate::custom::api::get_testimonials_data;
use crate::custom::datatypes::{Testimonials, View};
fn main() {
    dotenv().expect("Failed to read .env file"); // This line loads the .env file into environment variables
    let access_token = env::var("CONTENTFUL_CONTENT_DELIVERY_API_ACCESS_TOKEN")
        .expect("CONTENTFUL_ACCESS_TOKEN not found");
    let space_id = env::var("CONTENTFUL_SPACE_ID").expect("CONTENTFUL_SPACE_ID not found");

    const GLOBAL_STYLES: &[Style] = &[
        Style::BackgroundColor(colors::RICH_BLACK),
        Style::Margin(Unit::Px(0)),
        Style::Width(Unit::Percent(100.0)),
        Style::Font("Ubuntu"),
        Style::TextColor(colors::DARK_MEDIUM_GRAY),
    ];

    let home = Page::new("Sashin Dev", ".public/index.html", PageData::None);

    let mut site = Site::new(home, "Sashin Dev");

    let main = home_page_template(content(&access_token, &space_id));

    let mut testimonials_page = Page::new(
        "Sashin Dev - Testimonials",
        ".public/testimonials.html",
        PageData::None,
    );
    let testimonials_data = get_testimonials_data(&access_token, &space_id)
        .expect("Failed to get testimonials data from contentful");
    let testimonials = Testimonials::from_items(&access_token, &space_id, testimonials_data);
    testimonials_page.push(page_template(testimonials.view()));
    site.add_page(testimonials_page);
    site.home.push(main);
    site.add_global_styles(GLOBAL_STYLES);
    site.publish();
}
