# sg-index-query

`sg-index-query` is a Rust crate providing a `QueryOptions` struct for use in CosmWasm smart contracts.
It allows you to specify query parameters such as limit, order, and the min max of range queries.

## Features

- `QueryOptions` struct for specifying query parameters such as limit, order, and range.
- `unpack` function for converting `QueryOptions` into `QueryOptionsInternal`, which is used for range queries in `cw_storage_plus`.

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
sg-index-query = "0.1.0"
```

Then, you can use the `QueryOptions` struct in your code:

```rust
use sg_index_query::QueryOptions;

let query_options = QueryOptions::<String>::default();
```

You can specify query parameters like so:

```rust
use sg_index_query::{QueryOptions, QueryBound};

let query_options = QueryOptions {
    descending: Some(true),
    limit: Some(20),
    min: Some(QueryBound::Inclusive("test".to_string())),
    max: Some(QueryBound::Exclusive("test2".to_string())),
};
```

Then, you can unpack the `QueryOptions` into `QueryOptionsInternal`:

```rust
let query_options_internal = query_options.unpack(&|offset: &String| offset.to_string(), None, None);
```
