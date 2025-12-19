pub fn get_client_url() -> String {
  let client_type = crate::config::get_config()
    .client_type
    .unwrap_or("default".to_string());
  let url = if client_type == "default" {
    "https://discord.com"
  } else {
    &format!("https://{client_type}.discord.com")
  };

  url.to_string()
}

pub fn get_client_app_url() -> String {
  let url = get_client_url();
  format!("{url}/app")
}

pub fn get_url_for_guild(guild_id: String) -> String {
  let app_url = get_client_url();
  format!("{app_url}/channels/{guild_id}")
}

pub fn get_url_for_channel(guild_id: String, channel_id: String) -> String {
  let app_url = get_client_url();
  format!("{app_url}/channels/{guild_id}/{channel_id}")
}

pub fn get_url_for_message(guild_id: String, channel_id: String, message_id: String) -> String {
  let app_url = get_client_url();
  format!("{app_url}/channels/{guild_id}/{channel_id}/{message_id}")
}
