use parse::extract_response;
use poise::{serenity_prelude as serenity, Prefix, PrefixContext};
mod file_dl;
use dotenv;
use ::serenity::http;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use ::serenity::all::Attachment;
mod parse;

// use serde_json::json;
// use reqwest::Client;
// use std::env;


struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
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
// above is an example




// AI API CALL
use std::process::Command;
use std::env;

fn call_gemini(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = dotenv::var("AI_API_KEY")?;

    println!("{prompt}");
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

    let data = format!(
        r#"{{
            "contents": [
                {{
                    "parts": [
                        {{ "text": "{}" }}
                    ]
                }}
            ]
        }}"#,
        prompt
    );

    let output = Command::new("curl")
        .args([
            "-s",
            "-X", "POST",
            "-H", &format!("x-goog-api-key: {}", api_key),
            "-H", "Content-Type: application/json",
            "-d", &data,
            url,
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}




// GIF
// TODO: GIF conversion command
#[poise::command(prefix_command)]
async fn gif(
    ctx: PrefixContext<'_, Data, Error>,
) -> Result<(), Error> {

    let new_message = ctx.msg;

    let message = new_message.referenced_message.clone().unwrap();



    let files = message.attachments.clone();

    println!("{:?}", files);
    if files.is_empty() {
            ctx.reply(format!("No attachments found!")).await?;
        } else {

            for img in &message.attachments {

                let link = &img.url;

                let result = file_dl::download_image(link);

                // let attachment = serenity::CreateAttachment::path("./image_store/image.gif");

                let bytes = fs::read("./image_store/image.gif");
                // let attachment = serenity::CreateAttachment::bytes(bytes.unwrap(), "lebron_james.gif");
                println!("{:?}", bytes);
                // let reply = poise::CreateReply::default()
                //     .attachment(attachment);
                //
                // ctx.send(reply);
                
            }
    }
    let response = format!("PONG!");
    ctx.say(response).await?;
    Ok(())
}


// PING
#[poise::command(prefix_command)]
async fn ping(
    ctx: Context<'_>,
    msg: Option<serenity::Message>,
) -> Result<(), Error> {

    let response = format!("PONG!");
    ctx.say(response).await?;
    Ok(())
}





#[tokio::main]
async fn main() {

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            on_error: |error| {
                Box::pin(async move {
                    println!("what the hell");
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
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.content.to_lowercase().contains(".pear")
            {
                let mut ref_exists = false;
                let replied_to = new_message.referenced_message.clone();
                match replied_to {
                    None => ref_exists = false,
                    Some(msg) => {
                        let content = msg.content;
                        let prompt = format!("I will give you a message and i want you to reformulte it in an aristocratic type of tone. Like old sophisticated english, but not to the point where there is complicated words like shakespeare (like none of those harth type words that end with th). Make sure to translate the right meaning for slang words too. Ignore any links in the message. And if the message is in french, do the same but in french, emulating moliere lamnguage for example. AND REPLY ONLY WITH THE RESPONSE MESSAGE.\n The message: <<{content}>> ");
                        let response = call_gemini(prompt.as_str());
                        let r = response.unwrap();
                        let parsed = extract_response(&r.as_str());
                        let author = match &new_message.author.global_name {
                            None => "Unknown".to_string(),
                            Some(auth) => auth.to_string(),
                        };
                        new_message
                            .reply(
                                ctx,
                                format!("{parsed} \n -- {:?}", author),
                            )
                            .await?;                       
                            }
                    }
            }
            if new_message.content.to_lowercase().contains(".ask")
            {
                let mut ref_exists = false;
                let content = new_message.content.clone();
                let prompt = format!("Answer this question in no more than 1500 characters, and go straight to the point. If the question only contains '.ask' or doesnt have any real question respond with a random food/animal emoji. Use MARKDOWN for formatting. \n The question: <<{content}>> ");
                let response = call_gemini(prompt.as_str());
                let r = response.unwrap();
                let parsed = extract_response(&r.as_str());
                new_message
                    .reply(
                        ctx,
                        format!("{parsed} \n -Gemini AI"),
                    )
                    .await?;                       
            }

            if new_message.content.to_lowercase().contains("!gif")
            {
                let message = new_message.referenced_message.clone().unwrap();



                let files = message.attachments.clone();

                println!("{:?}", files);
                if files.is_empty() {
                        new_message.reply(ctx, format!("No attachments found!")).await?;
                    } else {
                        for img in &message.attachments {

                            let link = &img.url;

                            let result = file_dl::download_image(link);
                            // Path to your local file
                            // let path = "../image_store/image.gif";
                            // let file = tokio::fs::File::open(path);
                            // let clean = file.await.unwrap();
                            // let attachment = serenity::CreateAttachment::file(&clean, "image.gif");
                            
                        }
                }
            }
        }
        _ => {}
    }
    Ok(())
}








