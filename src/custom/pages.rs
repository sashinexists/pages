use crate::site::Page;

use super::{
    components::page_template,
    datatypes::{PageData, Projects, Testimonials, View},
};

pub fn testimonials(testimonials: &Testimonials) -> Page {
    let mut testimonials_page = Page::new(
        "Sashin Dev - Testimonials",
        ".public/testimonials.html",
        PageData::Testimonials(testimonials.clone()),
    );
    testimonials_page.push(page_template(testimonials.view()));
    testimonials_page
}
pub fn projects(projects: &Projects) -> Page {
    let mut projects_page = Page::new(
        "Sashin Dev - Past Projects",
        ".public/past-projects.html",
        PageData::Projects(projects.clone()),
    );
    projects_page.push(page_template(projects.view()));
    projects_page
}
