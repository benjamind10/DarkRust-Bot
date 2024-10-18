mod commands;  // Import the commands module

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use log::{info, error};
use tokio::sync::Mutex;
use std::sync::Arc;

/// User data that is accessible in all commands
struct Data {
    db_pool: PgPool,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Arc<Mutex<Data>>, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Database setup (assuming PostgreSQL)
    let db_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL in environment");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create database pool");

    let data = Arc::new(Mutex::new(Data { db_pool: pool }));

    // Define bot commands using FrameworkOptions
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),  // Register the ping command from commands/ping.rs
            ],
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                info!("Bot is ready!");
                Ok(data.clone())
            })
        })
        .build()
        .await
        .expect("Error creating framework");

    if let Err(why) = framework.start().await {
        error!("Client error: {:?}", why);
    }
}
