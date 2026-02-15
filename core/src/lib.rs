#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::dbg_macro)]

use anyhow::Result;
use reqwest::Response;
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

    let resp: Response = client
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

    parse_feed(&response)
}

pub async fn search_papers(query: &str) -> Result<Vec<Paper>> {
    let papers = fetch_arxiv(query).await?;

    // For now, skip embeddings
    // Just return results to verify arXiv works

    Ok(papers)
}

fn parse_feed(xml: &str) -> anyhow::Result<Vec<Paper>> {
    let feed: Feed = quick_xml::de::from_str(xml)?;

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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn parse_feed_parses_single_entry() {
        let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <entry>
                <title>Test Paper</title>
                <summary>Test Abstract</summary>
                <id>http://arxiv.org/abs/1234.5678</id>
                <author>
                    <name>John Doe</name>
                </author>
            </entry>
        </feed>
        "#;

        let papers = parse_feed(xml).unwrap();

        assert_eq!(papers.len(), 1);
        assert_eq!(papers[0].title, "Test Paper");
        assert_eq!(papers[0].abstract_text, "Test Abstract");
        assert_eq!(papers[0].authors[0], "John Doe");
        assert_eq!(papers[0].url, "http://arxiv.org/abs/1234.5678");
    }

    #[test]
    fn parse_feed_handles_empty_feed() {
        let xml = r#"
    <feed xmlns="http://www.w3.org/2005/Atom">
    </feed>
    "#;

        let papers = parse_feed(xml).unwrap();
        assert!(papers.is_empty());
    }
}
