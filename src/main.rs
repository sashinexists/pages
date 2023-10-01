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

use crate::custom::api::{get_past_projects_data, get_skills_data, get_testimonials_data};
use crate::custom::datatypes::Home;
use crate::custom::pages;
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

    let model = Home::new(&access_token, &space_id);
    let main = home_page_template(content(&model));

    let testimonials_page = pages::testimonials(&model.testimonials);
    let projects_page = pages::projects(&model.past_projects);
    let home = Page::new("Sashin Dev", ".public/index.html", PageData::Home(model));

    let mut site = Site::new(home, "Sashin Dev");
    site.add_page(testimonials_page);
    site.add_page(projects_page);
    site.home.push(main);
    site.add_global_styles(GLOBAL_STYLES);
    site.publish();
}
