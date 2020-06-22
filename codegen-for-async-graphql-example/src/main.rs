// use codegen_for_async_graphql_derive::*;

mod models;

use async_graphql::*;
use async_std::task;
use models::root::Root;

#[derive(Debug, Clone, Copy)]
pub struct DataSource {}

impl DataSource {
    fn active(&self) -> bool {
        true
    }

    fn me(self) -> models::me::Me {
        models::me::Me {
            id: ID::from("11111"),
            name: "aaa".to_string(),
            email: "aaa".to_string(),
            age: 30,
            rank: 5.1,
        }
    }
}

fn main() {
    task::block_on(async { run().await });
}

// #[DynSchema("./codegen-for-async-graphql-example/schema.graphql")]
async fn run() {
    let data_source = DataSource {};
    let schema = Schema::build(Root { active: true }, EmptyMutation, EmptySubscription)
        .data(data_source)
        .finish();
    let res = schema
        .execute("{ active me { id name email age rank } }")
        .await;
    let json = serde_json::to_string(&async_graphql::http::GQLResponse(res));
    println!("{:?}", json);
}

#[test]
fn instance_query() {
    task::block_on(async { run().await });
}
