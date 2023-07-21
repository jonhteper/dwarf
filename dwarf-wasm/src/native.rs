use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
pub struct Bill {
    pub input: Decimal,
    pub iva: Decimal,
    pub isr: Decimal,
    pub taxes_free: Decimal,
    pub subtotal: Decimal,
    pub total: Decimal,
}

pub fn bill(input: Decimal) -> Bill {
    let n = input * Decimal::ONE_HUNDRED / Decimal::try_from(98.75).unwrap();
    let iva = n * Decimal::try_from(0.16).unwrap();
    let isr = n * Decimal::try_from(0.0125).unwrap();
    let total = n + iva - isr;
    let taxes_free = total - iva;

    Bill {
        input: input.round_dp(3),
        iva: iva.round_dp(3),
        isr: isr.round_dp(3),
        total: total.round_dp(3),
        taxes_free: taxes_free.round_dp(3),
        subtotal: (taxes_free + isr).round_dp(3),
    }
}

pub fn reversed_bill(input: Decimal) -> Bill {
    let n = input * Decimal::try_from(87.1459).unwrap() / Decimal::ONE_HUNDRED;
    let iva = n * Decimal::try_from(0.16).unwrap();
    let isr = n * Decimal::try_from(0.0125).unwrap();
    let total = n + iva - isr;
    let taxes_free = n - isr;

    Bill {
        input: input.round_dp(3),
        iva: iva.round_dp(3),
        isr: isr.round_dp(3),
        total: total.round_dp(3),
        taxes_free: taxes_free.round_dp(3),
        subtotal: n.round_dp(3),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = bill(Decimal::from(8_000));
        dbg!(result);
        assert_eq!(result.total.to_string(), "9296.203");
        let a: f64 = result.total.try_into().unwrap();
        assert_eq!(a, 9296.203);

        let result2 = reversed_bill(Decimal::try_from(9_296.20).unwrap());
        dbg!(result2);
        assert_eq!(result2.total.to_string(), "9296.193");
    }
}
