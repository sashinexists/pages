use reqwest;
use serde_json;

use self::contentful::{AssetData, PersonData, TestimonialsData};

#[derive(Debug)]
pub enum ContentfulFetchError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
}

pub fn get_testimonials_data(
    access_token: &str,
    space_id: &str,
) -> Result<TestimonialsData, ContentfulFetchError> {
    let url = format!(
        "https://cdn.contentful.com/spaces/{}/environments/master/entries?content_type=testimonial&access_token={}&order=sys.updatedAt",
        space_id, access_token
    );

    let client = reqwest::blocking::Client::new();
    let response = match client.get(&url).send() {
        Ok(resp) => resp,
        Err(err) => return Err(ContentfulFetchError::ReqwestError(err)),
    };

    if !response.status().is_success() {
        println!("Failed to get testimonials JSON data");
        return match response.error_for_status() {
            Ok(_) => unreachable!(), // Because we checked for is_success above
            Err(err) => Err(ContentfulFetchError::ReqwestError(err)),
        };
    }

    let json: serde_json::Value = match response.json() {
        Ok(json) => json,
        Err(err) => return Err(ContentfulFetchError::ReqwestError(err)),
    };
    match serde_json::from_value(json) {
        Ok(testimonials_data) => Ok(testimonials_data),
        Err(err) => Err(ContentfulFetchError::SerdeJsonError(err)),
    }
}

pub fn get_person_data_by_id(
    access_token: &str,
    space_id: &str,
    id: &str,
) -> Result<PersonData, ContentfulFetchError> {
    let url = format!(
        "https://cdn.contentful.com/spaces/{}/environments/master/entries?content_type=person&access_token={}&order=sys.updatedAt&sys.id={}",
        space_id, access_token,id
    );
    let client = reqwest::blocking::Client::new();
    let response = match client.get(&url).send() {
        Ok(resp) => resp,
        Err(err) => return Err(ContentfulFetchError::ReqwestError(err)),
    };

    if !response.status().is_success() {
        println!("Failed to get testimonials JSON data");
        return match response.error_for_status() {
            Ok(_) => unreachable!(), // Because we checked for is_success above
            Err(err) => Err(ContentfulFetchError::ReqwestError(err)),
        };
    }

    let json: serde_json::Value = match response.json() {
        Ok(json) => json,
        Err(err) => return Err(ContentfulFetchError::ReqwestError(err)),
    };
    match serde_json::from_value(json) {
        Ok(person_data) => Ok(person_data),
        Err(err) => Err(ContentfulFetchError::SerdeJsonError(err)),
    }
}

pub fn get_asset_by_id(
    access_token: &str,
    space_id: &str,
    id: &str,
) -> Result<AssetData, ContentfulFetchError> {
    let url = format!(
        "https://cdn.contentful.com/spaces/{space_id}/environments/master/assets/{id}?access_token={access_token}"
    );
    let client = reqwest::blocking::Client::new();
    let response = match client.get(&url).send() {
        Ok(resp) => resp,
        Err(err) => return Err(ContentfulFetchError::ReqwestError(err)),
    };

    if !response.status().is_success() {
        println!("Failed to get testimonials JSON data");
        return match response.error_for_status() {
            Ok(_) => unreachable!(), // Because we checked for is_success above
            Err(err) => Err(ContentfulFetchError::ReqwestError(err)),
        };
    }

    let json: serde_json::Value = match response.json() {
        Ok(json) => json,
        Err(err) => return Err(ContentfulFetchError::ReqwestError(err)),
    };
    match serde_json::from_value(json) {
        Ok(person_data) => Ok(person_data),
        Err(err) => Err(ContentfulFetchError::SerdeJsonError(err)),
    }
}

mod contentful {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize)]
    pub struct AssetData {
        fields: Fields,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct TestimonialsData {
        items: Vec<Item>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct PersonData {
        items: Vec<Item>,
    }

    // you may have to change the shape of this, it also might make sense to change this to image data
    #[derive(Debug, Clone, Deserialize)]
    pub struct PhotoData {
        items: Vec<Item>,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct Item {
        fields: Fields,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct Fields {
        author: Option<NestedSys>,
        text: Option<String>,
        name: Option<String>,
        slug: Option<String>,
        photo: Option<NestedSys>,
        website: Option<String>,
        title: Option<String>,
        organisation: Option<String>,
        description: Option<String>,
        file: Option<File>,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct NestedSys {
        sys: Sys,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct Sys {
        id: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct File {
        url: String,
    }
}
