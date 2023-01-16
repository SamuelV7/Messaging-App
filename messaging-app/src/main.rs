use std::{env, sync::Arc, sync::RwLock};
use dotenv;
use uuid::Uuid;
use redis::{Connection, Commands, RedisError, PubSubCommands};
use serde::{self, Serialize, Deserialize};
use serde_json;
use tokio::{task};

struct RedisDB{
    connection : Arc<RwLock<Connection>>,
}
impl RedisDB {
    fn new() -> Self{
        dotenv::dotenv().ok();
        let redis_url = env::var("REDIS_URL").expect("Error finding url");
        let client = redis::Client::open(redis_url).expect("Error with Client");
        let con = client.get_connection().expect("Connection error");
        RedisDB { connection: Arc::new(RwLock::new(con))}
    }

    fn post_in_channel(self, message : Message)-> Result<(), RedisError>{
        let msg_string = serde_json::to_string(&message).expect("Error with json conversion");
        let mut guard = self.connection.write().unwrap();
        guard.publish(message.channel, msg_string)
        // self.connection.publish(message.channel, msg_string)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id : String,
    channel : String,
    payload : String
}

impl Message {
    fn new(payload : String, channel : String) -> Self{
        let uid = Uuid::new_v4().to_string();
        Message{id: uid, channel, payload}
    }
}

#[tokio::main]
async fn main() {
    let callback = |msg: redis::Msg| {
        println!("Received message: {:?}", msg);
        redis::ControlFlow::Continue::<()>
    };
    let rdb = Arc::new(RedisDB::new());
    let db = rdb.clone();
    let _tsk = task::spawn_blocking(move || {
        let r = Arc::clone(&db);
        let _res = r.connection.write().unwrap().subscribe("Channel", callback);
        // r.publish(channel, message);
    });
}
