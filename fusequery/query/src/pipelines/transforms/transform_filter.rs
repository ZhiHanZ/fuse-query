// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::any::Any;
use std::convert::TryInto;
use std::sync::Arc;
use std::time::Instant;

use common_arrow::arrow;
use common_datablocks::DataBlock;
use common_datavalues as datavalues;
use common_datavalues::BooleanArray;
use common_datavalues::DataSchemaRef;
use common_datavalues::DataSchemaRefExt;
use common_exception::ErrorCode;
use common_exception::Result;
use common_planners::Expression;
use common_streams::SendableDataBlockStream;
use common_tracing::tracing;
use tokio_stream::StreamExt;

use crate::pipelines::processors::EmptyProcessor;
use crate::pipelines::processors::Processor;
use crate::pipelines::transforms::ExpressionExecutor;

pub struct FilterTransform {
    input: Arc<dyn Processor>,
    executor: Arc<ExpressionExecutor>,
    predicate: Expression,
    having: bool,
}

impl FilterTransform {
    pub fn try_create(schema: DataSchemaRef, predicate: Expression, having: bool) -> Result<Self> {
        let mut fields = schema.fields().clone();
        fields.push(predicate.to_data_field(&schema)?);

        let executor = ExpressionExecutor::try_create(
            "filter executor",
            schema,
            DataSchemaRefExt::create(fields),
            vec![predicate.clone()],
            false,
        )?;
        executor.validate()?;

        Ok(FilterTransform {
            input: Arc::new(EmptyProcessor::create()),
            executor: Arc::new(executor),
            predicate,
            having,
        })
    }
}

#[async_trait::async_trait]
impl Processor for FilterTransform {
    fn name(&self) -> &str {
        if self.having {
            return "HavingTransform";
        }
        "FilterTransform"
    }

    fn connect_to(&mut self, input: Arc<dyn Processor>) -> Result<()> {
        self.input = input;
        Ok(())
    }

    fn inputs(&self) -> Vec<Arc<dyn Processor>> {
        vec![self.input.clone()]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn execute(&self) -> Result<SendableDataBlockStream> {
        let input_stream = self.input.execute().await?;
        let executor = self.executor.clone();
        let column_name = self.predicate.column_name();

        let execute_fn = |executor: Arc<ExpressionExecutor>,
                          column_name: &str,
                          block: Result<DataBlock>|
         -> Result<DataBlock> {
            tracing::debug!("execute...");
            let start = Instant::now();

            let block = block?;
            let filter_block = executor.execute(&block)?;
            let filter_array = filter_block.try_column_by_name(column_name)?.to_array()?;
            // Downcast to boolean array
            let filter_array = datavalues::downcast_array!(filter_array, BooleanArray)?;

            // Convert to arrow record_batch
            let batch = block.try_into()?;
            let batch = arrow::compute::filter_record_batch(&batch, filter_array)?;

            let delta = start.elapsed();
            tracing::debug!("Filter cost: {:?}", delta);
            batch.try_into()
        };

        let stream = input_stream.filter_map(move |v| {
            execute_fn(executor.clone(), &column_name, v)
                .map(Some)
                .transpose()
        });
        Ok(Box::pin(stream))
    }
}
