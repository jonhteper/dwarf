const LOCAL_STORAGE_NAME = "resico.db";

export const save_in_storage = (bill) => {
    let bills_map = read_storage();
    let now = Date.now();
    bills_map.set(now.toString(), {
        input: bill.input,
        isr: bill.isr,
        iva: bill.iva,
        subtotal: bill.subtotal,
        taxesFree: bill.taxes_free,
        total: bill.total,
        createdAt: new Date(now).toLocaleString(),
    });
    localStorage.setItem(
        LOCAL_STORAGE_NAME,
        JSON.stringify(Object.fromEntries(bills_map))
    );

    return bills_map;
};

export const read_storage = () => {
    const saved_values = localStorage.getItem(LOCAL_STORAGE_NAME) || "{}";
    return new Map(Object.entries(JSON.parse(saved_values)));
};

export const deleteBill = (key) => {
    const savedBills = read_storage();
    savedBills.delete(key);

    localStorage.setItem(
        LOCAL_STORAGE_NAME,
        JSON.stringify(Object.fromEntries(savedBills))
    );
};
