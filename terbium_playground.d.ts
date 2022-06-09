/* tslint:disable */
/* eslint-disable */
/**
*/
export function main(): void;
/**
* @param {string} content
* @returns {any}
*/
export function ast(content: string): any;
/**
* @param {string} code
* @returns {any}
*/
export function dis(code: string): any;
/**
* @param {string} code
* @returns {any}
*/
export function interpret(code: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly ast: (a: number, b: number) => number;
  readonly dis: (a: number, b: number) => number;
  readonly interpret: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
