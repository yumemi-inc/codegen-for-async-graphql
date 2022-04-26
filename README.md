# codegen-for-async-graphql

[![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/gobang/CI/main)](https://github.com/yumemi-inc/codegen-for-async-graphql/actions)

## Usage

```bash
cargo codegen-for-async-graphql --schema {path_to_schema} --output {path_to_output}
# cargo codegen-for-async-graphql --schema ./schema.graphql --output src/models
```

```rust
mod models;

use async_graphql::*;

use models::{
  Mutation, Query,
};

let data_source = DataSource {};
let schema = Schema::build(Query {}, Mutation {}, EmptySubscription)
    .register_type::<User>()
    .data(data_source)
    .finish();
let res = schema.execute(query).await;
let json = serde_json::to_string_pretty(&async_graphql::http::GQLResponse(res));
json.unwrap()
```
