use tokio_tungstenite;

use rand::Rng;

use tokio::{runtime, task::JoinHandle, time};

use std::time::Duration;

use futures::prelude::*;
use futures_util::{sink::SinkExt, stream::StreamExt};

use crate::data::Snapshot;

pub struct Stream {
    pub stream_handle_spawn: JoinHandle<Result<(), ()>>,
    pub runtime: runtime::Runtime,
    pub pair_id: Box<str>,
}

impl Stream {
    pub fn new<'a, F>(pair_id: String, mut handler: F) -> Result<Self, ()>
    where
        F: FnMut(Snapshot) -> Result<(), ()> + Send + Sync + 'static,
    {
        let pair_id_str = pair_id.clone().into_boxed_str();
        let rt_main = runtime::Runtime::new().unwrap();
        let rt_heartbeat = rt_main.handle().clone();

        let stream = Stream {
			stream_handle_spawn: rt_main.spawn(async {
				let url = generate_ws_url();
				tokio_tungstenite::connect_async (
					&url
				).then(|stream_response| async move {
					stream_response.expect("Failed to get tokio_tungstenite::connect_async(..)")
				}).then(|(mut stream, _response)| async move {
					if stream.next().await.unwrap().unwrap().to_text().unwrap() == "o" {
						Ok(stream.split())
					} else {
						Err(())
					}
				}).and_then(|(mut tx, mut rx)| async move {
					tx.send(format !("[\"{{\\\"_event\\\":\\\"bulk-subscribe\\\",\\\"tzID\\\":\\\"8\\\",\\\"message\\\":\\\"pid-{}:\\\"}}\"]", &pair_id).into())
            .await
						.expect("Expect tx.send(bulk-subscribe, tzID, pid) to server");
					tx.send("[\"{\\\"_event\\\":\\\"UID\\\",\\\"UID\\\":0}\"]".into())
						.await
						.expect("Expect tx.send(UID=0) to server");

					rt_heartbeat
						.spawn(async move {
							loop {
								tx.send("[\"{\\\"_event\\\":\\\"heartbeat\\\",\\\"data\\\":\\\"h\\\"}\"]".into())
									.await
									.expect("Expect tx.send(heartbeat) to server");
								time::sleep(Duration::from_millis(3200u64)).await;
							}
						});

					let key = format!("pid-{}::{{", pair_id);
					let key = key.as_str();

					while let Some(msg) = rx.next().await {
						let msg = msg.unwrap();
						let msg = msg.to_text().unwrap();
						if msg.contains(key) {
							let stop = handler(
								Snapshot::from_str (
									msg
								)
							);
							if let Err(_) = stop {
								return Ok(());
							}
						}
					}

					println !("EOD");
					Ok(())
				}).or_else(|e| async move {
					println!("Failed: {:?}", e);
					Err(e)
				}).await
			}),
			runtime: rt_main,
			pair_id: pair_id_str,
		};

        Ok(stream)
    }
}

pub fn generate_ws_url() -> String {
    let mut rnd = rand::thread_rng();
    return format!(
        "wss://streaming.forexpros.com/echo/{:03x}/{:08x}/websocket",
        rnd.gen::<u16>(),
        rnd.gen::<u32>(),
    );
}
