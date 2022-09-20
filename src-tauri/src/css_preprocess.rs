use tauri::regex::Regex;

#[tauri::command]
pub async fn localize_imports(css: String) -> String {
  let reg = Regex::new(r"@import url\(.*\);").unwrap();
  let url_reg = Regex::new(r"\((.*)\)").unwrap();
  let mut new_css = css.clone();

  // First localize images to base64 data representations
  new_css = localize_images(new_css).await;

  while reg.is_match(new_css.clone().as_str()) {
    let first_match = reg.find_iter(&new_css).next().unwrap();
    let url = url_reg
      .captures(first_match.as_str())
      .unwrap()
      .get(1)
      .unwrap()
      .as_str()
      // Remove quotes
      .replace('\'', "")
      .replace('\"', "");

    if url.is_empty() {
      continue;
    }

    let response = match reqwest::get(&url).await {
      Ok(r) => r,
      Err(e) => {
        println!("Request failed: {}", e);
        println!("URL: {}", &url);

        new_css = new_css.replace(first_match.as_str(), "");
        continue;
      }
    };
    let text = response.text().await.unwrap();

    new_css = new_css.replace(first_match.as_str(), text.as_str());
  }

  new_css
}

pub async fn localize_images(css: String) -> String {
  let img_reg = Regex::new(r"url\((.*(jpg|png|jpeg|gif))\)").unwrap();
  let matches = img_reg.captures_iter(&css);
  let mut new_css = css.clone();

  for groups in matches {
    let url = groups.get(1).unwrap().as_str();
    let filetype = url.split('.').last().unwrap();

    if url.is_empty() {
      continue;
    }

    let response = match reqwest::get(url).await {
      Ok(r) => r,
      Err(e) => {
        println!("Request failed: {}", e);
        println!("URL: {}", &url);

        continue;
      }
    };
    let bytes = response.bytes().await.unwrap();
    let b64 = base64::encode(bytes);

    new_css = new_css.replace(
      url,
      format!("data:image/{};base64,{}", filetype, b64).as_str(),
    )
  }

  new_css
}
