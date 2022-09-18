use tauri::regex::{Regex};
use reqwest;

#[tauri::command]
pub async fn localize_imports(css: String) -> String {
  let reg = Regex::new(r"@import url\(.*\);").unwrap();
  let url_reg = Regex::new(r"\((.*)\)").unwrap();
  // let matches = reg.find_iter(&css);
  let mut new_css = css.clone();

  while reg.is_match(new_css.clone().as_str()) {
    let first_match = reg.find_iter(&new_css).next().unwrap();
    let url = url_reg.captures(first_match.as_str()).unwrap().get(1).unwrap().as_str();

    if url.is_empty() {
      continue;
    }

    let response = match reqwest::get(url).await {
      Ok(r) => r,
      Err(e) => {
        println!("Request failed: {}", e);

        continue;
      }
    };
    let text = response.text().await.unwrap();

    new_css = new_css.replace(first_match.as_str(), text.as_str());
  }

  new_css
}