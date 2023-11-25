mod actions;

use stream_deck_sdk::action_manager::ActionManager;
use stream_deck_sdk::init;
use crate::actions::{ZoomToNavaid, TransferToPosition, IntercomAccept, IntercomReject, IntercomCall, Release, SetSpeed, SetWP, SetAlt};


#[tokio::main]
async fn main() {

	let actions = ActionManager::new().register(vec![
		Box::new(ZoomToNavaid),
		Box::new(TransferToPosition),
		Box::new(IntercomCall),
		Box::new(IntercomReject),
		Box::new(IntercomAccept),
		Box::new(Release),
		Box::new(SetSpeed),
		Box::new(SetWP),
		Box::new(SetAlt)
	]);

	let init = init(actions, None).await;
	init.connect().await;
}
