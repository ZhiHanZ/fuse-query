// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::sync::Arc;

use common_arrow::arrow::array::DurationMicrosecondArray;
use common_arrow::arrow::array::DurationMillisecondArray;
use common_arrow::arrow::array::DurationNanosecondArray;
use common_arrow::arrow::array::DurationSecondArray;
use common_arrow::arrow::array::IntervalDayTimeArray;
use common_arrow::arrow::array::IntervalYearMonthArray;
use common_arrow::arrow::array::LargeBinaryArray;
use common_arrow::arrow::array::LargeStringArray;
use common_arrow::arrow::array::Time32MillisecondArray;
use common_arrow::arrow::array::Time32SecondArray;
use common_arrow::arrow::array::Time64MicrosecondArray;
use common_arrow::arrow::array::Time64NanosecondArray;
use common_arrow::arrow::array::TimestampMicrosecondArray;
use common_arrow::arrow::array::TimestampMillisecondArray;
use common_arrow::arrow::array::TimestampNanosecondArray;
use common_arrow::arrow::array::TimestampSecondArray;
use common_exception::Result;

use crate::BinaryArray;
use crate::DataArrayRef;
use crate::DataArrayScatter;
use crate::DataColumnarValue;
use crate::DataValue;
use crate::Date32Array;
use crate::Date64Array;
use crate::Float32Array;
use crate::Float64Array;
use crate::Int16Array;
use crate::Int32Array;
use crate::Int64Array;
use crate::Int8Array;
use crate::StringArray;
use crate::UInt16Array;
use crate::UInt32Array;
use crate::UInt64Array;
use crate::UInt8Array;

