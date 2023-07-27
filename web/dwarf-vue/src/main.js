import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import initWasm from "./utils/dwarf/dwarf_wasm";
import store from "./store";
import { dynStartTutorial } from "./utils/tutorial";

initWasm().then(() => {
    console.log("Webassembly loaded!");
    createApp(App).use(store).mount("#app");
    dynStartTutorial();
});
