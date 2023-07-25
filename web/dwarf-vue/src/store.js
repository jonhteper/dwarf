import { createStore } from "vuex";
import { read_storage } from "./utils/storage";

export default createStore({
    state: {
        storedBills: read_storage(),
        selectedBill: {},
    },
    mutations: {
        updateStoredBills(state) {
            state.storedBills = read_storage();
        },
        updateSelectedBill(state, key) {
            state.selectedBill = state.storedBills.get(key);
            console.warn(state.selectedBill)
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
