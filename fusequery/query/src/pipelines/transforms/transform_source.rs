// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::any::Any;
use std::sync::Arc;

use common_exception::ErrorCode;
use common_exception::Result;
use common_streams::SendableDataBlockStream;
use common_tracing::tracing;

use crate::pipelines::processors::EmptyProcessor;
use crate::pipelines::processors::Processor;
use crate::sessions::FuseQueryContextRef;

pub struct SourceTransform {
    ctx: FuseQueryContextRef,
    db: String,
    table: String,
    remote: bool,
}

impl SourceTransform {
    pub fn try_create(
        ctx: FuseQueryContextRef,
        db: &str,
        table: &str,
        remote: bool,
    ) -> Result<Self> {
        Ok(SourceTransform {
            ctx,
            db: db.to_string(),
            table: table.to_string(),
            remote,
        })
    }
}

#[async_trait::async_trait]
impl Processor for SourceTransform {
    fn name(&self) -> &str {
        "SourceTransform"
    }

    fn connect_to(&mut self, _: Arc<dyn Processor>) -> Result<()> {
        Result::Err(ErrorCode::LogicalError(
            "Cannot call SourceTransform connect_to",
        ))
    }

    fn inputs(&self) -> Vec<Arc<dyn Processor>> {
        vec![Arc::new(EmptyProcessor::create())]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn execute(&self) -> Result<SendableDataBlockStream> {
        tracing::debug!(
            "execute, table:{:#}.{:#}, is_remote:{:#}...",
            self.db,
            self.table,
            self.remote
        );

        let table = if self.remote {
            self.ctx
                .get_remote_table(self.db.as_str(), self.table.as_str())
                .await?
        } else {
            self.ctx.get_table(self.db.as_str(), self.table.as_str())?
        };

        table.read(self.ctx.clone()).await
    }
}
