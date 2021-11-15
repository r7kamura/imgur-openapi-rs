/*
 * Imgur API
 *
 * Imgur's API exposes the entire Imgur infrastructure via a standardized programmatic interface. Using Imgur's API, you can do just about anything you can do on imgur.com, while using your programming language of choice.
 *
 * The version of the OpenAPI document: 0.1.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,
    #[serde(rename = "account_url", skip_serializing_if = "Option::is_none")]
    pub account_url: Option<String>,
    #[serde(rename = "ad_type")]
    pub ad_type: i32,
    #[serde(rename = "ad_url")]
    pub ad_url: String,
    #[serde(rename = "animated")]
    pub animated: bool,
    #[serde(rename = "bandwidth")]
    pub bandwidth: i32,
    #[serde(rename = "deletehash", skip_serializing_if = "Option::is_none")]
    pub deletehash: Option<String>,
    #[serde(rename = "datetime")]
    pub datetime: i32,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "favorite")]
    pub favorite: bool,
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "in_gallery")]
    pub in_gallery: bool,
    #[serde(rename = "in_most_viral")]
    pub in_most_viral: bool,
    #[serde(rename = "is_ad")]
    pub is_ad: bool,
    #[serde(rename = "link")]
    pub link: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(rename = "section", skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "views")]
    pub views: i32,
    #[serde(rename = "vote", skip_serializing_if = "Option::is_none")]
    pub vote: Option<String>,
    #[serde(rename = "width")]
    pub width: i32,
}

impl Image {
    pub fn new(ad_type: i32, ad_url: String, animated: bool, bandwidth: i32, datetime: i32, favorite: bool, height: i32, id: String, in_gallery: bool, in_most_viral: bool, is_ad: bool, link: String, size: i32, tags: Vec<String>, _type: String, views: i32, width: i32) -> Image {
        Image {
            account_id: None,
            account_url: None,
            ad_type,
            ad_url,
            animated,
            bandwidth,
            deletehash: None,
            datetime,
            description: None,
            favorite,
            height,
            id,
            in_gallery,
            in_most_viral,
            is_ad,
            link,
            name: None,
            nsfw: None,
            section: None,
            size,
            tags,
            title: None,
            _type,
            views,
            vote: None,
            width,
        }
    }
}


