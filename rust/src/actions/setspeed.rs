use aurora_api::aurora::Aurora;
use aurora_api::AuroraAPI;
use serde::Deserialize;
use stream_deck_sdk::action::Action;
use stream_deck_sdk::events::events::KeyEvent;
use stream_deck_sdk::get_settings;
use stream_deck_sdk::stream_deck::StreamDeck;
use async_trait::async_trait;

pub struct SetSpeed;

#[derive(Deserialize, Debug)]
struct SetSpeedSettings {
    pub spd: String
}

#[async_trait]
impl Action for SetSpeed {
    fn uuid(&self) -> &str {
        "com.alvaro.aurorastream.setspeed"
    }

    async fn on_key_down(&self, e: KeyEvent, sd: StreamDeck) {

        let callsign = match Aurora::get_selected_traffic() {
            Ok(callsign) => callsign,
            Err(error) => {
                sd.log(format!("[{}] {}", self.uuid(), error)).await;
                return ()
            }
        };


        let spd = match get_settings::<SetSpeedSettings>(e.payload.settings) {
            None => {
                sd.log(format!("[{}] Couldn't fetch settings from streamdeck correctly", self.uuid())).await;
                return ()
            }
            Some(settings) => settings.spd
        };

        match Aurora::set_speed_label(&callsign, &spd) {
            Ok(_) => (),
            Err(a) => { sd.log(format!("[{}] {}", self.uuid(), a)).await}
        }
    }
}
