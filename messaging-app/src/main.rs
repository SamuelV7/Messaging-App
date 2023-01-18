use std::{env, sync::Arc, sync::RwLock, net::SocketAddr};
use axum_macros::debug_handler;
use dotenv;
use uuid::Uuid;
use redis::{Connection, Commands, RedisError, PubSubCommands, aio::MultiplexedConnection, AsyncCommands};
use serde::{self, Serialize, Deserialize};
use serde_json;
use tokio::{task};
use futures::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
use axum::{
    extract::{ws::{Message, WebSocketUpgrade, Websocket}},
    routing::{get, post},
    http::StatusCode,
    Extension,
    response::{IntoResponse, Response},
    Json, Router};


struct RedisDB{
    connection : MultiplexedConnection,
}

impl RedisDB {
    async fn new() -> Self{
        dotenv::dotenv().ok();
        let redis_url = env::var("REDIS_URL").expect("Error finding url");
        let client = redis::Client::open(redis_url).expect("Error with Client");

        // let con = client.get_connection().expect("Connection error");
        let con_multiplex = client.get_multiplexed_async_connection().await.expect("Error with connection");
        RedisDB { connection: con_multiplex}
    }
     
    async fn post_in_channel(mut self, message : Message){
        let msg_string = serde_json::to_string(&message).expect("Error with json conversion");
        let guard : Result<(), RedisError>= self.connection.publish(message.channel, msg_string).await;
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
    // Need to add to stream
    // Read stream and other Stream Management
}

async fn root() -> &'static str {
    "Soli Deo Gloria"
}

fn ws_handler(mut socket : Websocket){
    let (mut sender, mut receiver) = socket.split();
}


async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse{
    ws.on_upgrade(move |socket| ws_handler)
}

async fn read(receiver: SplitStream<Websocket>){
    while let Some(msg) = receiver.next().await {
        let msg = msg.unwrap();
        let msg = msg.to_str().unwrap();
        println!("Message: {}", msg);
    }
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    // initialising tracing
    let app = Router::new()
        .route("/ws", get(handler))
        .route("/", get(root));

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

}

// let callback = |msg: redis::Msg| {
//     println!("Received message: {:?}", msg);
//     redis::ControlFlow::Continue::<()>
// };

// let rdb = RedisDB::new().await;
// let _tsk = task::spawn_blocking(move || {
//     let db = rdb.connection.clone();
    
// });