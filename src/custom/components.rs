use super::api::get_testimonials_data;
use super::datatypes::{Projects, Skills, Testimonials, View};
use super::theme::*;
use super::utility::*;
use crate::custom::api::{get_past_projects_data, get_skills_data};
use crate::ui::*;
use chrono::prelude::*;
use chrono::{Datelike, Timelike};
use chrono_tz::Australia::Sydney;
pub fn footer() -> Element {
    let utc_now = chrono::Utc::now().naive_utc();
    let sydney_now = Sydney.from_utc_datetime(&utc_now);

    let (is_pm, hour_12) = sydney_now.hour12();
    let am_pm = if is_pm { "PM" } else { "AM" };

    let day_name = sydney_now.format("%A").to_string();
    let day = sydney_now.day();
    let month = sydney_now.format("%B").to_string();
    let year = sydney_now.year();

    row()
        .push(text(&format!(
            "Last Updated on {}, {} of {}, {} at {}:{:02}:{:02}{}",
            day_name,
            ordinal(day),
            month,
            year,
            hour_12,
            sydney_now.minute(),
            sydney_now.second(),
            am_pm,
        )))
        .add_styles(&[Style::FontSize(Unit::Px(14)), Style::Padding(Unit::Px(5))])
}

pub fn header_link(label: &str, target: &str) -> Element {
    link(text(label), target)
        .add_styles(&[
            Style::Padding(Unit::Px(15)),
            Style::Height(Unit::Percent(100.0)),
            Style::TextColor(colors::DARK_MEDIUM_GRAY),
            Style::NoUnderline,
        ])
        .add_hover_style(Style::TextColor(colors::PLATINUM))
}

pub fn introduction() -> Element {
    const INTRO_TEXT:&'static str = "My name is Sashin, and I help ambitious and creative individuals and organisations design and build their dream websites.\n\nI work directly with clients to bring their vision to life, getting to know them, their mission and brand, and create websites that reflect them.";
    column()
        .push(
            row()
                .push(text(INTRO_TEXT).add_styles(&[
                    Style::Width(Unit::Percent(100.0)),
                    Style::LineHeight(Unit::Percent(150.0)),
                ]))
                .add_styles(&[Style::Width(Unit::Percent(100.0))]),
        )
        .add_styles(&[
            Style::Width(Unit::Percent(100.0)),
            Style::Padding(Unit::Px(20)),
            Style::FontWeight(FontWeight::Light),
            Style::FontSize(Unit::Px(18)),
            Style::TextAlign(TextAlign::Justify),
        ])
}

pub fn page_title(title: &str) -> Element {
    heading(HeadingLevel::H1, title)
        .add_styles(&[
            Style::FontWeight(FontWeight::Light),
            Style::TextColor(colors::MIDDLE_GREEN),
            Style::FontSize(Unit::Px(36)),
        ])
        .add_hover_style(Style::TextColor(colors::TURQUOISE_GREEN))
}

pub fn page_header() -> Element {
    row()
        .add_styles(&[
            Style::JustifyContent(JustifyContent::SpaceBetween),
            Style::Width(Unit::Percent(100.0)),
            Style::PaddingEach(Sides::new(
                Unit::Px(5),
                Unit::Px(5),
                Unit::Px(20),
                Unit::Px(20),
            )),
            Style::Height(Unit::Percent(100.0)),
            Style::FontSize(Unit::Px(13)),
        ])
        .push(link(page_title("Sashin Dev"), "https://sashin.dev").add_style(Style::NoUnderline))
        .push(
            row()
                .push(header_link("Past Work", "/past-work"))
                .push(header_link("Skills", "/skills"))
                .push(header_link("Testimonials", "/testimonials"))
                .push(header_link("Writing", "/writing"))
                .push(header_link("Now", "/now")),
        )
}

pub fn banner() -> Element {
    column()
        .add_styles(&[
            Style::BackgroundImage(Image {
                src: "'assets/images/banner.jpg'".to_string(),
                alt: "banner-image".to_string(),
            }),
            Style::Width(Unit::Percent(100.0)),
            Style::Height(Unit::Px(300)),
            Style::JustifyContent(JustifyContent::End),
        ])
        .push(
            row()
                .add_styles(&[
                    Style::Width(Unit::Percent(100.0)),
                    Style::BackgroundColor(colors::EERIE_BLACK_DARKER_TRANSPARENT),
                    Style::PaddingEach(Sides::new(
                        Unit::Px(20),
                        Unit::Px(20),
                        Unit::Px(0),
                        Unit::Px(0),
                    )),
                ])
                .push(
                    text("Crafting better software for creators and innovators").add_styles(&[
                        Style::FontSize(Unit::Px(30)),
                        Style::FontWeight(FontWeight::ExtraLight),
                        Style::TextAlign(TextAlign::Center),
                        Style::Width(Unit::Percent(100.0)),
                    ]),
                ),
        )
}

