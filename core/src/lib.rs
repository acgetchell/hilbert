use anyhow::Result;
use quick_xml::de::from_str;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Feed {
    entry: Option<Vec<Entry>>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    title: String,
    summary: String,
    id: String,
    author: Vec<Author>,
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
}
#[derive(Debug, Serialize)]
pub struct Paper {
    pub title: String,
    pub abstract_text: String,
    pub authors: Vec<String>,
    pub url: String,
    pub score: f32,
}

async fn fetch_arxiv(query: &str) -> Result<Vec<Paper>> {
    let client = reqwest::Client::builder()
        .user_agent("hilbert.tools (research app; contact: your@email.com)")
        .build()?;

    let resp: reqwest::Response = client
        .get("https://export.arxiv.org/api/query")
        .query(&[
            ("search_query", format!("all:{}", query)),
            ("start", "0".to_string()),
            ("max_results", "10".to_string()),
        ])
        .send()
        .await?
        .error_for_status()?; // fail on non-200

    let response: String = resp.text().await?;

    let feed: Feed = from_str(&response)?;

    let mut papers = Vec::new();

    if let Some(entries) = feed.entry {
        for entry in entries {
            papers.push(Paper {
                title: entry.title.trim().to_string(),
                abstract_text: entry.summary.trim().to_string(),
                authors: entry.author.into_iter().map(|a| a.name).collect(),
                url: entry.id,
                score: 0.0,
            });
        }
    }

    Ok(papers)
}

pub async fn search_papers(query: &str) -> Result<Vec<Paper>> {
    let papers = fetch_arxiv(query).await?;

    // For now, skip embeddings
    // Just return results to verify arXiv works

    Ok(papers)
}
