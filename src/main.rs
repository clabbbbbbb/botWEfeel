mod commands;
use poise::serenity_prelude as serenity;
use std::env::var;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

#[tokio::main]
async fn main() {
    let token = var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable is required");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    println!("Starting up...");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::basic::help(), commands::basic::today()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("hwf!".into()),
                ..Default::default()
            },
            on_error: |error| {
                Box::pin(async move {
                    println!("An error occurred. {}", error);
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    println!("Bot is running.");

    client.unwrap().start().await.unwrap()
}
