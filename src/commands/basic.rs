use crate::{Context, Error};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub entry: String,
    pub user_id: Option<String>,
}

#[poise::command(prefix_command, track_edits)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "how we feeling?",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn today(ctx: Context<'_>) -> Result<(), Error> {
    let now = Utc::now();
    let entries = get_entries(now).await?;
    if entries.is_empty() {
        ctx.say("No entries found for today.").await?;
        return Ok(());
    }

    let entries_count = count_entries(entries);
    let entries_count_formatted = entries_count
        .iter()
        .map(|(emotion, count)| format!("{}: {}", emotion, count))
        .collect::<Vec<String>>()
        .join("\n");
    let top_entry = &entries_count[0].0.to_lowercase();

    let message = format!("Today is a {} day.\n{}\nSubmit [howYOUfeel](https://mehrezat.com/howWEfeel/home.html) now!", top_entry, entries_count_formatted);

    ctx.say(message).await?;
    Ok(())
}

pub async fn get_entries(date_time: DateTime<Utc>) -> Result<Vec<Entry>, Error> {
    let client = Postgrest::new("https://qzeihdpxiklgqyflggjv.supabase.co/rest/v1")
        .insert_header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InF6ZWloZHB4aWtsZ3F5ZmxnZ2p2Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDM4Nzk1NTAsImV4cCI6MjA1OTQ1NTU1MH0.jtiUrPywII-ucmMtWuP8INhfkCs1wS8DVmOnODY7Xmg");

    let resp = client
        .from("entries")
        .select("*")
        .gte(
            "created_at",
            date_time.format("%Y-%m-%d").to_string() + "T00:00:00Z",
        )
        .lt(
            "created_at",
            date_time.format("%Y-%m-%d").to_string() + "T23:59:59Z",
        )
        .order("created_at.desc")
        .execute()
        .await;

    if let Err(e) = resp {
        println!("Error fetching entries: {}", e);
        return Err(Error::from(e));
    }

    let resp_text = resp.unwrap().text().await?;

    let entries: Vec<Entry> = serde_json::from_str(&resp_text).map_err(|e| {
        eprintln!("Failed to deserialize from JSON: {}", e);
        e
    })?;

    Ok(entries)
}

pub fn count_entries(entries: Vec<Entry>) -> Vec<(String, u32)> {
    let mut emotion_counts = HashMap::new();

    for entry in entries {
        let emotion_count_entry = emotion_counts.entry(entry.entry).or_insert(0);
        *emotion_count_entry += 1;
    }

    let mut sorted_counts: Vec<(String, u32)> = emotion_counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| {
        let sorted_val = b.1.cmp(&a.1);
        if sorted_val == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            sorted_val
        }
    });

    sorted_counts
}
