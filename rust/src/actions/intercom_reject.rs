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

pub struct IntercomReject;

#[derive(Deserialize, Debug)]
struct ZoomToNavidSettings{
	pub navaid: String
}

#[async_trait]
impl Action for IntercomReject {
	fn uuid(&self) -> &str {
		"com.alvaro.aurorastream.intercomreject"
	}

	async fn on_key_down(&self, e: KeyEvent, sd: StreamDeck) {

		match Aurora::intercom_reject(){
			Ok(_)=> {}
			Err(error) => {sd.log(format!("[{}] {}", self.uuid(), error)).await}
		}
	}
}
