extern crate discord;
extern crate rand;

use discord::{Discord, ChannelRef, State};
use discord::voice::AudioReceiver;
use discord::model::{Event, UserId, ChannelType};
use std::env;

pub fn main() {
	// Log in to Discord using a bot token from the environment
	let discord = Discord::from_bot_token(
		&env::var("DISCORD_TOKEN").expect("Expected token"),
	).expect("login failed");
	let prefix: &str = "!";

	// Establish and use a websocket connection
	let (mut connection, ready) = discord.connect().expect("connect failed");
	let mut state = State::new(ready);
	let channel_count: usize = state.servers().iter()
		.map(|srv| srv.channels.iter()
			.filter(|chan| chan.kind == ChannelType::Text)
			.count()
		).fold(0, |v, s| v + s);
	println!("[Ready] {} logging {} servers with {} text channels", state.user().username, state.servers().len(), channel_count);

	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("{} says: {}", message.author.name, message.content);
				let mut split = message.content.split(" ");
				let command = split.next().unwrap_or("");
				let argument = split.next().unwrap_or("");

				if message.content == "!test" {
					let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
				} else if message.content == prefix.to_owned() + "quit" {
					    let _ = discord.send_message(message.channel_id, "Go fuck yourself Striped.", "", false);
                }
                if message.content == prefix.to_owned() + "help" {
                    let _ = discord.send_message(message.channel_id, "I can't help you. I'm just a bot.", "", false);
                }
                if message.content == prefix.to_owned() + "about" {
                    let _ = discord.send_message(message.channel_id, "I have no clue. All I know is that I was written by Arroz in a language called Rust, and my code is still being developed.", "", false);
                }
                if message.content == prefix.to_owned() + "uber" {
                    let _ = discord.send_message(message.channel_id, "Fuck off m8.", "", false);
                }
                if message.content == prefix.to_owned() + "ask" {
                    let _ = discord.send_message(message.channel_id, "Not just yet, I'll have the answers soon.", "", false);
                }
				if message.content == prefix.to_owned() + "givemeyourcode" {
					let _ = discord.send_message(message.channel_id, "Okay, here you go. https://github.com/aarroz/rice_bot2", "", false);
				}
				if message.content == prefix.to_owned() {
					let _ = discord.send_message(message.channel_id, "Not a valid command?", "", false);
				}
}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break
			}
			Err(err) => println!("Receive error: {:?}", err)
		}
	}
}
