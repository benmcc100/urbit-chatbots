use json;
use reqwest::blocking::get;
use urbit_chatbot_framework::{AuthoredMessage, Chatbot, Message};

fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // Split the message up into words (split on whitespace)
    let words = authored_message.message.to_formatted_words();
    // convert spaces to url format ???

    // If the first word is the command `!artbot`
    if words[0] == "!artbot" {
        // get everything after command
        let query = words[1..].join("%20");
        
        let url = format!(
            "https://www.wikiart.org/en/api/2/PaintingSearch?term={}&authSessionKey={}",
            query,
            "null"
        );
        // Send a GET request to the url and parse as string
        let res_string = get(&url).ok()?.text().ok()?;
        // Convert the String to JsonValue
        let res_json = json::parse(&res_string).ok()?;
        // Get the image link from the json
        let img_url = res_json["data"][0]["image"].clone();
        // Check if no price was returned, meaning crypto wasn't found in coingecko api
        if img_url.is_null() {
            // Return error message
            return Some(Message::new().add_text("No painting with that name found."));
        }
        // Else price acquired and is to be returned
        else {
            // Return the price Message
            return Some(Message::new().add_url(&format!("{}", img_url)));
        }
    }

    // Otherwise do not respond to message
    None
}

fn api_authenticate() -> Option<String> {
    // authenticate with wikiart, only necessary once per 2 hours, need to write that as a check or something
    let res_auth_string = get("https://www.wikiart.org/en/Api/2/login?accessCode=91dfc0e379c54835&secretCode=b2d26dabbc897c18")
    .ok()?.text().ok()?;
    let res_auth_json = json::parse(&res_auth_string).ok()?;
    let api_key = res_auth_json["SessionKey"].clone(); 
    println!("{}", api_key);
    return Some(api_key.to_string());
}

fn main() {
    let chat_bot = Chatbot::new_with_local_config(respond_to_message, "~bacrys", "chat-3658");
    //let key = api_authenticate();
    chat_bot.run();
}
