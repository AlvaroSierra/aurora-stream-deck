use std::io::Error;
use aurora_api::aurora::Aurora;
use aurora_api::AuroraAPI;
use serde::Deserialize;
use stream_deck_sdk::action::Action;
use stream_deck_sdk::events::events::KeyEvent;
use stream_deck_sdk::get_settings;
use stream_deck_sdk::stream_deck::StreamDeck;
use crate::actions::ZoomToNavaid;

pub struct TransferToPosition;

#[derive(Deserialize, Debug)]
struct TransferToPositionSettings{
	pub station: String
}

#[async_trait]
impl Action for ZoomToNavaid{
	fn uuid(&self) -> &str {
		"com.alvaro.aurorastream.transfertostation"
	}

	async fn on_key_down(&self, e: KeyEvent, sd: StreamDeck) {

		let callsign = match Aurora::get_selected_traffic() {
			Ok(callsign) => callsign,
			Err(error) => {
				sd.log(format!("[{}] {}", Self::uuid(), error));
				return ()
			}
		};

		let station = match get_settings::<TransferToPositionSettings>(e.payload.settings) {
			None => {
				sd.log(format!("[{}] Couldn't fetch settings from streamdeck correctly", Self::uuid()));
				return ()
			}
			Some(settings) => settings.station
		};

		match Aurora::transfer_traffic_to(&station, &callsign){
			Ok(_) => (),
			Err(a) => {sd.log(format!("[{}] {}", self.uuid(), a)).await}
		}

	}
}
