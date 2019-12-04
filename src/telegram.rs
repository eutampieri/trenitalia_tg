use serde::Deserialize;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

#[derive(Deserialize)]
struct TgResult {
    ok: bool,
    result: Vec<Update>,
}

#[derive(Deserialize)]
struct Update {
    update_id: i64,
    message: Message,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Chat {
    pub id: i64
}

#[derive(Deserialize, Clone, Debug)]
pub struct Message {
    pub message_id: i64,
    pub chat: Chat,
    #[serde(default)]
    pub text: Option<String>,
}

pub struct Telegram {
    token: String,
    last_update: Option<i64>,
}

impl Telegram {
    pub fn from(token: &str) -> Self {
        Telegram{token: token.to_string(), last_update: None}
    }
    fn base_url(&self) -> String {
        format!("https://api.telegram.org/bot{}/", &self.token)
    }
    pub fn read_messages(&mut self) -> Result<Vec<Message>, reqwest::Error> {
        println!("{:?}", self.last_update);
        let updates: TgResult = reqwest::get(format!(
            "{}getUpdates?timeout=10{}",
            self.base_url(),
            {
                if let Some(o) = &self.last_update {
                    format!("&offset={}", o)
                } else {
                    "".to_string()
                }
            }
        ).as_str())?.json()?;
        if updates.result.len() == 0 {
            return Ok(Vec::new());
        }
        self.last_update = Some(updates.result[&updates.result.len() - 1].update_id + 1);
        Ok(updates.result.iter().map(|x| x.message.clone()).collect())
    }
    pub fn send_message(&self, message: &Message) -> Result<(), reqwest::Error> {
        reqwest::get(format!(
            "{}sendMessage?chat_id={}&text={}",
            self.base_url(),
            message.chat.id,
            utf8_percent_encode(message.text.clone().unwrap().as_str(), FRAGMENT)).as_str()
        )?;
        Ok(())
    }
}