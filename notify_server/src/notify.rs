use std::{collections::HashSet, sync::Arc};

use core_lib::{Chat, Message};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgListener;
use tracing::info;

use crate::AppState;
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AppEvent {
    NewChat(Chat),
    NewMessage(Message),
    AddToChat(Chat),
    RemoveFromChat(Chat),
}
#[derive(Debug)]
struct Notification {
    //user being impact
    user_ids: HashSet<u64>,
    event: Arc<AppEvent>,
}

//通过serde json需要转换的收到数据的tragger的事件数据结构
/*'chat_updated',
json_build_object(
    'op',
    TG_OP,
    'old',
    OLD,
    'new',
    NEW
)*/
#[derive(Debug, Serialize, Deserialize)]
struct ChatUpdated {
    op: String,
    old: Option<Chat>,
    new: Option<Chat>,
}
//'chat_message_created',row_to_json(NEW)
#[derive(Debug, Serialize, Deserialize)]
struct ChatMessageCreated {
    members: Vec<i64>,

    message: Message,
}

pub async fn setup_pg_listener(state: AppState) -> anyhow::Result<()> {
    let db_url = &state.config.server.db_url;
    let mut listener = PgListener::connect(db_url.as_str()).await?;
    listener.listen("chat_updated").await?;
    listener.listen("chat_message_created").await?;
    let mut stream = listener.into_stream();
    tokio::spawn(async move {
        while let Some(Ok(notification)) = stream.next().await {
            let notification = Notification::load(notification.channel(), notification.payload())?;
            info!("notification: {:?}", notification);

            let users = &state.users;
            info!("users: {:?}", users);
            for user_id in notification.user_ids {
                if let Some(tx) = users.get(&user_id) {
                    if let Err(e) = tx.send(notification.event.clone()) {
                        info!("send event failed: {:?}", e);
                    };
                }
            }
            info!("send event success");
        }
        Ok::<(), anyhow::Error>(())
    });

    Ok(())
}
impl Notification {
    fn load(r#type: &str, playload: &str) -> anyhow::Result<Self> {
        match r#type {
            "chat_updated" => {
                let payload: ChatUpdated = serde_json::from_str(playload)?;
                let user_ids = get_affected_user_ids(payload.old.as_ref(), payload.new.as_ref());
                let event = match payload.op.as_str() {
                    "INSERT" => AppEvent::NewChat(payload.new.expect("new should exist")),
                    "UPDATE" => AppEvent::AddToChat(payload.new.expect("new should exist")),
                    "DELETE" => AppEvent::RemoveFromChat(payload.old.expect("old should exist")),
                    _ => return Err(anyhow::anyhow!("Invalid op")),
                };
                Ok(Self {
                    user_ids,
                    event: Arc::new(event),
                })
            }
            "chat_message_created" => {
                let payload: ChatMessageCreated = serde_json::from_str(playload)?;
                let user_ids = payload.members.iter().map(|v| *v as u64).collect();
                Ok(Self {
                    user_ids,
                    event: Arc::new(AppEvent::NewMessage(payload.message)),
                })
            }
            _ => Err(anyhow::anyhow!("Invalid type")),
        }
    }
}

fn get_affected_user_ids(old: Option<&Chat>, new: Option<&Chat>) -> HashSet<u64> {
    let user_ids = HashSet::new();
    match (old, new) {
        (Some(old), Some(new)) => {
            let old_user_ids: HashSet<_> = old.members.iter().map(|v| *v as u64).collect();
            let new_user_ids: HashSet<_> = new.members.iter().map(|v| *v as u64).collect();
            if old_user_ids == new_user_ids {
                user_ids
            } else {
                old_user_ids.union(&new_user_ids).copied().collect()
            }
        }
        (Some(old), None) => old.members.iter().map(|v| *v as u64).collect(),
        (None, Some(new)) => new.members.iter().map(|v| *v as u64).collect(),
        (None, None) => HashSet::new(),
    }
}
