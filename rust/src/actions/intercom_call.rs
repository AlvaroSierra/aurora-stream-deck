use aurora_api::aurora::Aurora;
use aurora_api::AuroraAPI;
use serde::{Deserialize, Serialize};
use stream_deck_sdk::action::Action;
use stream_deck_sdk::events::events::KeyEvent;
use stream_deck_sdk::get_settings;
use stream_deck_sdk::stream_deck::StreamDeck;
use async_trait::async_trait;
use std::fs::File;
use std::io::{Error, Write};
use std::ops::Deref;

pub struct IntercomCall;

#[derive(Deserialize, Debug)]
struct IntercomCallSettings {
	pub station: String
}

#[async_trait]
impl Action for IntercomCall {
	fn uuid(&self) -> &str {
		"com.alvaro.aurorastream.intercomcall"
	}

	async fn on_key_down(&self, e: KeyEvent, sd: StreamDeck) {

		let station = match get_settings::<IntercomCallSettings>(e.payload.settings){
			None => {
				sd.log(format!("[{}] Couldn't fetch settings from streamdeck correctly", self.uuid()));
				return ();
			}
			Some(settings) => settings.station
		};

		match Aurora::intercom_call(&station) {
			Ok(_) => (),
			Err(a) => {sd.log(format!("[{}] {}", self.uuid(), a)).await}
		};


	}
}
