use crate::util::file_dl;
use poise::PrefixContext;
use poise::serenity_prelude as serenity;
use crate::commands::crypting::Data;


type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[poise::command(prefix_command)]
pub async fn gif(ctx: PrefixContext<'_, Data, Error>) -> Result<(), Error> {

    let new_message = ctx.msg;
    let message = new_message.referenced_message.clone().unwrap();
    let files = message.attachments.clone();


    if files.is_empty() {
        ctx.reply("No attachments found!").await?;
        return Ok(());
    }

    for img in &message.attachments {
        let link = &img.url;

        if let Err(e) = file_dl::download_image(link) {
            eprintln!("Failed to download image: {e}");
            continue;
        }

        let image_result = serenity::CreateAttachment::path("./bot_storage/image.gif").await;

        match image_result {
            Ok(attachment) => {
                println!("Attachment created: {:?}", attachment);

                let reply = poise::CreateReply::default().attachment(attachment);
                ctx.send(reply).await?;
            }
            Err(e) => {
                eprintln!("Error creating attachment: {}", e);
            }
        }
    }

    Ok(())
}

