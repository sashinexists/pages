use super::api::{
    contentful::{AssetData, Item, Items},
    get_asset_by_id,
};
use std::path::PathBuf;
use url::Url;
#[derive(Debug, Clone)]
pub enum PageData {
    None,
    Home(Home),
    Projects(Projects),
    Testimonials(Testimonials),
}

// there is an empty PageData enum (with only None as an option) here by default, everything else is custome

#[derive(Debug, Clone)]
pub struct Home {
    testimonials: Testimonials,
    past_projects: Projects,
    skills: Skills,
}

#[derive(Debug, Clone)]
pub struct Testimonials(Vec<Testimonial>);

#[derive(Debug, Clone)]
pub struct Testimonial {
    author: Person,
    text: String,
    slug: String,
}

#[derive(Debug, Clone)]
pub struct Image {
    title: String,
    src: Url,
    alt: String,
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
                &asset
                    .fields
                    .file
                    .expect("Failed to parse image file path")
                    .url,
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
pub struct Projects(Vec<Projects>);

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
            website_url: Url::parse(
                &item
                    .fields
                    .website
                    .expect("Failed to get project's website"),
            )
            .ok(),
            github_url: Url::parse(
                &item
                    .fields
                    .github_url
                    .expect("Failed to get project's website"),
            )
            .ok(),
            testimonial: todo!(),
            skills: todo!(),
        })
    }
}

#[derive(Debug, Clone)]
struct Skills(Vec<Skill>);

impl Skills {
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
struct Skill {
    name: String,
    description: String,
    thumbnail: Image,
    about: String,
    slug: String,
}

impl Skill {
    fn from_item(access_token: &str, space_id: &str, item: Item) -> Result<Self, ParseError> {
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
                        .photo
                        .expect("Failed to get thumbnail photo")
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
