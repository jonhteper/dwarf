use native::Bill;
use rust_decimal::Decimal;
use wasm_bindgen::prelude::*;
mod native;

#[wasm_bindgen(getter_with_clone)]
pub struct BillResult {
    pub input: String,
    pub iva: String,
    pub isr: String,
    pub taxes_free: String,
    pub subtotal: String,
    pub total: String,
}

impl From<Bill> for BillResult {
    fn from(bill: Bill) -> Self {
        Self {
            input: bill.input.to_string(),
            iva: bill.iva.to_string(),
            isr: bill.isr.to_string(),
            taxes_free: bill.taxes_free.to_string(),
            subtotal: bill.subtotal.to_string(),
            total: bill.total.to_string(),
        }
    }
}

#[wasm_bindgen]
pub fn bill(input: f64) -> Option<BillResult> {
    if let Ok(n) = Decimal::try_from(input) {
        return Some(native::bill(n).into());
    }

    None
}

#[wasm_bindgen]
pub fn reversed_bill(input: f64) -> Option<BillResult> {
    if let Ok(n) = Decimal::try_from(input) {
        return Some(native::reversed_bill(n).into());
    }

    None
}
