// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::any::Any;
use std::sync::Arc;

use common_aggregate_functions::AggregateFunctionFactory;
use common_datablocks::DataBlock;
use common_datavalues::BooleanArray;
use common_datavalues::DataField;
use common_datavalues::DataSchemaRef;
use common_datavalues::DataSchemaRefExt;
use common_datavalues::DataType;
use common_datavalues::StringArray;
use common_exception::Result;
use common_functions::FunctionFactory;
use common_planners::Partition;
use common_planners::ReadDataSourcePlan;
use common_planners::ScanPlan;
use common_planners::Statistics;
use common_streams::DataBlockStream;
use common_streams::SendableDataBlockStream;

use crate::datasources::Table;
use crate::sessions::FuseQueryContextRef;

pub struct FunctionsTable {
    schema: DataSchemaRef,
}

impl FunctionsTable {
    pub fn create() -> Self {
        FunctionsTable {
            schema: DataSchemaRefExt::create(vec![
                DataField::new("name", DataType::Utf8, false),
                DataField::new("is_aggregate", DataType::Boolean, false),
            ]),
        }
    }
}

#[async_trait::async_trait]
impl Table for FunctionsTable {
    fn name(&self) -> &str {
        "functions"
    }

    fn engine(&self) -> &str {
        "SystemFunctions"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> Result<DataSchemaRef> {
        Ok(self.schema.clone())
    }

    fn is_local(&self) -> bool {
        true
    }

    fn read_plan(
        &self,
        _ctx: FuseQueryContextRef,
        scan: &ScanPlan,
        _partitions: usize,
    ) -> Result<ReadDataSourcePlan> {
        Ok(ReadDataSourcePlan {
            db: "system".to_string(),
            table: self.name().to_string(),
            schema: self.schema.clone(),
            partitions: vec![Partition {
                name: "".to_string(),
                version: 0,
            }],
            statistics: Statistics::default(),
            description: "(Read from system.functions table)".to_string(),
            scan_plan: Arc::new(scan.clone()),
            remote: false,
        })
    }

    async fn read(&self, _ctx: FuseQueryContextRef) -> Result<SendableDataBlockStream> {
        let func_names = FunctionFactory::registered_names();
        let aggr_func_names = AggregateFunctionFactory::registered_names();

        let names: Vec<&str> = func_names
            .iter()
            .chain(aggr_func_names.iter())
            .map(|x| x.as_ref())
            .collect();

        let is_aggregate = (0..names.len())
            .map(|i| i >= func_names.len())
            .collect::<Vec<bool>>();

        let block = DataBlock::create_by_array(self.schema.clone(), vec![
            Arc::new(StringArray::from(names)),
            Arc::new(BooleanArray::from(is_aggregate)),
        ]);

        Ok(Box::pin(DataBlockStream::create(
            self.schema.clone(),
            None,
            vec![block],
        )))
    }
}
