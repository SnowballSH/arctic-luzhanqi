import init, { random_start } from '../../../../packages/wasm-bindings/pkg/luzhanqi_wasm.js';

export async function initWasm() {
    await init();
}

export { random_start };
