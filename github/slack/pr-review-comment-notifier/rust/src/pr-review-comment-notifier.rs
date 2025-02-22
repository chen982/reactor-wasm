#[allow(unused_imports)]
use serde_json::Value;
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let res: Value = match serde_json::from_str(s.as_str()) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!(
                "GitHub webhook payloads parsing failed: {}.",
                e.to_string()
            ))
        }
    };

    let mut event_type: String = String::new();
    let pr_title: &str = res.get("pull_request").unwrap()["title"].as_str().unwrap();
    let mut body: &str = "";
    let mut commenter: &str = "";
    let mut html_url: &str = "";

    match res["action"].as_str() {
        Some(action) => event_type = format!("pr review comment {}", action),
        None => return Err("Parse action failed.".to_string()),
    };


    if let Some(comment) = res.get("comment") {
        body = comment["body"].as_str().unwrap();
        commenter = comment["author_association"].as_str().unwrap();
        html_url = comment["html_url"].as_str().unwrap();
    }


    if event_type != "" {
        return Ok(format!(
            "{}\n{}\n{}\n{}\n{}",
            event_type,
            pr_title,
            body,
            commenter,
            html_url
        ));
    } else {
        return Err(format!(
            "Event type is empty.",
        ))
    }
}
