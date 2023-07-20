use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Bill {
    pub input: Decimal,
    pub iva: Decimal,
    pub isr: Decimal,
    pub taxes_free: Decimal,
    pub total: Decimal,
}

pub fn bill(input: Decimal) -> Bill {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
