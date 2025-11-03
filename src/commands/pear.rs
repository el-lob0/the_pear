use poise::serenity_prelude as serenity;
use crate::util::parse::extract_response;
use crate::util::ai::call_gemini;









pub async fn pear(new_message: serenity::Message, ctx: serenity::Context) {
    let replied_to = new_message.referenced_message.clone();
    match replied_to {
        None => {
            let _ = new_message.reply(ctx, "This command is used on replied-to messages.").await;
        },
        Some(msg) => {
            let content = &msg.content;
            let prompt = format!("I will give you a message and i want you to reformulte it in an aristocratic type of tone. Like old sophisticated english, but not to the point where there is complicated words like shakespeare (like none of those harth type words that end with th). Make sure to translate the right meaning for slang words too. Ignore any links in the message. And if the message is in french, do the same but in french, emulating moliere lamnguage for example. AND REPLY ONLY WITH THE RESPONSE MESSAGE.\n The message: <<{content}>> ");
            let response = call_gemini(prompt.as_str());
            let r = response.unwrap();
            let parsed = extract_response(&r.as_str());
            let author = match &msg.author.global_name {
                None => "Unknown".to_string(),
                Some(auth) => auth.to_string(),
            };
            let _ = new_message
                .reply(
                    ctx,
                    format!("{parsed} \n -- {:?}", author),
                )
                .await;                       
                }
        }
}


