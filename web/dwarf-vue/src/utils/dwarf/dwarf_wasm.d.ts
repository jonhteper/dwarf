/* tslint:disable */
/* eslint-disable */
/**
* @param {number} input
* @returns {BillResult | undefined}
*/
export function bill(input: number): BillResult | undefined;
/**
* @param {number} input
* @returns {BillResult | undefined}
*/
export function reversed_bill(input: number): BillResult | undefined;
/**
*/
export class BillResult {
  free(): void;
/**
*/
  input: string;
/**
*/
  isr: string;
/**
*/
  iva: string;
/**
*/
  subtotal: string;
/**
*/
  taxes_free: string;
/**
*/
  total: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_billresult_free: (a: number) => void;
  readonly __wbg_get_billresult_input: (a: number, b: number) => void;
  readonly __wbg_set_billresult_input: (a: number, b: number, c: number) => void;
  readonly __wbg_get_billresult_iva: (a: number, b: number) => void;
  readonly __wbg_set_billresult_iva: (a: number, b: number, c: number) => void;
  readonly __wbg_get_billresult_isr: (a: number, b: number) => void;
  readonly __wbg_set_billresult_isr: (a: number, b: number, c: number) => void;
  readonly __wbg_get_billresult_taxes_free: (a: number, b: number) => void;
  readonly __wbg_set_billresult_taxes_free: (a: number, b: number, c: number) => void;
  readonly __wbg_get_billresult_subtotal: (a: number, b: number) => void;
  readonly __wbg_set_billresult_subtotal: (a: number, b: number, c: number) => void;
  readonly __wbg_get_billresult_total: (a: number, b: number) => void;
  readonly __wbg_set_billresult_total: (a: number, b: number, c: number) => void;
  readonly bill: (a: number) => number;
  readonly reversed_bill: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
