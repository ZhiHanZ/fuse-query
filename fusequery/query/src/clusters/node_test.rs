// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use common_exception::Result;

use crate::clusters::address::Address;
use crate::clusters::Node;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_serialize_node() -> Result<()> {
    let node = Node::create(
        String::from("name"),
        1,
        Address::create(&String::from("localhost:9090"))?,
        true,
        2,
    )?;

    let node_json = "{\"name\":\"name\",\"priority\":1,\"address\":\"localhost:9090\",\"local\":true,\"sequence\":2}";

    assert_eq!(serde_json::to_string(&node)?, node_json.clone());
    assert_eq!(serde_json::from_str::<Node>(node_json.clone())?, node);

    Ok(())
}
