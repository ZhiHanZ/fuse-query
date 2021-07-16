// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use common_datavalues::DataSchemaRef;
use common_datavalues::DataSchemaRefExt;
use common_exception::Result;
use common_streams::SendableDataBlockStream;

#[async_trait::async_trait]
pub trait Interpreter: Sync + Send {
    fn name(&self) -> &str;
    async fn execute(&self) -> Result<SendableDataBlockStream>;

    fn schema(&self) -> DataSchemaRef {
        DataSchemaRefExt::create(vec![])
    }
}

pub type InterpreterPtr = std::sync::Arc<dyn Interpreter>;
