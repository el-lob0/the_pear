// AI API CALL
use poise::serenity_prelude as serenity;
use crate::util::parse::extract_response;
use crate::util::ai::call_gemini;
use simple_db;
use chrono::Local;





fn check_usr_limit(usr_id: u64) ->  usize {
    let usr_requests = simple_db::SimpleDB::find_database("usr_requests.txt");
    let mut db = usr_requests.unwrap();
    let max_requests = 5;

    let now = Local::now();
    let formatted = now.format("%d/%m/%Y").to_string();

    let key = format!("{}-{}", usr_id, formatted);

    println!("{key}");
    remove_expired_keys(formatted);

    match db.get_value_from_db(&key.to_string()) {
// add new value
        None => {
            let value = 1;
            println!("NONE arm matches");
            let _fail = db.insert_into_db(key.to_string(), value.to_string());
            return max_requests-value
        },  
        Some(c) => {
            let count = c.parse::<usize>().unwrap();

            if count < max_requests {
                let value = count+1;
                let _fail = db.insert_into_db(key.to_string(), value.to_string());
                return max_requests-value
            } else {
                return 0
            }
        },  
    }
    // return number of requests left
}


fn remove_expired_keys(id: String) {
    let usr_requests = simple_db::SimpleDB::find_database("usr_requests.txt");
    let mut db = usr_requests.unwrap();

    let re = regex::Regex::new(r"-(.*)").unwrap();

    // Get current user's date (the day part)
    let db_day = re
        .captures(&id)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string());

    // Collect keys to delete (to avoid mutating while iterating)
    let mut to_delete = Vec::new();

    for key in db.data.keys() {
        let day = re
            .captures(key)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string());

        if day != db_day {
            to_delete.push(key.clone());
        }
    }

    // Now safely delete
    for key in to_delete {
        db.delete_from_db(&key);
    }
}




pub async fn ask(new_message: serenity::Message, ctx: serenity::Context) {
    let usr_limit = check_usr_limit(new_message.author.id.into());
    if usr_limit > 0 {
        let content = new_message.content.clone();

        let prompt = format!("Answer this question in detail (But without exceeding 2000 characters). If the question only contains '.ask' or doesnt have any real question respond with a random food/animal emoji. Use MARKDOWN for formatting. \n The question: <<{content}>> ");

        let response = call_gemini(prompt.as_str());
        let r = response.unwrap();
        let parsed = extract_response(&r.as_str());

        new_message
            .reply(
                ctx,
                format!("{parsed} \n \n -gemini ai \n \n (***{usr_limit} requests** are left for this user.*)"),
            )
            .await;

    } else {
        new_message
            .reply(
                ctx,
                format!("You've hit your daily limit of requests already gng :pensive:"),
            )
            .await;
    }
}
