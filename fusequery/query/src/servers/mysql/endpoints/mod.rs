// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

mod endpoint;
mod endpoint_on_query;

pub use self::endpoint::MySQLEndpoint;
pub use self::endpoint_on_query::done as on_query_done;
