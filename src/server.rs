use futures::{StreamExt};
use futures_util::lock::Mutex;
use std::sync::Arc;
use std::hash::{Hasher, Hash};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;

use myproto::my_rpc_server::{MyRpc,MyRpcServer};
use myproto::{Key, KvPair, Value, ReplyState, RequestState};

impl Eq for Key {}
impl Hash for Key {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.key.hash(state);
    }
}

pub mod myproto {
    include!("pb/myproto.rs");
}

#[derive(Debug)]
pub struct MyRpcService {
    table: Arc<Mutex<HashMap<Key, Value>>>,
}

#[tonic::async_trait]
impl MyRpc for MyRpcService {
    async fn set_kv(
        &self,
        _request: Request<KvPair>,
    ) -> Result<Response<ReplyState>, Status> {
        println!("set_kv = {:?}", _request);

        let kvpair = _request.into_inner();
        let k = kvpair.key.expect("key should not none.");
        let v = kvpair.val.expect("value should not none.");
        let tb = self.table.clone();

        if let Some(val) = 
            tb.lock().await.insert(k.clone(), v) {
            return Ok(Response::new(ReplyState {
                reply_info: "update value, return old.".into(),
                kvpair: Some(KvPair {
                    key: Some(k),
                    val: Some(val.clone()),
                }),
            }));
        }

        Ok(Response::new(
            ReplyState{
                reply_info: "set new kvpair.".into(),
                kvpair: Some(KvPair {
                    key: Some(k),
                    val: None,
                }),
            }))
    }

    async fn get_kv(
        &self,
        _request: Request<Key>,
    ) -> Result<Response<ReplyState>, Status> {
        println!("get_kv = {:?}", _request);

        let k = _request.into_inner();
        let tb = self.table.clone();
        
        if let Some(val) = 
            tb.lock().await.get(&k) {
            return Ok(Response::new(ReplyState {
                reply_info: "get success.".into(),
                kvpair: Some(KvPair {
                    key: Some(k),
                    val: Some(val.clone()),
                }),
            }))
        }

        Ok(Response::new(ReplyState {
            reply_info: "get failed.".into(),
            kvpair: Some(KvPair {
                key: Some(k),
                val: None,
            }),
        }))
    }   

    ///Server streaming response type for the GetKvList method.
    type GetKvListStream = ReceiverStream<Result<KvPair, Status>>;

    /// A server-to-client streaming RPC.
    async fn get_kv_list(
        &self,
        _request: Request<RequestState>,
    ) -> Result<Response<Self::GetKvListStream>, Status> {
        println!("get_kv_list = {:?}", _request);
        let (tx, rx) = mpsc::channel(10);
        let tb = self.table.clone();

        tokio::spawn(async move {
            for (k, v) in tb.lock().await.iter() {
                println!("  => send k = {:?}, v = {:?}", k, v);
                tx.send(Ok(KvPair {
                    key: Some(k.clone()),
                    val: Some(v.clone()),
                })).await.unwrap();
            }
            println!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    /// A client-to-server streaming RPC.
    async fn set_kv_list(
        &self,
        _request: Request<tonic::Streaming<KvPair>>,
    ) -> Result<Response<ReplyState>, Status> {
        println!("set_kv_list = {:?}", _request);

        let tb = self.table.clone();
        let mut stream = _request.into_inner();

        while let Some(kvpair) = stream.next().await {
            let kvpair = kvpair?;
            let k = kvpair.key;
            let v = kvpair.val;
            tb.lock().await.insert(k.unwrap(), v.unwrap());
        }

        Ok(Response::new(ReplyState {
            reply_info: "set all kvpair done.".into(),
            kvpair: None,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "[::1]:10000".parse().unwrap();

    let my_rpc = MyRpcService {
        table: Arc::new(Mutex::new(HashMap::new())),
    };

    let svc = MyRpcServer::new(my_rpc);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
