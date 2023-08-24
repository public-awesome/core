//! # sg-index-query
//!
//! `sg-index-query` is a Rust crate providing a `QueryOptions` struct for use in CosmWasm smart contracts.
//! It allows you to specify query parameters such as limit, order, and range.
//!
//! ## Features
//!
//! - `QueryOptions` struct for specifying query parameters such as limit, order, and range.
//! - `unpack` function for converting `QueryOptions` into `QueryOptionsInternal`, which is used for range queries in `cw_storage_plus`.
//!
//! ## Usage
//!
//! First, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sg-index-query = "0.1.0"
//! ```
//!
//! Then, you can use the `QueryOptions` struct in your code:
//!
//! ```rust
//! use sg_index_query::QueryOptions;
//!
//! let query_options = QueryOptions::<String>::default();
//! ```
//!
//! You can specify query parameters and unpack like so:
//!
//! ```rust
//! use sg_index_query::{QueryOptions, QueryBound, QueryOptionsInternal};
//!
//! let query_options = QueryOptions {
//!     descending: Some(true),
//!     limit: Some(20),
//!     min: Some(QueryBound::Inclusive("test".to_string())),
//!     max: Some(QueryBound::Exclusive("test2".to_string())),
//! };
//!
//! let query_options_internal = query_options.unpack(&|offset: &String| offset.to_string(), None, None);
//! ```

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Order;
use cw_storage_plus::{Bound, PrimaryKey};

const DEFAULT_QUERY_LIMIT: u32 = 10;
const MAX_QUERY_LIMIT: u32 = 100;

#[cw_serde]
pub enum QueryBound<T> {
    Inclusive(T),
    Exclusive(T),
}

/// QueryOptions are used to pass in options to a query function
#[cw_serde]
pub struct QueryOptions<T> {
    /// The number of items that will be returned
    pub limit: Option<u32>,
    /// Whether to sort items in ascending or descending order
    pub descending: Option<bool>,
    /// The minimum key value to fetch
    pub min: Option<QueryBound<T>>,
    /// The maximum key value to fetch
    pub max: Option<QueryBound<T>>,
}

impl<T> Default for QueryOptions<T> {
    fn default() -> Self {
        QueryOptions {
            descending: None,
            min: None,
            max: None,
            limit: None,
        }
    }
}

/// QueryOptionsInternal are derived from QueryOptions
/// using the `unpack` function.
pub struct QueryOptionsInternal<'a, U: PrimaryKey<'a>> {
    /// The number of items that will be returned
    pub limit: usize,
    /// The [cosmwasm_std::Order] used to sort items in [cw_storage_plus] range queries
    pub order: Order,
    /// The [cw_storage_plus::Bound] used to sort items in [cw_storage_plus] range queries
    pub min: Option<Bound<'a, U>>,
    /// The [cw_storage_plus::Bound] used to sort items in [cw_storage_plus] range queries
    pub max: Option<Bound<'a, U>>,
}

impl<T> QueryOptions<T> {
    pub fn unpack<'a, U: PrimaryKey<'a>>(
        &self,
        offset_to_bound_fn: &dyn Fn(&T) -> U,
        default_query_limit: Option<u32>,
        max_query_limit: Option<u32>,
    ) -> QueryOptionsInternal<'a, U> {
        let default_query_limit = default_query_limit.unwrap_or(DEFAULT_QUERY_LIMIT);
        let max_query_limit = max_query_limit.unwrap_or(MAX_QUERY_LIMIT);

        let limit = self
            .limit
            .unwrap_or(default_query_limit)
            .min(max_query_limit) as usize;

        let mut order = Order::Ascending;
        if let Some(_descending) = self.descending {
            if _descending {
                order = Order::Descending;
            }
        };

        let min = match &self.min {
            Some(QueryBound::Inclusive(offset)) => {
                Some(Bound::inclusive(offset_to_bound_fn(offset)))
            }
            Some(QueryBound::Exclusive(offset)) => {
                Some(Bound::exclusive(offset_to_bound_fn(offset)))
            }
            None => None,
        };

        let max = match &self.max {
            Some(QueryBound::Inclusive(offset)) => {
                Some(Bound::inclusive(offset_to_bound_fn(offset)))
            }
            Some(QueryBound::Exclusive(offset)) => {
                Some(Bound::exclusive(offset_to_bound_fn(offset)))
            }
            None => None,
        };

        QueryOptionsInternal {
            limit,
            order,
            min,
            max,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn try_unpack_default() {
        use super::*;
        let query_options = QueryOptions::<String>::default();

        let query_options_internal =
            query_options.unpack(&|offset: &String| offset.to_string(), None, None);

        assert_eq!(query_options_internal.limit as u32, DEFAULT_QUERY_LIMIT);

        match query_options_internal.order {
            Order::Ascending => {}
            Order::Descending => panic!("Expected Order::Ascending"),
        }
        assert!(query_options_internal.min.is_none());
        assert!(query_options_internal.max.is_none());
    }

    #[test]
    fn try_unpack_query_options() {
        use super::*;
        let query_options = QueryOptions {
            descending: Some(true),
            limit: Some(20),
            min: Some(QueryBound::Inclusive("test".to_string())),
            max: Some(QueryBound::Exclusive("test2".to_string())),
        };

        let query_options_internal =
            query_options.unpack(&|offset: &String| offset.to_string(), None, None);

        assert_eq!(query_options_internal.limit as u32, 20u32);
        match query_options_internal.order {
            Order::Ascending => panic!("Expected Order::Descending"),
            Order::Descending => {}
        }

        match query_options_internal.min {
            Some(Bound::Inclusive(offset)) => {
                assert_eq!(offset.0, "test".to_string())
            }
            _ => panic!("Expected Bound::Inclusive"),
        }

        match query_options_internal.max {
            Some(Bound::Exclusive(offset)) => {
                assert_eq!(offset.0, "test2".to_string())
            }
            _ => panic!("Expected Bound::Exclusive"),
        }
    }
}
