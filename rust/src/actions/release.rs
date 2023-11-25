use aurora_api::aurora::Aurora;
use aurora_api::AuroraAPI;
use stream_deck_sdk::action::Action;
use stream_deck_sdk::events::events::KeyEvent;
use stream_deck_sdk::stream_deck::StreamDeck;
use async_trait::async_trait;

pub struct Release;

#[async_trait]
impl Action for Release{
	fn uuid(&self) -> &str {
		"com.alvaro.aurorastream.release"
	}

	async fn on_key_down(&self, _e: KeyEvent, sd: StreamDeck) {

		let callsign = match Aurora::get_selected_traffic() {
			Ok(callsign) => callsign,
			Err(error) => {
				sd.log(format!("[{}] {}", self.uuid(), error)).await;
				return ()
			}
		};

		match Aurora::release(&callsign){
			Ok(_) => (),
			Err(a) => {sd.log(format!("[{}] {}", self.uuid(), a)).await}
		}

	}
}
