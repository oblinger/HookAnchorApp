use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudScanConfig {
    #[serde(rename = "type")]
    pub service_type: String,
    pub enabled: bool,
    pub root: String,
    pub api_key: Option<String>,
    pub credentials: Option<String>,
}

#[derive(Debug)]
pub struct NotionPage {
    pub id: String,
    pub title: String,
    pub parent_path: String,
    pub last_modified: DateTime<Utc>,
    pub url: String,
}

pub struct NotionScanner {
    api_key: String,
    client: reqwest::blocking::Client,
}

impl NotionScanner {
    pub fn new(api_key: String) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", api_key).parse().unwrap(),
        );
        headers.insert("Notion-Version", "2022-06-28".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { api_key, client }
    }

    pub fn scan_all_pages(&self) -> Result<Vec<NotionPage>, String> {
        println!("[NOTION] Starting scan of all accessible pages...");
        
        let mut all_pages = Vec::new();
        let mut has_more = true;
        let mut start_cursor: Option<String> = None;

        while has_more {
            let mut body = serde_json::json!({
                "filter": {
                    "property": "object",
                    "value": "page"
                },
                "page_size": 100
            });

            if let Some(cursor) = start_cursor {
                body["start_cursor"] = serde_json::json!(cursor);
            }

            let response = self
                .client
                .post("https://api.notion.com/v1/search")
                .json(&body)
                .send()
                .map_err(|e| format!("Failed to send request: {}", e))?;

            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().unwrap_or_default();
                return Err(format!("Notion API error {}: {}", status, text));
            }

            let data: serde_json::Value = response
                .json()
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            has_more = data["has_more"].as_bool().unwrap_or(false);
            start_cursor = data["next_cursor"].as_str().map(String::from);

            if let Some(results) = data["results"].as_array() {
                for page in results {
                    if let Some(parsed) = self.parse_page(page) {
                        all_pages.push(parsed);
                    }
                }
            }
        }

        Ok(all_pages)
    }

    fn parse_page(&self, page: &serde_json::Value) -> Option<NotionPage> {
        let id = page["id"].as_str()?.to_string();
        
        let title = self.extract_title(page).unwrap_or_else(|| "Untitled".to_string());
        
        let parent_path = self.extract_parent_path(page);
        
        let last_modified = page["last_edited_time"]
            .as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let url = format!("https://www.notion.so/{}", id.replace("-", ""));

        Some(NotionPage {
            id,
            title,
            parent_path,
            last_modified,
            url,
        })
    }

    fn extract_title(&self, page: &serde_json::Value) -> Option<String> {
        if let Some(props) = page["properties"].as_object() {
            for (_key, value) in props {
                if value["type"] == "title" {
                    if let Some(title_array) = value["title"].as_array() {
                        if let Some(first) = title_array.first() {
                            return first["plain_text"].as_str().map(String::from);
                        }
                    }
                }
            }
        }
        None
    }

    fn extract_parent_path(&self, page: &serde_json::Value) -> String {
        if let Some(parent) = page["parent"].as_object() {
            if let Some(parent_type) = parent["type"].as_str() {
                match parent_type {
                    "workspace" => return "/".to_string(),
                    "page_id" => {
                        if let Some(id) = parent["page_id"].as_str() {
                            return format!("/page/{}", &id[0..8]);
                        }
                    }
                    "database_id" => {
                        if let Some(id) = parent["database_id"].as_str() {
                            return format!("/database/{}", &id[0..8]);
                        }
                    }
                    _ => {}
                }
            }
        }
        "/".to_string()
    }

    pub fn log_pages(&self, pages: &[NotionPage]) {
        for page in pages {
            let modified = page.last_modified.format("%Y-%m-%d");
            let full_path = format!("{}/{}", page.parent_path, page.title);
            println!(
                "[NOTION] {} - {} (ID: {}, Modified: {})",
                full_path,
                page.title,
                &page.id[0..8],
                modified
            );
        }
        println!("[NOTION] Total pages found: {}", pages.len());
    }
}

pub fn scan_cloud_services() {
    // Load the config file directly to get the YAML structure
    let config_path = dirs::home_dir()
        .map(|h| h.join(".config/hookanchor/config.yaml"))
        .expect("Could not find home directory");
    
    let contents = match std::fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[CLOUD] Error reading config: {}", e);
            return;
        }
    };
    
    let config: serde_yaml::Value = match serde_yaml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[CLOUD] Error parsing config YAML: {}", e);
            return;
        }
    };
    
    if let Some(cloud_roots) = config["popup_settings"]["cloud_scan_roots"].as_sequence() {
        for root_config in cloud_roots {
            let service_type = root_config["type"].as_str().unwrap_or("");
            let enabled = root_config["enabled"].as_bool().unwrap_or(false);

            if !enabled {
                continue;
            }

            match service_type {
                "notion" => {
                    if let Some(api_key) = root_config["api_key"].as_str() {
                        let expanded_key = if api_key.starts_with("${") && api_key.ends_with("}") {
                            let env_var = &api_key[2..api_key.len() - 1];
                            std::env::var(env_var).unwrap_or_else(|_| api_key.to_string())
                        } else {
                            api_key.to_string()
                        };

                        if expanded_key.starts_with("ntn_") || expanded_key.starts_with("secret_") {
                            println!("[NOTION] Scanning with API key...");
                            let scanner = NotionScanner::new(expanded_key);
                            match scanner.scan_all_pages() {
                                Ok(pages) => scanner.log_pages(&pages),
                                Err(e) => eprintln!("[NOTION] Error scanning: {}", e),
                            }
                        } else {
                            eprintln!("[NOTION] Invalid API key format");
                        }
                    }
                }
                "google_drive" => {
                    println!("[GDRIVE] Google Drive scanning not yet implemented");
                }
                _ => {}
            }
        }
    }
}