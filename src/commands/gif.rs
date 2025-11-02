use std::fs;
use crate::util::file_dl;
use poise::PrefixContext;


struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// GIF
#[poise::command(prefix_command)]
pub async fn gif(
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

                let _result = file_dl::download_image(link);

                // let attachment = serenity::CreateAttachment::path("./image_store/image.gif");

                let bytes = fs::read("./bot_storage/image.gif");
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
