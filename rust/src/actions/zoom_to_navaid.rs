use aurora_api::aurora::Aurora;
use aurora_api::AuroraAPI;
use serde::{Deserialize, Serialize};
use stream_deck_sdk::action::Action;
use stream_deck_sdk::events::events::KeyEvent;
use stream_deck_sdk::get_settings;
use stream_deck_sdk::stream_deck::StreamDeck;
use async_trait::async_trait;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;

pub struct ZoomToNavaid;

#[derive(Serialize, Deserialize, Debug)]
struct ZoomToNavidSettings{
	pub navaid: String
}

#[async_trait]
impl Action for ZoomToNavaid{
	fn uuid(&self) -> &str {
		"com.alvaro.aurorastream.zoomtonavid"
	}

	async fn on_key_down(&self, e: KeyEvent, sd: StreamDeck) {
		let settings = get_settings::<ZoomToNavidSettings>(e.payload.settings);

		if let Some(settings) = &settings {
			Aurora::zoom_to_navaid(&settings.navaid).expect("TODO: panic message");
		}
	}
}
