pub mod data;

/// Re-exports for our public API.
pub mod ext {
    pub use bigdecimal;
    pub use num_bigint;
}

#[cfg(test)]
mod tests {
    use crate::data::*;
    use bigdecimal::BigDecimal;
    use num_bigint::BigInt;
    use rust_decimal::Decimal;
    use std::mem::size_of;

    #[test]
    fn todo() {
        let bigint_size = size_of::<BigInt>();
        println!("BigInt: {bigint_size}");
        let bigdec_size = size_of::<BigDecimal>();
        println!("BigDec: {bigdec_size}");
        let dec_size = size_of::<Decimal>();
        println!("Decimal: {dec_size}");
        let scalar_size = size_of::<Scalar>();
        println!("Scalar: {scalar_size}");
    }
}
