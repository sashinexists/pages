use reqwest;
use serde::Deserialize;
use serde_json;

use self::contentful::{AssetData, Items};

#[derive(Debug)]
pub enum ContentfulFetchError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
}

fn fetch_and_parse_data<T>(url: &str) -> Result<T, ContentfulFetchError>
where
    T: for<'a> Deserialize<'a> + std::fmt::Debug,
{
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .send()
        .map_err(ContentfulFetchError::ReqwestError)?;

    if !response.status().is_success() {
        return response
            .error_for_status()
            .map(|_| unreachable!())
            .map_err(ContentfulFetchError::ReqwestError);
    }

    let json: serde_json::Value = response
        .json()
        .map_err(ContentfulFetchError::ReqwestError)?;
    serde_json::from_value(json).map_err(ContentfulFetchError::SerdeJsonError)
}

pub fn get_testimonials_data(
    access_token: &str,
    space_id: &str,
) -> Result<Items, ContentfulFetchError> {
    let url = format!(
        "https://cdn.contentful.com/spaces/{}/environments/master/entries?content_type=testimonial&access_token={}&order=sys.updatedAt",
        space_id, access_token
    );
    fetch_and_parse_data(&url)
}

pub fn get_skills_data(access_token: &str, space_id: &str) -> Result<Items, ContentfulFetchError> {
    let url = format!("https://cdn.contentful.com/spaces/{space_id}/environments/master/entries?content_type=skill&access_token={access_token}&order=sys.updatedAt");
    fetch_and_parse_data(&url)
}

pub fn get_past_projects_data(
    access_token: &str,
    space_id: &str,
) -> Result<Items, ContentfulFetchError> {
    let url = format!("https://cdn.contentful.com/spaces/{space_id}/environments/master/entries?content_type=pastProject&access_token={access_token}&order=-sys.createdAt");
    fetch_and_parse_data(&url)
}

pub fn get_person_data_by_id(
    access_token: &str,
    space_id: &str,
    id: &str,
) -> Result<Items, ContentfulFetchError> {
    let url = format!(
        "https://cdn.contentful.com/spaces/{}/environments/master/entries?content_type=person&access_token={}&order=sys.updatedAt&sys.id={}",
        space_id, access_token, id
    );
    fetch_and_parse_data(&url)
}

pub fn get_asset_by_id(
    access_token: &str,
    space_id: &str,
    id: &str,
) -> Result<AssetData, ContentfulFetchError> {
    let url = format!(
        "https://cdn.contentful.com/spaces/{}/environments/master/assets/{}?access_token={}",
        space_id, id, access_token
    );
    fetch_and_parse_data(&url)
}
pub mod contentful {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize)]
    pub struct AssetData {
        pub fields: Fields,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Items {
        pub items: Vec<Item>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Item {
        pub fields: Fields,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Fields {
        pub author: Option<NestedSys>,
        pub text: Option<String>,
        pub name: Option<String>,
        pub slug: Option<String>,
        pub photo: Option<NestedSys>,
        pub website: Option<String>,
        pub title: Option<String>,
        pub organisation: Option<String>,
        pub description: Option<String>,
        pub file: Option<File>,
        pub about: Option<String>,
        pub screenshot: Option<NestedSys>,
        pub github_url: Option<String>,
        pub testimonial: Option<NestedSys>,
        pub skills: Vec<NestedSys>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct NestedSys {
        pub sys: Sys,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Sys {
        pub id: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct File {
        pub url: String,
    }
}
