use super::theme::*;
use super::utility::*;
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
            "Last Updated on {}, {} of {}, {} at {}:{}:{}{}",
            day_name,
            ordinal(day),
            month,
            year,
            hour_12,
            sydney_now.minute(),
            sydney_now.second(),
            am_pm,
        )))
        .add_style(Style::FontSize(Unit::Px(14)))
        .add_style(Style::Padding(Unit::Px(5)))
}
pub fn header_link(label: &str, target: &str) -> Element {
    link(text(label), target)
        .add_style(Style::Padding(Unit::Px(15)))
        .add_style(Style::Height(Unit::Percent(100.0)))
        .add_style(Style::TextColor(colors::DARK_MEDIUM_GRAY))
        .add_style(Style::NoUnderline)
        .add_hover_style(Style::TextColor(colors::PLATINUM))
}

pub fn introduction(title: &str, content: &str, src: &str, alt: &str) -> Element {
    column()
        .push(
            row()
                .push(text(content).add_style(Style::Width(Unit::Percent(100.0))))
                .add_style(Style::Width(Unit::Percent(100.0))),
        )
        .add_style(Style::Width(Unit::Percent(100.0)))
        .add_style(Style::Padding(Unit::Px(20)))
        .add_style(Style::FontWeight(FontWeight::Light))
        .add_style(Style::FontSize(Unit::Px(18)))
        .add_style(Style::TextAlign(TextAlign::Justify))
}
