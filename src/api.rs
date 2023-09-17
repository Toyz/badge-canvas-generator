use crate::models::{BadgeInfo, AvatarProfileCard};
use serde_json::Value;
use dxr_client::{Client, ClientBuilder, Url, Call};

pub async fn fetch_avatar_profile_card(cid: i64) -> Result<AvatarProfileCard, Box<dyn std::error::Error>> {
    let url = format!("https://www.imvu.com/api/avatarcard.php?cid={}", cid);

    let resp = reqwest::get(&url)
        .await?
        .json::<Value>()
        .await?;

    let avname = resp["avname"].as_str().ok_or("No avatar name found")?;

    let badges = match resp["badge_layout"].as_object() {
        Some(badge_layout) => badge_layout.iter()
            .map(|(key, value)| {
                let badge_info: BadgeInfo = serde_json::from_value(value.clone()).expect("Error parsing badge info");
                (key.clone(), badge_info)
            }).collect(),
        None => Vec::new(),
    };

    // Construct AvatarProfileCard using data
    let avatar_profile_card = AvatarProfileCard {
        avname: avname.to_string(),
        cid: cid,
        badges,
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