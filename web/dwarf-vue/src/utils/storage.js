const LOCAL_STORAGE_NAME = "resico.db";

export const saveInStorage = (bill) => {
    let billsMap = readStorage();
    let now = Date.now();
    billsMap.set(now.toString(), {
        input: bill.input,
        isr: bill.isr,
        iva: bill.iva,
        subtotal: bill.subtotal,
        taxesFree: bill.taxesFree,
        total: bill.total,
        createdAt: new Date(now).toLocaleString(),
    });
    localStorage.setItem(
        LOCAL_STORAGE_NAME,
        JSON.stringify(Object.fromEntries(billsMap))
    );

    return billsMap;
};

export const readStorage = () => {
    const savedValues = localStorage.getItem(LOCAL_STORAGE_NAME) || "{}";
    return new Map(Object.entries(JSON.parse(savedValues)));
};

export const deleteBill = (key) => {
    const savedBills = readStorage();
    savedBills.delete(key);

    localStorage.setItem(
        LOCAL_STORAGE_NAME,
        JSON.stringify(Object.fromEntries(savedBills))
    );
};
