# dwarf
Calculador de facturas RESICO.

Solo para personas físicas con actividad empresarial.


## Construcción

### Dependencias
* [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
* [`npm`](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

### Compilación

#### Binario WASM
***Nota**: Aunque el proyecto ya cuenta con una versión precompilada de la librería WASM, opcionalmente puede recompilarla.*

* Entre en `dwarf-wasm` y ejecute
```bash
make
make deploy
```

#### Aplicación Vue
Entre en `dwarf-vue` y ejecute 
```bash
npm run build
```

## Ejecutar en local
Entre en `dwarf-vue` y ejecute 
```bash
npm run dev
```
