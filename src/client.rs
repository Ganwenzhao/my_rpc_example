use tonic::Request;
use tonic::transport::Channel;
use futures::stream;
use std::error::Error;

use myproto::my_rpc_client::MyRpcClient;
use myproto::{Key, KvPair, Value, RequestState};

pub mod myproto {
    include!("pb/myproto.rs");
}

async fn print_kv_list(client: &mut MyRpcClient<Channel>) -> Result<(), Box<dyn Error>> {
    let rqs = RequestState {
        request_info: "get all.".into(),
    };

    let mut stream = client
        .get_kv_list(Request::new(rqs))
        .await?
        .into_inner();

    while let Some(kvpair) = stream.message().await? {
        println!("KvPair = {:?}", kvpair);
    }

    Ok(())
}

async fn run_set_kv_list(client: &mut MyRpcClient<Channel>) -> Result<(), Box<dyn Error>> {
    let mut pairs = vec![];

    for i in 0..3 {
        pairs.push(KvPair {
            key: Some(Key{key: i.to_string()}),
            val: Some(Value{val: i.to_string()})
        })
    }

    println!("pairs num = {:?}", pairs.len());
    let request = Request::new(stream::iter(pairs));

    match client.set_kv_list(request).await {
        Ok(response) => println!("ReplyState = {:?}", response.into_inner()),
        Err(e) => println!("something went wrong: {:?}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MyRpcClient::connect("http://[::1]:10000").await?;
    
    println!("*** SIMPLE RPC ***");
    let response0 = client
    .set_kv(Request::new(KvPair {
        key: Some(Key{key: "a".into()}),
        val: Some(Value{val: "1". into()}),
    }))
    .await?;
    println!("RESPONSE0 = {:?}", response0);

    let response1 = client
    .get_kv(Request::new(Key{key: "a".into()}))
    .await?;
    println!("RESPONSE1 = {:?}", response1);

    println!("\n*** CLIENT STREAMING ***");
    run_set_kv_list(&mut client).await?;

    println!("\n*** SERVER STREAMING ***");
    print_kv_list(&mut client).await?;

    Ok(())
}