mod actions;

use stream_deck_sdk::action_manager::ActionManager;
use stream_deck_sdk::init;
use crate::actions::{ZoomToNavaid, TransferToPosition, IntercomAccept, IntercomReject, IntercomCall};


#[tokio::main]
async fn main() {

	let actions = ActionManager::new().register(vec![
		Box::new(ZoomToNavaid),
		Box::new(TransferToPosition),
		Box::new(IntercomCall),
		Box::new(IntercomReject),
		Box::new(IntercomAccept)
	]);

	let init = init(actions, None).await;

	init.connect().await;
}
