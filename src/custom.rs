// This is not a part of the actual framework, it's an example of the framework being used
use crate::ui::*;

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
        .add_style(Style::Padding(Unit::Px(30)))
        .add_style(Style::Rounded(Unit::Px(15)))
        .add_style(Style::FontWeight(FontWeight::Light))
        .add_style(Style::FontSize(Unit::Px(18)))
        .add_style(Style::TextAlign(TextAlign::Justify))
}

pub fn card(title: &str, content: &str, src: &str, alt: &str) -> Element {
    column()
        .push(
            row()
                .push(text(content).add_style(Style::Width(Unit::Percent(100.0))))
                .add_style(Style::Width(Unit::Percent(100.0))),
        )
        .add_style(Style::Width(Unit::Percent(100.0)))
        .add_style(Style::Padding(Unit::Px(30)))
        .add_style(Style::Rounded(Unit::Px(15)))
        .add_style(Style::FontWeight(FontWeight::Light))
        .add_style(Style::FontSize(Unit::Px(24)))
        .add_style(Style::TextAlign(TextAlign::Justify))
}
pub mod colors {

    use crate::Color; // if Color is defined in another module but in the same crate
    pub const RICH_BLACK: Color = Color::new(3, 3, 3, 1.0);
    pub const EERIE_BLACK: Color = Color::new(23, 23, 23, 1.0);
    pub const EERIE_BLACK_LIGHTEST: Color = Color::new(45, 45, 45, 1.0);
    pub const EERIE_BLACK_LIGHTEST_TRANSPARENT: Color = Color::new(45, 45, 45, 0.9);
    pub const EERIE_BLACK_LIGHTER: Color = Color::new(36, 36, 36, 1.0);
    pub const EERIE_BLACK_LIGHTER_TRANSPARENT: Color = Color::new(36, 36, 36, 0.9);
    pub const EERIE_BLACK_DARKER: Color = Color::new(18, 18, 18, 1.0);
    pub const EERIE_BLACK_DARKER_TRANSPARENT: Color = Color::new(18, 18, 18, 0.9);
    pub const CHARLESTON_GREEN: Color = Color::new(44, 44, 44, 1.0);
    pub const DARK_MEDIUM_GRAY: Color = Color::new(170, 170, 170, 1.0);
    pub const PLATINUM: Color = Color::new(233, 233, 233, 1.0);
    pub const MIDDLE_GREEN: Color = Color::new(82, 170, 94, 1.0);
    pub const TURQUOISE_GREEN: Color = Color::new(160, 208, 167, 1.0);
    pub const AMARANTH: Color = Color::new(239, 45, 86, 1.0);
}
