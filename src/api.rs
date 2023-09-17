use crate::models::{BadgeInfo, AvatarProfileCard};
use serde_json::Value;
use scraper::Html;
use dxr_client::{Client, ClientBuilder, Url, Call};

pub async fn fetch_avatar_profile_card(cid: i64) -> Result<AvatarProfileCard, Box<dyn std::error::Error>> {
    let url = format!("https://www.imvu.com/api/avatarcard.php?cid={}", cid);

    let resp = reqwest::get(&url)
        .await?
        .json::<Value>()
        .await?;

    let avname = resp["avname"].as_str().ok_or("No avatar name found")?;
    let badge_area_html = resp["badge_area_html"].as_str().ok_or("No badge area HTML found")?;

    let badges = match resp["badge_layout"].as_object() {
        Some(badge_layout) => badge_layout.iter()
            .map(|(key, value)| {
                let badge_info: BadgeInfo = serde_json::from_value(value.clone()).expect("Error parsing badge info");
                (key.clone(), badge_info)
            }).collect(),
        None => Vec::new(),
    };

    // parse the badge_area_html to get the badge positions where the id is the key and the value is a tuple of (x, y) coordinates the id on the html tag is badge-<creator_id>-<creator_badge_index> and the position is locationed in the style as top: and left: values
    let badge_positions = scraper::Html::parse_fragment(badge_area_html)
        .select(&scraper::Selector::parse("img.badgeimg").unwrap())
        .map(|badge| {
            let id = badge.value().attr("id").unwrap().to_string();
            let style = badge.value().attr("style").unwrap().to_string();
            let x = style.split("left:").collect::<Vec<&str>>()[1].split("px").collect::<Vec<&str>>()[0].trim().parse::<i64>().unwrap();
            let y = style.split("top:").collect::<Vec<&str>>()[1].split("px").collect::<Vec<&str>>()[0].trim().parse::<i64>().unwrap();
            (id, x, y)
        }).collect::<Vec<(String, i64, i64)>>();

    // Construct AvatarProfileCard using data
    let avatar_profile_card = AvatarProfileCard {
        avname: avname.to_string(),
        cid: cid,
        badges,
        badge_positions,
    };

    Ok(avatar_profile_card)
}

pub async fn get_user_id_from_avatar_name(avatar_name: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let url = Url::parse("http://secure.imvu.com/api/xmlrpc/gateway.php")?;
    let client: Client = ClientBuilder::new(url)
        .user_agent("dxr-client-example")
        .build();

    let request = Call::new("gateway.getUserIdForAvatarName", avatar_name);
    let response: i32 = client.call(request).await?;
    Ok(response as i64)
}