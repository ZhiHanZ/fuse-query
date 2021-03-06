// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use common_exception::Result;

use crate::aggregate_function_factory::FactoryCombinatorFuncRef;
use crate::aggregate_function_factory::FactoryFuncRef;
use crate::AggregateArgMaxFunction;
use crate::AggregateArgMinFunction;
use crate::AggregateAvgFunction;
use crate::AggregateCountFunction;
use crate::AggregateDistinctCombinator;
use crate::AggregateIfCombinator;
use crate::AggregateMaxFunction;
use crate::AggregateMinFunction;
use crate::AggregateSumFunction;

pub struct AggregatorFunction;

impl AggregatorFunction {
    pub fn register(map: FactoryFuncRef) -> Result<()> {
        let mut map = map.write();
        // FuseQuery always uses lowercase function names to get functions.
        map.insert("count", AggregateCountFunction::try_create);
        map.insert("min", AggregateMinFunction::try_create);
        map.insert("max", AggregateMaxFunction::try_create);
        map.insert("sum", AggregateSumFunction::try_create);
        map.insert("avg", AggregateAvgFunction::try_create);
        map.insert("argmin", AggregateArgMinFunction::try_create);
        map.insert("argmax", AggregateArgMaxFunction::try_create);

        map.insert("uniq", AggregateDistinctCombinator::try_create_uniq);

        Ok(())
    }

    pub fn register_combinator(map: FactoryCombinatorFuncRef) -> Result<()> {
        let mut map = map.write();
        map.insert("distinct", AggregateDistinctCombinator::try_create);
        map.insert("if", AggregateIfCombinator::try_create);

        Ok(())
    }
}