#[test]
fn test_scatter_array_data_column() -> Result<()> {
    #[allow(dead_code)]
    struct ArrayTest {
        name: &'static str,
        array: DataArrayRef,
        indices: DataArrayRef,
        expect: Vec<DataArrayRef>,
        error: &'static str,
    }

    let tests = vec![
        ArrayTest {
            name: "non-null-Int8",
            array: Arc::new(Int8Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Int8Array::from(vec![1, 3])),
                Arc::new(Int8Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Int16",
            array: Arc::new(Int16Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Int16Array::from(vec![1, 3])),
                Arc::new(Int16Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Int32",
            array: Arc::new(Int32Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Int32Array::from(vec![1, 3])),
                Arc::new(Int32Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Int64",
            array: Arc::new(Int64Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Int64Array::from(vec![1, 3])),
                Arc::new(Int64Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-UInt8",
            array: Arc::new(UInt8Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(UInt8Array::from(vec![1, 3])),
                Arc::new(UInt8Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-UInt16",
            array: Arc::new(UInt16Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(UInt16Array::from(vec![1, 3])),
                Arc::new(UInt16Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-UInt32",
            array: Arc::new(UInt32Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(UInt32Array::from(vec![1, 3])),
                Arc::new(UInt32Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-UInt64",
            array: Arc::new(UInt64Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(UInt64Array::from(vec![1, 3])),
                Arc::new(UInt64Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Float32",
            array: Arc::new(Float32Array::from(vec![1., 2., 3.])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Float32Array::from(vec![1., 3.])),
                Arc::new(Float32Array::from(vec![2.])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Float64",
            array: Arc::new(Float64Array::from(vec![1., 2., 3.])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Float64Array::from(vec![1., 3.])),
                Arc::new(Float64Array::from(vec![2.])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Date32",
            array: Arc::new(Date32Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Date32Array::from(vec![1, 3])),
                Arc::new(Date32Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Date64",
            array: Arc::new(Date64Array::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Date64Array::from(vec![1, 3])),
                Arc::new(Date64Array::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-TimestampSecond",
            array: Arc::new(TimestampSecondArray::from_vec(vec![1, 2, 3], None)),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(TimestampSecondArray::from_vec(vec![1, 3], None)),
                Arc::new(TimestampSecondArray::from_vec(vec![2], None)),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-TimestampMillisecond",
            array: Arc::new(TimestampMillisecondArray::from_vec(vec![1, 2, 3], None)),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(TimestampMillisecondArray::from_vec(vec![1, 3], None)),
                Arc::new(TimestampMillisecondArray::from_vec(vec![2], None)),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-TimestampMicrosecond",
            array: Arc::new(TimestampMicrosecondArray::from_vec(vec![1, 2, 3], None)),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(TimestampMicrosecondArray::from_vec(vec![1, 3], None)),
                Arc::new(TimestampMicrosecondArray::from_vec(vec![2], None)),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-TimestampNanosecond",
            array: Arc::new(TimestampNanosecondArray::from_vec(vec![1, 2, 3], None)),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(TimestampNanosecondArray::from_vec(vec![1, 3], None)),
                Arc::new(TimestampNanosecondArray::from_vec(vec![2], None)),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Time32Second",
            array: Arc::new(Time32SecondArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Time32SecondArray::from(vec![1, 3])),
                Arc::new(Time32SecondArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Time32Millisecond",
            array: Arc::new(Time32MillisecondArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Time32MillisecondArray::from(vec![1, 3])),
                Arc::new(Time32MillisecondArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Time64Microsecond",
            array: Arc::new(Time64MicrosecondArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Time64MicrosecondArray::from(vec![1, 3])),
                Arc::new(Time64MicrosecondArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Time64Nanosecond",
            array: Arc::new(Time64NanosecondArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(Time64NanosecondArray::from(vec![1, 3])),
                Arc::new(Time64NanosecondArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-IntervalYearMonth",
            array: Arc::new(IntervalYearMonthArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(IntervalYearMonthArray::from(vec![1, 3])),
                Arc::new(IntervalYearMonthArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-IntervalDayTime",
            array: Arc::new(IntervalDayTimeArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(IntervalDayTimeArray::from(vec![1, 3])),
                Arc::new(IntervalDayTimeArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-DurationSecond",
            array: Arc::new(DurationSecondArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(DurationSecondArray::from(vec![1, 3])),
                Arc::new(DurationSecondArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-DurationMillisecond",
            array: Arc::new(DurationMillisecondArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(DurationMillisecondArray::from(vec![1, 3])),
                Arc::new(DurationMillisecondArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-DurationMicrosecond",
            array: Arc::new(DurationMicrosecondArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(DurationMicrosecondArray::from(vec![1, 3])),
                Arc::new(DurationMicrosecondArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-DurationNanosecond",
            array: Arc::new(DurationNanosecondArray::from(vec![1, 2, 3])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(DurationNanosecondArray::from(vec![1, 3])),
                Arc::new(DurationNanosecondArray::from(vec![2])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-Binary",
            array: Arc::new(BinaryArray::from(vec![
                &vec![1_u8][..],
                &vec![2_u8][..],
                &vec![3_u8][..],
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(BinaryArray::from(vec![&vec![1_u8][..], &vec![3_u8][..]])),
                Arc::new(BinaryArray::from(vec![&vec![2_u8][..]])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-LargeBinary",
            array: Arc::new(LargeBinaryArray::from(vec![
                &vec![1_u8][..],
                &vec![2_u8][..],
                &vec![3_u8][..],
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(LargeBinaryArray::from(vec![
                    &vec![1_u8][..],
                    &vec![3_u8][..],
                ])),
                Arc::new(LargeBinaryArray::from(vec![&vec![2_u8][..]])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-String",
            array: Arc::new(StringArray::from(vec!["1", "2", "3"])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(StringArray::from(vec!["1", "3"])),
                Arc::new(StringArray::from(vec!["2"])),
            ],
            error: "",
        },
        ArrayTest {
            name: "non-null-LargeString",
            array: Arc::new(LargeStringArray::from(vec!["1", "2", "3"])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 0])),
            expect: vec![
                Arc::new(LargeStringArray::from(vec!["1", "3"])),
                Arc::new(LargeStringArray::from(vec!["2"])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Int8",
            array: Arc::new(Int8Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Int8Array::from(vec![None, Some(2), None])),
                Arc::new(Int8Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Int16",
            array: Arc::new(Int16Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Int16Array::from(vec![None, Some(2), None])),
                Arc::new(Int16Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Int32",
            array: Arc::new(Int32Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Int32Array::from(vec![None, Some(2), None])),
                Arc::new(Int32Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Int64",
            array: Arc::new(Int64Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Int64Array::from(vec![None, Some(2), None])),
                Arc::new(Int64Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-UInt8",
            array: Arc::new(UInt8Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(UInt8Array::from(vec![None, Some(2), None])),
                Arc::new(UInt8Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-UInt16",
            array: Arc::new(UInt16Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(UInt16Array::from(vec![None, Some(2), None])),
                Arc::new(UInt16Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-UInt32",
            array: Arc::new(UInt32Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(UInt32Array::from(vec![None, Some(2), None])),
                Arc::new(UInt32Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-UInt64",
            array: Arc::new(UInt64Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(UInt64Array::from(vec![None, Some(2), None])),
                Arc::new(UInt64Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Float32",
            array: Arc::new(Float32Array::from(vec![
                None,
                Some(1.),
                None,
                Some(2.),
                None,
                Some(3.),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Float32Array::from(vec![None, Some(2.), None])),
                Arc::new(Float32Array::from(vec![Some(1.), None, Some(3.)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Float64",
            array: Arc::new(Float64Array::from(vec![
                None,
                Some(1.),
                None,
                Some(2.),
                None,
                Some(3.),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Float64Array::from(vec![None, Some(2.), None])),
                Arc::new(Float64Array::from(vec![Some(1.), None, Some(3.)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Date32",
            array: Arc::new(Date32Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Date32Array::from(vec![None, Some(2), None])),
                Arc::new(Date32Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Date64",
            array: Arc::new(Date64Array::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Date64Array::from(vec![None, Some(2), None])),
                Arc::new(Date64Array::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-TimestampSecond",
            array: Arc::new(TimestampSecondArray::from_opt_vec(
                vec![None, Some(1), None, Some(2), None, Some(3)],
                None,
            )),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(TimestampSecondArray::from_opt_vec(
                    vec![None, Some(2), None],
                    None,
                )),
                Arc::new(TimestampSecondArray::from_opt_vec(
                    vec![Some(1), None, Some(3)],
                    None,
                )),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-TimestampMillisecond",
            array: Arc::new(TimestampMillisecondArray::from_opt_vec(
                vec![None, Some(1), None, Some(2), None, Some(3)],
                None,
            )),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(TimestampMillisecondArray::from_opt_vec(
                    vec![None, Some(2), None],
                    None,
                )),
                Arc::new(TimestampMillisecondArray::from_opt_vec(
                    vec![Some(1), None, Some(3)],
                    None,
                )),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-TimestampMicrosecond",
            array: Arc::new(TimestampMicrosecondArray::from_opt_vec(
                vec![None, Some(1), None, Some(2), None, Some(3)],
                None,
            )),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(TimestampMicrosecondArray::from_opt_vec(
                    vec![None, Some(2), None],
                    None,
                )),
                Arc::new(TimestampMicrosecondArray::from_opt_vec(
                    vec![Some(1), None, Some(3)],
                    None,
                )),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-TimestampNanosecond",
            array: Arc::new(TimestampNanosecondArray::from_opt_vec(
                vec![None, Some(1), None, Some(2), None, Some(3)],
                None,
            )),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(TimestampNanosecondArray::from_opt_vec(
                    vec![None, Some(2), None],
                    None,
                )),
                Arc::new(TimestampNanosecondArray::from_opt_vec(
                    vec![Some(1), None, Some(3)],
                    None,
                )),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Time32Second",
            array: Arc::new(Time32SecondArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Time32SecondArray::from(vec![None, Some(2), None])),
                Arc::new(Time32SecondArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Time32Millisecond",
            array: Arc::new(Time32MillisecondArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Time32MillisecondArray::from(vec![None, Some(2), None])),
                Arc::new(Time32MillisecondArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Time64Microsecond",
            array: Arc::new(Time64MicrosecondArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Time64MicrosecondArray::from(vec![None, Some(2), None])),
                Arc::new(Time64MicrosecondArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Time64Nanosecond",
            array: Arc::new(Time64NanosecondArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(Time64NanosecondArray::from(vec![None, Some(2), None])),
                Arc::new(Time64NanosecondArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-IntervalYearMonth",
            array: Arc::new(IntervalYearMonthArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(IntervalYearMonthArray::from(vec![None, Some(2), None])),
                Arc::new(IntervalYearMonthArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-IntervalDayTime",
            array: Arc::new(IntervalDayTimeArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(IntervalDayTimeArray::from(vec![None, Some(2), None])),
                Arc::new(IntervalDayTimeArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-DurationSecond",
            array: Arc::new(DurationSecondArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(DurationSecondArray::from(vec![None, Some(2), None])),
                Arc::new(DurationSecondArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-DurationMillisecond",
            array: Arc::new(DurationMillisecondArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(DurationMillisecondArray::from(vec![None, Some(2), None])),
                Arc::new(DurationMillisecondArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-DurationMicrosecond",
            array: Arc::new(DurationMicrosecondArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(DurationMicrosecondArray::from(vec![None, Some(2), None])),
                Arc::new(DurationMicrosecondArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-DurationNanosecond",
            array: Arc::new(DurationNanosecondArray::from(vec![
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(DurationNanosecondArray::from(vec![None, Some(2), None])),
                Arc::new(DurationNanosecondArray::from(vec![Some(1), None, Some(3)])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-Binary",
            array: Arc::new(BinaryArray::from(vec![
                None,
                Some(&vec![1_u8][..]),
                None,
                Some(&vec![2_u8][..]),
                None,
                Some(&vec![3_u8][..]),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(BinaryArray::from(vec![None, Some(&vec![2_u8][..]), None])),
                Arc::new(BinaryArray::from(vec![
                    Some(&vec![1_u8][..]),
                    None,
                    Some(&vec![3_u8][..]),
                ])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-LargeBinary",
            array: Arc::new(LargeBinaryArray::from(vec![
                None,
                Some(&vec![1_u8][..]),
                None,
                Some(&vec![2_u8][..]),
                None,
                Some(&vec![3_u8][..]),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(LargeBinaryArray::from(vec![
                    None,
                    Some(&vec![2_u8][..]),
                    None,
                ])),
                Arc::new(LargeBinaryArray::from(vec![
                    Some(&vec![1_u8][..]),
                    None,
                    Some(&vec![3_u8][..]),
                ])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-String",
            array: Arc::new(StringArray::from(vec![
                None,
                Some("1"),
                None,
                Some("2"),
                None,
                Some("3"),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(StringArray::from(vec![None, Some("2"), None])),
                Arc::new(StringArray::from(vec![Some("1"), None, Some("3")])),
            ],
            error: "",
        },
        ArrayTest {
            name: "null-LargeString",
            array: Arc::new(LargeStringArray::from(vec![
                None,
                Some("1"),
                None,
                Some("2"),
                None,
                Some("3"),
            ])),
            indices: Arc::new(UInt64Array::from(vec![0, 1, 1, 0, 0, 1])),
            expect: vec![
                Arc::new(LargeStringArray::from(vec![None, Some("2"), None])),
                Arc::new(LargeStringArray::from(vec![Some("1"), None, Some("3")])),
            ],
            error: "",
        },
    ];

    for test in tests {
        let result = DataArrayScatter::scatter(
            &DataColumnarValue::Array(test.array.clone()),
            &DataColumnarValue::Array(test.indices.clone()),
            2,
        );

        match result {
            Ok(scatters_array) => {
                let scatters_array = scatters_array
                    .iter()
                    .map(DataColumnarValue::to_array)
                    .collect::<Result<Vec<_>>>()?;

                assert_eq!(
                    scatters_array, test.expect,
                    "failed in the test: {}",
                    test.name
                );
            }
            Err(e) => {
                assert_eq!(
                    test.error,
                    e.to_string(),
                    "failed in the test: {}",
                    test.name
                );
            }
        }
    }

    Ok(())
}

#[test]
fn test_scatter_array_with_constants_indices() -> Result<()> {
    let indices_values = vec![
        DataValue::Int8(Some(1)),
        DataValue::Int16(Some(1)),
        DataValue::Int32(Some(1)),
        DataValue::Int64(Some(1)),
        DataValue::UInt8(Some(1)),
        DataValue::UInt16(Some(1)),
        DataValue::UInt32(Some(1)),
        DataValue::UInt64(Some(1)),
    ];
    for data_value in indices_values {
        let result = DataArrayScatter::scatter(
            &DataColumnarValue::Array(Arc::new(Int8Array::from(vec![1, 2, 3]))),
            &DataColumnarValue::Constant(data_value, 3),
            2,
        )?;

        assert_eq!(result.len(), 2);
        match &result[0] {
            DataColumnarValue::Constant(_, _) => assert!(false, "result[0] must be a DataArray"),
            DataColumnarValue::Array(data) => {
                assert_eq!(data.len(), 0);
            }
        }

        match &result[1] {
            DataColumnarValue::Constant(_, _) => assert!(false, "result[1] must be a DataArray"),
            DataColumnarValue::Array(data) => {
                let expect: DataArrayRef = Arc::new(Int8Array::from(vec![1, 2, 3]));
                assert_eq!(data.len(), 3);
                assert_eq!(data, &expect);
            }
        }
    }

    Ok(())
}

#[test]
fn test_scatter_const_array_with_constants_indices() -> Result<()> {
    let result = DataArrayScatter::scatter(
        &DataColumnarValue::Constant(DataValue::Int8(Some(3)), 3),
        &DataColumnarValue::Constant(DataValue::Int8(Some(1)), 3),
        2,
    )?;

    assert_eq!(result.len(), 2);
    match &result[0] {
        DataColumnarValue::Array(_) => assert!(false, "result[0] must be a Constant"),
        DataColumnarValue::Constant(value, len) => {
            assert_eq!(*len, 0);
            assert_eq!(value, &DataValue::Int8(Some(3)));
        }
    }

    match &result[1] {
        DataColumnarValue::Array(_) => assert!(false, "result[1] must be a Constant"),
        DataColumnarValue::Constant(value, len) => {
            assert_eq!(*len, 3);
            assert_eq!(value, &DataValue::Int8(Some(3)));
        }
    }

    Ok(())
}
