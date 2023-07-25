import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import init_wasm from "./utils/dwarf/dwarf_wasm";
import store from "./store";

init_wasm().then(() => {
    console.log("Webassembly loaded!");
    createApp(App).use(store).mount("#app");
});
