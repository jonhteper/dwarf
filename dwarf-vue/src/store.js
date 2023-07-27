import { createStore } from "vuex";
import { readStorage } from "./utils/storage";

export default createStore({
    state: {
        storedBills: readStorage(),
        selectedBill: {},
    },
    mutations: {
        updateStoredBills(state) {
            state.storedBills = readStorage();
        },
        updateSelectedBill(state, key) {
            state.selectedBill = state.storedBills.get(key);
        },
        clearSelectedBill(state) {
            state.selectedBill = {};
        },
    },
    getters: {
        storedBills: (state) => state.storedBills,
        selectedBill: (state) => state.selectedBill,
    },
});
