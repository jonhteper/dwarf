import { createStore } from "vuex";
import { read_storage } from "./utils/storage";

export default createStore({
    state: {
        storedBills: read_storage(),
    },
    mutations: {
        updateStoredBills(state) {
            state.storedBills = read_storage();
        },
    },
    getters: {
        storedBills: (state) => state.storedBills,
    },
});
