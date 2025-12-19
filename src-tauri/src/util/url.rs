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
