// Copyright (C) 2024 OpenTier FZCO
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use log::error;
use prost::Message;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use zenoh::handlers::FifoChannelHandler;
use zenoh::pubsub::Subscriber;
use zenoh::sample::Sample;
use zenoh::Session;

pub struct ZenohSubscriber {
    pub subscriber: Subscriber<FifoChannelHandler<Sample>>,
}

impl ZenohSubscriber {
    pub async fn new(
        session: Arc<Session>,
        key_expr: &'static str,
    ) -> Result<ZenohSubscriber, Box<dyn std::error::Error + Send + Sync>> {
        let subscriber = session.declare_subscriber(key_expr).await?;
        Ok(ZenohSubscriber { subscriber })
    }
}

pub struct SubscriberTaskSpawner;

impl SubscriberTaskSpawner {
    pub fn spawn_task<T>(
        session: Arc<Session>,
        key_expr: &'static str,
        sender: mpsc::Sender<T>,
    ) -> JoinHandle<()>
    where
        T: Message + Default + Send + Sync + 'static,
    {
        tokio::spawn(async move {
            match ZenohSubscriber::new(session, key_expr).await {
                Ok(subscriber) => {
                    while let Ok(sample) = subscriber.subscriber.recv_async().await {
                        let bytes = sample.payload().to_bytes();
                        match T::decode(&*bytes) {
                            Ok(message) => {
                                if let Err(err) = sender.send(message).await {
                                    error!("Failed to send message through channel: {:?}", err);
                                    break;
                                }
                            }
                            Err(e) => {
                                error!("Failed to decode message: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to create subscriber: {:?}", e)
                }
            };
        })
    }
}
