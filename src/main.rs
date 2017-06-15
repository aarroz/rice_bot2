extern crate discord;

use discord::{Discord, ChannelRef, State};
use discord::voice::AudioReceiver;
use discord::model::{Event, UserId, ChannelType};
use std::env;

struct VoiceTest;

impl AudioReceiver for VoiceTest {
	fn speaking_update(&mut self, ssrc: u32, user_id: UserId, speaking: bool) {
		println!("[{}] is {:?} -> {}", ssrc, user_id, speaking);
	}

	fn voice_packet(&mut self, ssrc: u32, sequence: u16, timestamp: u32, stereo: bool, _data: &[i16]) {
		println!("[{}] ({}, {}) stereo = {}", ssrc, sequence, timestamp, stereo);
	}
}

pub fn main() {
	// Log in to Discord using a bot token from the environment
	let discord = Discord::from_bot_token(
		&env::var("DISCORD_TOKEN").expect("Expected token"),
	).expect("login failed");

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
				if message.content == "!test" {
					let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
				} else if message.content == "!quit" {
					    println!("Quitting.");
					    break
                }
                if message.content == "!help" {
                    let _ = discord.send_message(message.channel_id, "I can't help you. I'm just a bot.", "", false);
                }
                if message.content == "!about" {
                    let _ = discord.send_message(message.channel_id, "I have no clue. All I know is that I was written by Arroz in a language called Rust, and my code is still being developed.", "", false);
                }
                if message.content == "!uber" {
                    let _ = discord.send_message(message.channel_id, "Fuck off m8.", "", false);
                }
                if message.content == "!ask" {
                    let _ = discord.send_message(message.channel_id, "Not just yet, I'll have the answers soon.", "", false);
                }
				if message.content == "!givemeyourcode" {
					let _ = discord.send_message(message.channel_id, "Okay, here you go. https://github.com/aarroz/rice_bot2", "", false);
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
