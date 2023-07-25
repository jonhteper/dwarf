use native::Bill;
use rust_decimal::Decimal;
use wasm_bindgen::prelude::*;
use Lotus::Lotus;
mod native;

#[wasm_bindgen(getter_with_clone)]
pub struct BillResult {
    pub input: String,
    pub iva: String,
    pub isr: String,
    #[wasm_bindgen(js_name = taxesFree)]
    pub taxes_free: String,
    pub subtotal: String,
    pub total: String,
}

impl TryFrom<Bill> for BillResult {
    type Error = rust_decimal::Error;
    fn try_from(bill: Bill) -> Result<Self, Self::Error> {
        let formatter = Lotus::new("$", 3);
        let input_value = f64::try_from(bill.input)?;
        let iva_value = f64::try_from(bill.iva)?;
        let isr_value = f64::try_from(bill.isr)?;
        let taxes_free_value = f64::try_from(bill.taxes_free)?;
        let subtotal_value = f64::try_from(bill.subtotal)?;
        let total_value = f64::try_from(bill.total)?;

        let result = Self {
            input: formatter.format(input_value),
            iva: formatter.format(iva_value),
            isr: formatter.format(isr_value),
            taxes_free: formatter.format(taxes_free_value),
            subtotal: formatter.format(subtotal_value),
            total: formatter.format(total_value),
        };

        Ok(result)
    }
}

#[wasm_bindgen]
pub fn bill(input: f64) -> Option<BillResult> {
    if let Ok(n) = Decimal::try_from(input) {
        return native::bill(n).try_into().ok();
    }

    None
}

#[wasm_bindgen(js_name = reversedBill)]
pub fn reversed_bill(input: f64) -> Option<BillResult> {
    if let Ok(n) = Decimal::try_from(input) {
        return native::reversed_bill(n).try_into().ok();
    }

    None
}
