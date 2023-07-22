const LOCAL_STORAGE_NAME = "resico.db";

export const save_in_storage = (bill) => {   
    let bills_list = read_storage();
    bills_list.push({
        input: bill.input,
        isr: bill.isr,
        iva: bill.iva,
        subtotal: bill.subtotal,
        taxes_free: bill.taxes_free,
        total: bill.total,
    });
    localStorage.setItem(LOCAL_STORAGE_NAME, JSON.stringify(bills_list));
};

export const read_storage = () => {
    const saved_values = localStorage.getItem(LOCAL_STORAGE_NAME) || "[]";
    return JSON.parse(saved_values);
}