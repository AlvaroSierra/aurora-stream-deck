mod actions;

use stream_deck_sdk::action_manager::ActionManager;
use stream_deck_sdk::init;
use crate::actions::ZoomToNavaid;


#[tokio::main]
async fn main() {

	let actions = ActionManager::new().register(vec![
		Box::new(ZoomToNavaid)
	]);

	let init = init(actions, None).await;

	init.connect().await;
}
