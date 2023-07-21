all: format build

format:
	cargo fmt

build:
	wasm-pack build --target web

lint:
	cargo clippy

deploy:
	cp pkg/dwarf_wasm_bg.wasm ../web/dwarf-vue/src/pkg
	cp pkg/dwarf_wasm.js ../web/dwarf-vue/src/pkg
	cp pkg/dwarf_wasm.d.ts ../web/dwarf-vue/src/pkg