use poise::{serenity_prelude as serenity, PrefixContext, CreateReply};
use rand::Rng;
use std::fs;
use simple_db::SimpleDB;


pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;




// ---- encrypt ----
#[poise::command(slash_command)]
pub async fn encrypt(

    ctx: Context<'_>,
    #[description = "Message to encrypt"] message: String,

) -> Result<(), Error> {

    let d = SimpleDB::find_database("./bot_storage/encrypted.txt");
    let mut db = d.unwrap();
    let message_id = ctx.id();

    let mut key = generate_new_protocol().to_string();
    if key.len() > 10 { key.pop(); }

    let _ = db.insert_into_db(message_id.to_string(), key.clone());

    // convert key digits '0'..'9' -> 0..9 (key is numeric from your generator)
    let shifts: Vec<u8> = key.bytes().map(|b| b.saturating_sub(b'0')).collect();

    let mut new_message = String::with_capacity(message.len());
    let mut j = 0usize;


    for ch in message.chars() {
        if j == shifts.len() { j = 0; }
        if ch.is_ascii() {
            let base = b' ';        // start from space (32) instead of '!' (33)
            let range = 95u8;       // 32..=126 inclusive (printable + space)
            let chb = ch as u8;
            let shift = shifts[j] % range;
            let shifted = (chb - base + shift) % range + base;
            new_message.push(shifted as char);
            j += 1;
        } else {
            new_message.push(ch);
        }
    }


    let encrypted_message = format!("{}{}", message_id, new_message);
    let _ = ctx.reply(encrypted_message).await;
    Ok(())
}


// ---- decrypt ----
#[poise::command(prefix_command)]
pub async fn decrypt(

    ctx: PrefixContext<'_, Data, Error>,

) -> Result<(), Error> {

    let permitted: Vec<u64> = fs::read_to_string("./bot_storage/permission_to_decrypt.txt")
        .expect("Failed to read file")
        .lines()                               // split by newline
        .filter_map(|line| line.trim().parse::<u64>().ok()) // try parsing each line to u64
        .collect();

    let mut allowed = false;
    for i in permitted {
        if ctx.msg.author.id == i {
            allowed = true;
        }
    }

    if !allowed {
        ctx.send(
            CreateReply::default()
                .content(format!(":face_with_monocle: You do not have permission to read this message lol."))
                .ephemeral(true)
        ).await?;
        return Ok(());
    }

    let message = ctx.msg.referenced_message.clone();

    let ref_message = message.unwrap().content;

    let (part1, part2) = ref_message.split_at(19);

    let message_id = part1.to_string();
    let encrypted_content = part2;

    let d = SimpleDB::find_database("./bot_storage/encrypted.txt");
    let mut db = d.unwrap();
    let key = db.get_value_from_db(&message_id);

    if key.is_none() {
        ctx.say("No key found for this message!").await?;
        return Ok(());
    }

    let key = key.unwrap();

    let shifts: Vec<u8> = key.bytes().map(|b| b.saturating_sub(b'0')).collect();

    let mut new_message = String::with_capacity(encrypted_content.len());
    let mut j = 0usize;

    for ch in encrypted_content.chars() {
        if j == shifts.len() { j = 0; }
        if ch.is_ascii() {
            let base = b' ';
            let range = 95u8;
            let chb = ch as u8;
            let shift = shifts[j] % range;
            let shifted = (chb - base + range - shift) % range + base;
            new_message.push(shifted as char);
            j += 1;
        } else {
            new_message.push(ch);
        }
    }


    let dm_channel = ctx.author().create_dm_channel(&ctx).await?;
    dm_channel.say(&ctx, format!("{}", new_message)).await?;
    // ctx.send(
    //     CreateReply::default()
    //         .content(format!("{}", new_message))
    //         .ephemeral(true)
    // ).await?;

    Ok(())
}



fn generate_new_protocol() -> u64 {

    let mut rng = rand::rng();
    let index: u64 = rng.random_range(2..=9);

    let key: u64 = rng.random_range(9999..=99999);
    let last: u64 = rng.random_range(1..=9);

    let num = (key*index+key/index+key*key)*10+last;
    let protocol = num;

    protocol
}











