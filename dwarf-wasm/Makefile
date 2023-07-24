all: format build

format:
	cargo fmt

build:
	wasm-pack build --target web

lint:
	cargo clippy

deploy:
	mkdir ../web/dwarf-vue/src/utils/dwarf || true
	cp pkg/dwarf_wasm_bg.wasm ../web/dwarf-vue/src/utils/dwarf
	cp pkg/dwarf_wasm.js ../web/dwarf-vue/src/utils/dwarf
	cp pkg/dwarf_wasm.d.ts ../web/dwarf-vue/src/utils/dwarf