mod queries;

use queries::ProjectQuery;
use serde_json::{json, Value};
use reqwest::blocking::Client;
use std::env;

fn cursor(page_info: &Value) -> String {
    if let Value::Bool(has_next_page) = page_info["hasNextPage"] { 
        if has_next_page {
            if let Value::String(end_cursor) = &page_info["endCursor"] {
                return end_cursor.to_string()
            }
        }
    }

    String::new()
}

fn main() {
    let token = env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN to be set");
    let client = Client::new();
    let gitlab_server = env::args().nth(1).expect("the GitLab server to be provided");
    let namespace_id = env::args().nth(2).expect("the namespace id to be provided");

    let mut query = ProjectQuery::new(&namespace_id);

    while query.has_next_page() {
        let query_str = query.to_string();
        let content: String = client.post(format!("https://{}/api/graphql", gitlab_server))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .body(json!({"query": query_str}).to_string())
            .send().expect("the result to be valid")
            .text().expect("this to be valid json");

        let json:Value = serde_json::from_str(&content).expect("the content to be valid json");
        let namespace = &json["data"]["namespace"]["projects"];
        let groups = &json["data"]["group"]["descendantGroups"];

        query.set_namespace_projects_cursor(&cursor(&namespace["pageInfo"]));

        if let Value::Array(ps) = &namespace["nodes"] {
            for p in ps {
                if let Value::String(url) = &p["httpUrlToRepo"] {
                    println!("{}", url);
                }
            }
        }

        if let Value::Array(gs) = &groups["nodes"] {
            for group in gs {
                // This doesn't properly paginate projects yet
                if let Value::Array(ps) = &group["projects"]["nodes"] {
                    for p in ps {
                        if let Value::String(url) = &p["httpUrlToRepo"] {
                            println!("{}", url);
                        }
                    }
                }
            }
        }

        query.set_groups_cursor(&cursor(&groups["pageInfo"]))
    }
}
