#![allow(unused)]
use carapax::types::{
    KeyboardButton, Message, ReplyKeyboardMarkup
};
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::ChatId,
    Api, App, Context, ExecuteError, Ref,
};

use dotenv::dotenv;
use std::env;

use std::fs::{File, self};
use std::io::{Write, Read};

mod diatonic;
mod scales;
use crate::diatonic::*;

#[tokio::main]
async fn main() {
    let file = File::create("file.txt");
        
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let app = App::new(context, echo);
    LongPoll::new(api, app).run().await
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), ExecuteError> {
    let message_text = message.get_text().unwrap().data.clone();

    let mut text_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("file.txt")
        .unwrap();
        
    if message_text == "/start" || message_text == "Back" {
        let file = File::create("file.txt");

        let button_c = KeyboardButton::new("C");
        let button_d = KeyboardButton::new("D");
        let button_e = KeyboardButton::new("E");
        let button_f = KeyboardButton::new("F");
        let button_g = KeyboardButton::new("G");
        let button_a = KeyboardButton::new("A");
        let button_h = KeyboardButton::new("H");
        
        let key_raw = ReplyKeyboardMarkup::row(
            ReplyKeyboardMarkup::default(), vec![
                button_c, button_d, button_e, button_f, button_g, button_a, button_h
            ]
        );
        
        let keyboard = ReplyKeyboardMarkup::resize_keyboard(key_raw, true);
        let text = "Root note";
        let sendmessage = SendMessage::new(chat_id, text);
        let button_message = SendMessage::reply_markup(sendmessage, keyboard);
        
        api.execute(button_message).await?;
    } else if message_text == "C" || message_text == "D" || message_text == "E" || message_text == "F"
    || message_text == "G" || message_text == "A" || message_text == "H" {
        let file = File::create("file.txt");
        let to_file = format!("{} ", message_text);
        text_file.write_all(to_file.as_bytes());
        
        let button_flat = KeyboardButton::new("b / Flat");
        let button_clean = KeyboardButton::new("Clean");
        let button_sharp = KeyboardButton::new("# / Sharp");
        let key_raw = ReplyKeyboardMarkup::row(
            ReplyKeyboardMarkup::default(), vec![button_flat, button_clean, button_sharp]
        );
        let keyboard = ReplyKeyboardMarkup::resize_keyboard(key_raw, true);
        let text = "Accidental";
        
        let sendmessage = SendMessage::new(chat_id, text);
        let button_message = SendMessage::reply_markup(sendmessage, keyboard);
        
        api.execute(button_message).await?;
    } else if message_text == "b / Flat" || message_text == "Clean" || message_text == "# / Sharp" {
        if message_text == "Clean" {
            text_file.write_all(b" ");
        } else {
            let to_file = format!("{}", &message_text[..1]);
            text_file.write_all(to_file.as_bytes());
        }
        
        let button_back = KeyboardButton::new("Back");
        let button_major = KeyboardButton::new("Major");
        let button_dorian = KeyboardButton::new("Dorian");
        let button_phrygian = KeyboardButton::new("Phrygian");
        let button_lydian = KeyboardButton::new("Lydian");
        let button_mixolydian = KeyboardButton::new("Mixolydian");
        let button_minor = KeyboardButton::new("Minor");
        let button_locrian = KeyboardButton::new("Locrian");
        
        let keys = vec![
            vec![button_major, button_dorian, button_phrygian, button_lydian], 
            vec![button_mixolydian, button_minor, button_locrian, button_back]
            
        ];
        let key_raw = ReplyKeyboardMarkup::row(
            ReplyKeyboardMarkup::from_vec(keys),
            vec![]
        );
        let keyboard = ReplyKeyboardMarkup::resize_keyboard(key_raw, true);
        let text = "Mode";
        
        let sendmessage = SendMessage::new(chat_id, text);
        let button_message = SendMessage::reply_markup(sendmessage, keyboard);
        
        api.execute(button_message).await?;
    } else if message_text == "Major" || message_text == "Dorian" || 
        message_text == "Phrygian" || message_text == "Lydian" || 
        message_text == "Mixolydian" || message_text == "Minor" || message_text == "Locrian" {
        text_file.write_all(message_text.as_bytes());
        
        let key_row = ReplyKeyboardMarkup::row(ReplyKeyboardMarkup::default(), vec![]);
        let keyboard = ReplyKeyboardMarkup::resize_keyboard(key_row, true);
        let contents = fs::read_to_string("file.txt").expect("Should have been able to read the file");

        // "C b Major"
        let note = contents[..1].chars().nth(0).unwrap();
        let sign = contents[2..3].chars().nth(0).unwrap();
        let scale = &contents[3..];
        let mut answer = format!("{}{} {}:", note, sign, scale);
        for i in scale_builder(note, sign, scale) {
            answer = format!("{} {}", answer, i)
        }

        let sendmessage = SendMessage::new(chat_id.clone(), answer);
        let button_message = SendMessage::reply_markup(sendmessage, keyboard);
        api.execute(button_message).await?;

        let button_c = KeyboardButton::new("C");
        let button_d = KeyboardButton::new("D");
        let button_e = KeyboardButton::new("E");
        let button_f = KeyboardButton::new("F");
        let button_g = KeyboardButton::new("G");
        let button_a = KeyboardButton::new("A");
        let button_h = KeyboardButton::new("H");
        
        let key_raw = ReplyKeyboardMarkup::row(
            ReplyKeyboardMarkup::default(), vec![
                button_c, button_d, button_e, button_f, button_g, button_a, button_h
            ]
        );
        let keyboard = ReplyKeyboardMarkup::resize_keyboard(key_raw, true);
        let text = "Root note";
        let sendmessage = SendMessage::new(chat_id, text);
        let button_message = SendMessage::reply_markup(sendmessage, keyboard);
        
        api.execute(button_message).await?;
    } else {
        let button_c = KeyboardButton::new("C");
        let button_d = KeyboardButton::new("D");
        let button_e = KeyboardButton::new("E");
        let button_f = KeyboardButton::new("F");
        let button_g = KeyboardButton::new("G");
        let button_a = KeyboardButton::new("A");
        let button_h = KeyboardButton::new("H");
        
        let key_raw = ReplyKeyboardMarkup::row(
            ReplyKeyboardMarkup::default(), vec![
                button_c, button_d, button_e, button_f, button_g, button_a, button_h
            ]
        );
        let keyboard = ReplyKeyboardMarkup::resize_keyboard(key_raw, true);
        let text = "Root note";
        let sendmessage = SendMessage::new(chat_id, text);
        let button_message = SendMessage::reply_markup(sendmessage, keyboard);
        
        api.execute(button_message).await?;
    };
    Ok(())
}
