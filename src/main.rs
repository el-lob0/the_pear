use poise::serenity_prelude as serenity;
use dotenv;
use std::fs;
use std::path::Path;
use std::io::Write;mod util;

mod commands;
use commands::{ask::ask, crypting::{decrypt, encrypt}, gif::gif, pear::pear};
use commands::crypting::Data;
// use commands::gif::gif;


// struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


// The example from the docs lol
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}


// PING
#[poise::command(prefix_command)]
async fn ping(
    ctx: Context<'_>,
    _msg: Option<serenity::Message>,
) -> Result<(), Error> {

    let response = format!("PONG!");
    ctx.say(response).await?;
    Ok(())
}

// Pong
#[poise::command(prefix_command)]
async fn summon(
    ctx: Context<'_>,
    _msg: Option<serenity::Message>,
) -> Result<(), Error> {

    let response = format!("mao zedong");
    ctx.say(response).await?;
    Ok(())
}




#[tokio::main]
async fn main() {

    let dir_path = "bot_storage";
    let file_path = format!("{}/permission_to_decrypt.txt", dir_path);

    // 1. Create directory if it doesn’t exist
    if !Path::new(dir_path).exists() {
        println!("Directory '{}' not found. Creating...", dir_path);
        fs::create_dir_all(dir_path);
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), encrypt(), decrypt(), gif(), summon()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            on_error: |error| {
                Box::pin(async move {
                    println!("ERROR -> passed prefix and event handler.");
                    match error {
                        poise::FrameworkError::ArgumentParse { error, .. } => {
                            if let Some(error) = error.downcast_ref::<serenity::RoleParseError>() {
                                println!("Found a RoleParseError: {:?}", error);
                            } else {
                                println!("Not a RoleParseError :(");
                            }
                        }
                        other => println!( "{:?}", poise::builtins::on_error(other).await.unwrap()),
                    }
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

    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}


async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.id == 1203281930788012084 && new_message.content.to_lowercase().contains(".pear") {
                let _ = new_message.reply(ctx, "Ferme la toi").await;
            } else {
                if new_message.content.to_lowercase().contains(".pear") {
                    pear(new_message.clone(), ctx.clone()).await;
                }
                if new_message.content.to_lowercase().contains(".ask") {
                    ask(new_message.clone(), ctx.clone()).await;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
