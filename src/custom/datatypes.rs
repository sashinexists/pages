use super::super::ui;
use super::super::*;
use super::{
    api::{
        contentful::{AssetData, Item, Items},
        get_asset_by_id, get_person_data_by_id, get_skill_by_id, get_testimonial_by_id,
    },
    theme::*,
};
use url::Url;
#[derive(Debug, Clone)]
pub enum PageData {
    None,
    Home(Home),
    Projects(Projects),
    Testimonials(Testimonials),
}

pub trait View {
    fn view(&self) -> Element;
}

// there is an empty PageData enum (with only None as an option) and the View trait here by default, everything else is custome

#[derive(Debug, Clone)]
pub struct Home {
    pub testimonials: Testimonials,
    pub past_projects: Projects,
    pub skills: Skills,
}

impl Home {
    pub fn new(access_token: &str, space_id: &str) -> Self {
        Self {
            testimonials: Testimonials::new(access_token, space_id),
            past_projects: Projects::new(access_token, space_id),
            skills: Skills::new(access_token, space_id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Testimonials(Vec<Testimonial>);

impl Testimonials {
    pub fn new(access_token: &str, space_id: &str) -> Self {
        let items = get_testimonials_data(access_token, space_id)
            .expect("Failed to get testimonials data from contentful");
        Self::from_items(access_token, space_id, items)
    }

    pub fn from_items(access_token: &str, space_id: &str, items: Items) -> Self {
        let testimonials: Vec<Testimonial> = items
            .items
            .into_iter()
            .map(|item| {
                Testimonial::from_item(access_token, space_id, item)
                    .expect("Failed to parse Testimonial from item")
            })
            .collect::<Vec<Testimonial>>();
        Self(testimonials)
    }
}

impl View for Testimonials {
    fn view(&self) -> Element {
        self.0
            .iter()
            .fold(column(), |mut testimonials_view, testimonial| {
                testimonials_view.push(testimonial.view())
            })
    }
}
#[derive(Debug, Clone)]
pub struct Testimonial {
    author: Person,
    text: String,
    slug: String,
}

impl Testimonial {
    pub fn from_item(access_token: &str, space_id: &str, item: Item) -> Result<Self, ParseError> {
        Ok(Self {
            text: item.fields.text.expect("Failed to get testimonial's text"),
            slug: item.fields.slug.expect("Failed to get testimonial's slug"),
            author: Person::from_item(
                access_token,
                space_id,
                get_person_data_by_id(
                    access_token,
                    space_id,
                    &item
                        .fields
                        .author
                        .expect("Failed to get testimonial's author")
                        .sys
                        .id,
                )
                .expect("Failed to get Testimonial's author")
                .items
                .first()
                .expect("No author with that id")
                .clone(),
            )
            .expect("failed to parse testimonial's author"),
        })
    }
}

impl View for Testimonial {
    fn view(&self) -> Element {
        row()
            .add_styles(&[
                Style::MarginEach(Sides::new(
                    Unit::Px(20),
                    Unit::Px(20),
                    Unit::Px(0),
                    Unit::Px(0),
                )),
                Style::Rounded(Unit::Px(10)),
                Style::BackgroundColor(colors::EERIE_BLACK_LIGHTER),
                Style::Padding(Unit::Px(20)),
                Style::JustifyContent(JustifyContent::SpaceBetween),
                Style::Height(Unit::Px(240)),
            ])
            .push(
                column()
                    .add_styles(&[
                        Style::Width(Unit::Percent(20.0)),
                        Style::Height(Unit::Percent(100.0)),
                        Style::JustifyContent(JustifyContent::Center),
                        Style::AlignItems(AlignItems::Center),
                    ])
                    .push(
                        image(&self.author.photo.src.to_string(), &self.author.photo.alt)
                            .add_styles(&[
                                Style::Width(Unit::Px(120)),
                                Style::Height(Unit::Px(120)),
                                Style::Rounded(Unit::Px(120)),
                                Style::Center,
                            ]),
                    ),
            )
            .push(
                column()
                    .add_styles(&[
                        Style::Width(Unit::Percent(75.0)),
                        Style::Height(Unit::Percent(100.0)),
                        Style::JustifyContent(JustifyContent::Center),
                        Style::MarginEach(Sides::new(
                            Unit::Px(0),
                            Unit::Px(0),
                            Unit::Percent(0.0),
                            Unit::Px(0),
                        )),
                    ])
                    .push(
                        row()
                            .add_styles(&[
                                Style::MarginEach(Sides::new(
                                    Unit::Px(5),
                                    Unit::Px(5),
                                    Unit::Px(0),
                                    Unit::Px(0),
                                )),
                                Style::FontSize(Unit::Px(20)),
                                Style::FontWeight(FontWeight::Light),
                                Style::LineHeight(Unit::Percent(120.0)),
                                Style::Width(Unit::Percent(100.0)),
                                Style::TextAlign(TextAlign::Left),
                                Style::JustifyContent(JustifyContent::Start),
                            ])
                            .push(text(&self.text)),
                    )
                    .push(
                        column()
                            .add_styles(&[
                                Style::MarginEach(Sides::new(
                                    Unit::Px(5),
                                    Unit::Px(5),
                                    Unit::Px(0),
                                    Unit::Px(0),
                                )),
                                Style::JustifyContent(JustifyContent::Start),
                                Style::Width(Unit::Percent(100.0)),
                            ])
                            .push(
                                row()
                                    .add_styles(&[
                                        Style::MarginEach(Sides::new(
                                            Unit::Px(5),
                                            Unit::Px(2),
                                            Unit::Px(0),
                                            Unit::Px(0),
                                        )),
                                        Style::FontSize(Unit::Px(15)),
                                        Style::FontWeight(FontWeight::Normal),
                                        Style::LineHeight(Unit::Percent(120.0)),
                                        Style::Width(Unit::Percent(100.0)),
                                        Style::TextAlign(TextAlign::Left),
                                        Style::JustifyContent(JustifyContent::Start),
                                    ])
                                    .push(text(&self.author.name)),
                            )
                            .push(
                                row()
                                    .add_styles(&[
                                        Style::FontSize(Unit::Px(12)),
                                        Style::FontWeight(FontWeight::Normal),
                                        Style::LineHeight(Unit::Percent(120.0)),
                                        Style::Width(Unit::Percent(100.0)),
                                        Style::TextAlign(TextAlign::Left),
                                        Style::JustifyContent(JustifyContent::Start),
                                    ])
                                    .push(text(
                                        &(self.author.title.clone()
                                            + ", "
                                            + &self.author.organisation),
                                    )),
                            ),
                    ),
            )
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub title: String,
    pub src: Url,
    pub alt: String,
}

impl Image {
    pub fn from_asset_data(asset: AssetData) -> Result<Self, ParseError> {
        Ok(Self {
            title: asset.fields.title.expect("Failed to parse image title"),
            alt: asset
                .fields
                .description
                .expect("Failed to parse image descrpition"),
            src: Url::parse(
                &("https://".to_string()
                    + &asset
                        .fields
                        .file
                        .expect("Failed to parse image file path")
                        .url),
            )
            .expect("Failed to parse asset url as valid url"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    title: String,
    photo: Image,
    website: Option<Url>,
    organisation: String,
}

impl Person {
    pub fn from_item(access_token: &str, space_id: &str, item: Item) -> Result<Self, ParseError> {
        Ok(Self {
            name: item.fields.name.expect("Failed to get person's name"),
            title: item.fields.title.expect("Failed to get person's title"),
            photo: Image::from_asset_data(
                get_asset_by_id(
                    access_token,
                    space_id,
                    &item
                        .fields
                        .photo
                        .expect("Failed to get person's photo")
                        .sys
                        .id,
                )
                .expect("Failed to parse person's photo"),
            )
            .expect("Failed to parse person's asset data"),
            website: {
                match item.fields.website {
                    Some(website) => Some(
                        Url::parse(&website)
                            .expect("Failed to parse author's website as valid URL"),
                    ),
                    None => None,
                }
            },
            organisation: item
                .fields
                .organisation
                .expect("Failed to get organisation for person"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Projects(Vec<Project>);

impl Projects {
    pub fn new(access_token: &str, space_id: &str) -> Self {
        let items = get_past_projects_data(access_token, space_id)
            .expect("Failed to get past projects data from contentful");
        Self::from_items(access_token, space_id, items)
    }
    pub fn from_items(access_token: &str, space_id: &str, items: Items) -> Self {
        let items = items
            .items
            .into_iter()
            .map(|item| {
                Project::from_item(access_token, space_id, item)
                    .expect("Invalid project in projects")
            })
            .collect::<Vec<Project>>();
        Self(items)
    }
}

impl View for Projects {
    fn view(&self) -> Element {
        self.0
            .iter()
            .fold(column(), |mut output, project| output.push(project.view()))
            .add_style(Style::Width(Unit::Percent(100.0)))
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    title: String,
    screenshot: Image,
    github_url: Option<Url>,
    description: String,
    about: String,
    testimonial: Option<Testimonial>,
    website_url: Option<Url>,
    skills: Vec<Skill>,
    slug: String,
}

impl Project {
    pub fn from_item(access_token: &str, space_id: &str, item: Item) -> Result<Self, ParseError> {
        Ok(Self {
            title: item.fields.title.expect("Failed to get project title"),
            screenshot: Image::from_asset_data(
                get_asset_by_id(
                    access_token,
                    space_id,
                    &item
                        .fields
                        .screenshot
                        .expect("Failed to get project screenshot")
                        .sys
                        .id,
                )
                .expect("Failed to parse project's screenshot"),
            )
            .expect("Failed to parse image from project's screenshot"),
            description: item
                .fields
                .description
                .expect("Failed to get project's description"),
            about: item
                .fields
                .about
                .expect("Failed to get projects about text"),
            slug: item.fields.slug.expect("Failed to get project's slug"),
            website_url: match &item.fields.website {
                Some(url) => Some(Url::parse(url).expect("failed to parse website url")),
                None => None,
            },
            github_url: match &item.fields.github_url {
                Some(url) => Some(Url::parse(url).expect("failed to parse github url")),
                None => None,
            },
            testimonial: match &item.fields.testimonial {
                Some(testimonial_nested_sys) => Testimonial::from_item(
                    access_token,
                    space_id,
                    get_testimonial_by_id(access_token, space_id, &testimonial_nested_sys.sys.id)
                        .expect(&format!(
                            "Failed to get testimonial from id {}",
                            &testimonial_nested_sys.sys.id
                        )),
                )
                .ok(),
                None => None,
            },
            skills: match item.fields.skills {
                Some(skills) => skills
                    .iter()
                    .map(|skill| {
                        Skill::from_item(
                            access_token,
                            space_id,
                            get_skill_by_id(access_token, space_id, &skill.sys.id)
                                .expect("Failed to get skill from id"),
                        )
                        .expect("Failed to parse project's skill")
                    })
                    .collect::<Vec<Skill>>(),

                None => Vec::new(),
            },
        })
    }
}

impl View for Project {
    fn view(&self) -> Element {
        const TITLE_BAR_HEIGHT: Unit = Unit::Px(60);
        column()
            .add_styles(&[
                Style::BackgroundColor(colors::EERIE_BLACK_LIGHTER),
                Style::Rounded(Unit::Px(10)),
                Style::MarginEach(Sides::new(
                    Unit::Px(0),
                    Unit::Px(70),
                    Unit::Px(0),
                    Unit::Px(0),
                )),
                Style::Width(Unit::Percent(100.0)),
            ])
            .push(
                row()
                    .add_styles(&[
                        Style::Height(TITLE_BAR_HEIGHT),
                        Style::Width(Unit::Percent(100.0)),
                        Style::AlignItems(ui::AlignItems::Center),
                        Style::JustifyContent(ui::JustifyContent::Center),
                    ])
                    .push(heading(HeadingLevel::H3, &self.title).add_styles(&[
                        Style::FontSize(Unit::Px(25)),
                        Style::FontWeight(FontWeight::Light),
                        Style::Padding(Unit::Px(0)),
                        Style::Width(Unit::Percent(100.0)),
                        Style::Height(Unit::Percent(100.0)),
                        Style::LineHeight(TITLE_BAR_HEIGHT),
                        Style::TextAlign(ui::TextAlign::Center),
                    ])),
            )
            .push(
                row()
                    .add_styles(&[Style::Width(Unit::Percent(100.0))])
                    .push(
                        column()
                            .add_styles(&[
                                Style::Width(Unit::Percent(100.0)),
                                Style::Height(Unit::Px(500)),
                                Style::BackgroundImage(crate::ui::Image {
                                    src: self.screenshot.src.to_string(),
                                    alt: self.screenshot.alt.clone(),
                                }),
                                Style::BackgroundSize(BackgroundSize::Cover),
                                Style::AlignItems(ui::AlignItems::Center),
                                Style::JustifyContent(ui::JustifyContent::End),
                                Style::Rounded(Unit::Px(10)),
                            ])
                            .push(
                                row()
                                    .add_styles(&[
                                        Style::Width(Unit::Percent(100.0)),
                                        Style::TextAlign(ui::TextAlign::Center),
                                        Style::BackgroundColor(
                                            colors::EERIE_BLACK_DARKER_TRANSPARENT,
                                        ),
                                        Style::RoundedEach(Corners::new(
                                            Unit::Px(0),
                                            Unit::Px(0),
                                            Unit::Px(10),
                                            Unit::Px(10),
                                        )),
                                        Style::Padding(Unit::Px(10)),
                                    ])
                                    .push(text(&self.description).add_styles(&[
                                        Style::Width(Unit::Percent(100.0)),
                                        Style::FontSize(Unit::Px(16)),
                                        Style::FontWeight(FontWeight::Light),
                                    ])),
                            ),
                    ),
            )
    }
}

#[derive(Debug, Clone)]
pub struct Skills(pub Vec<Skill>);

impl Skills {
    pub fn new(access_token: &str, space_id: &str) -> Self {
        let items = get_skills_data(access_token, space_id)
            .expect("Failed to get skills data from contentful");
        Self::from_items(access_token, space_id, items)
    }
    pub fn from_items(access_token: &str, space_id: &str, items: Items) -> Self {
        let items = items
            .items
            .into_iter()
            .map(|item| {
                Skill::from_item(access_token, space_id, item).expect("Invalid item in items")
            })
            .collect::<Vec<Skill>>();
        Self(items)
    }
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub thumbnail: Image,
    pub about: String,
    pub slug: String,
}

impl Skill {
    fn from_item(access_token: &str, space_id: &str, item: Item) -> Result<Self, ParseError> {
        let temp = &item.fields.photo.clone();
        Ok(Self {
            name: item.fields.name.expect("Failed to parse skill name"),
            description: item
                .fields
                .description
                .expect("Failed to parse skill description"),
            slug: item.fields.slug.expect("Failed to parse skill slug"),
            about: item.fields.about.expect("Faild to parse skill about"),
            thumbnail: Image::from_asset_data(
                get_asset_by_id(
                    access_token,
                    space_id,
                    &item
                        .fields
                        .thumbnail
                        .expect(&format!(
                            "Failed to get nested thumbnail from item {:#?}",
                            temp
                        ))
                        .sys
                        .id,
                )
                .expect("Failed to parse image asset data"),
            )
            .expect("Failed to parse image from asset data"),
        })
    }
}

#[derive(Debug, Clone)]
struct BlogPosts(Vec<BlogPost>);

#[derive(Debug, Clone)]
struct BlogPost {
    title: String,
    slug: String,
    content: String,
}

#[derive(Debug)]
pub struct ParseError;
