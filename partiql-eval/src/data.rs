// Copyright Amazon.com, Inc. or its affiliates.

//! Provides the runtime notion of data for PartiQL.
//!
//! This module can be thought of as providing the moral equivalent of *data frames* for PartiQL.
//! We refrain from using that terminology here because data frames typically refer to tables with
//! rows that have columns with fully concrete schema types.
//!
//! This API is currently experimental and under heavy development.

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use rust_decimal::Decimal;

/// Representation of `NULL` and `MISSING`.
pub enum NullKind {
    Value,
    Missing,
}

/// Various representation of integers.
pub enum IntKind {
    I32(i32),
    I64(i64),
    Big(BigInt),
}

/// 32/64-bit float.
pub enum FloatKind {
    F32(f32),
    F64(f64),
}

/// Decimal
pub enum DecimalKind {
    Fixed(Decimal),
    Arbitrary(BigDecimal),
}

// TODO the timestamp/time/date/etc.

/// Represents a scalar value
pub enum Scalar {
    Null(NullKind),
    Bool(bool),
    Int(IntKind),
    Decimal(DecimalKind),
}
