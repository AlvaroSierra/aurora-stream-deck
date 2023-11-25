use aurora_api::aurora::Aurora;
use aurora_api::AuroraAPI;
use stream_deck_sdk::action::Action;
use stream_deck_sdk::events::events::KeyEvent;
use stream_deck_sdk::stream_deck::StreamDeck;
use async_trait::async_trait;

pub struct IntercomReject;

#[async_trait]
impl Action for IntercomReject {
	fn uuid(&self) -> &str {
		"com.alvaro.aurorastream.intercomreject"
	}

	async fn on_key_down(&self, _e: KeyEvent, sd: StreamDeck) {

		match Aurora::intercom_reject(){
			Ok(_)=> {}
			Err(error) => {sd.log(format!("[{}] {}", self.uuid(), error)).await}
		}
	}
}
