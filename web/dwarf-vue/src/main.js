import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import init_wasm from './pkg/dwarf_wasm'

init_wasm().then(() => {
    console.log('Webassembly loaded!')
    createApp(App).mount('#app')
});