pub fn skills_bar(skills: Skills) -> Element {
    skills
        .0
        .iter()
        .fold(row(), |mut skills_bar, skill| {
            skills_bar.push(
                image(&skill.thumbnail.src.to_string(), &skill.thumbnail.alt).add_styles(&[
                    Style::Height(Unit::Px(50)),
                    Style::Width(Unit::Px(50)),
                    Style::Padding(Unit::Px(5)),
                ]),
            )
        })
        .add_styles(&[
            Style::Padding(Unit::Px(5)),
            Style::Rounded(Unit::Px(10)),
            Style::BackgroundColor(colors::EERIE_BLACK_LIGHTEST),
            Style::Width(Unit::Percent(100.0)),
            Style::JustifyContent(JustifyContent::SpaceEvenly),
        ])
}

pub fn projects_view(projects: Projects) -> Element {
    row()
        .add_styles(&[Style::Width(Unit::Percent(100.0))])
        .push(
            column()
                .add_styles(&[
                    Style::Width(Unit::Percent(100.0)),
                    Style::AlignItems(AlignItems::Center),
                    Style::JustifyContent(JustifyContent::Start),
                ])
                .push(
                    row()
                        .add_styles(&[Style::Width(Unit::Percent(100.0))])
                        .push(heading(HeadingLevel::H2, "Past Work").add_styles(&[
                            Style::FontWeight(FontWeight::ExtraLight),
                            Style::FontSize(Unit::Px(35)),
                            Style::Height(Unit::Px(20)),
                        ])),
                )
                .push(
                    row()
                        .add_styles(&[Style::Width(Unit::Percent(100.0))])
                        .push(projects.view()),
                ),
        )
}

pub fn testimonials_view(testimonials: Testimonials) -> Element {
    row()
        .add_styles(&[Style::Width(Unit::Percent(100.0))])
        .push(
            column()
                .add_styles(&[
                    Style::Width(Unit::Percent(100.0)),
                    Style::AlignItems(AlignItems::Center),
                    Style::JustifyContent(JustifyContent::Start),
                ])
                .push(
                    row()
                        .add_styles(&[Style::Width(Unit::Percent(100.0))])
                        .push(heading(HeadingLevel::H2, "Testimonials").add_styles(&[
                            Style::FontWeight(FontWeight::ExtraLight),
                            Style::FontSize(Unit::Px(35)),
                            Style::Height(Unit::Px(20)),
                        ])),
                )
                .push(
                    row()
                        .add_styles(&[Style::Width(Unit::Percent(100.0))])
                        .push(testimonials.view()),
                ),
        )
}

pub fn content(access_token: &str, space_id: &str) -> Element {
    let skills_data = get_skills_data(&access_token, &space_id).expect("Failed to get skills data");
    let skills = Skills::from_items(&access_token, &space_id, skills_data);
    let testimonials_data =
        get_testimonials_data(&access_token, &space_id).expect("Failed to get skills data");
    let testimonials = Testimonials::from_items(&access_token, &space_id, testimonials_data);
    let past_projects_data =
        get_past_projects_data(&access_token, &space_id).expect("Failed to get projects data");
    let projects: Projects = Projects::from_items(&access_token, &space_id, past_projects_data);

    column()
        .add_styles(&[
            Style::Width(Unit::Px(768)),
            Style::Center,
            Style::BackgroundColor(colors::EERIE_BLACK),
            Style::RoundedEach(Corners::new(
                Unit::Px(0),
                Unit::Px(0),
                Unit::Px(10),
                Unit::Px(10),
            )),
            Style::Padding(Unit::Px(15)),
            Style::JustifyContent(JustifyContent::Start),
        ])
        .push(introduction())
        .push(skills_bar(skills))
        .push(testimonials_view(testimonials))
        .push(projects_view(projects))
}
