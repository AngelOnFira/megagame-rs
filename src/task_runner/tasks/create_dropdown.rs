use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serenity::{
    builder::{CreateActionRow, CreateButton},
    client::Context,
    http::CacheHttp,
    model::{
        id::ChannelId,
        interactions::message_component::{ActionRow, SelectMenuOption},
    },
};

use super::{Task};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDropdown {
    guild_id: u64,
    channel_id: u64,
    custom_id: String,
    options: Vec<SelectMenuOption>,
    action_row: ActionRow,
}

#[async_trait]
impl Task for CreateDropdown {
    async fn handle(&self, ctx: Arc<Context>) {
        let _message = ChannelId(self.channel_id)
            .send_message(ctx.http(), |m| {
                m.content("Hello, world!");
                m.components(|c| {
                    c.add_action_row({
                        let mut ar = CreateActionRow::default();
                        ar.add_button({
                            let mut b = CreateButton::default();
                            b.label("test1");
                            b
                        });
                        ar
                    })
                })
            })
            .await
            .unwrap();
    }
}
