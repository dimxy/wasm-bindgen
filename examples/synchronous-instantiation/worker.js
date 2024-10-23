//import * as wasm from "./pkg/synchronous_instantiation.js";
self.wasm = await import("./pkg/synchronous_instantiation.js")

self.onmessage = async ({ data: bytes }) => {

  /**
   * When we receive the bytes as an `ArrayBuffer` we can use that to
   * synchronously initialize the module as opposed to asynchronously
   * via the default export. The synchronous method internally uses
   * `new WebAssembly.Module()` and `new WebAssembly.Instance()`.
   */
  wasm.initSync({ module: bytes });
  console.log("worker wasm.init'ed");

  /**
   * Once initialized we can call our exported `greet()` functions.
   */
  //await new Promise(r => setTimeout(r, 2000));
  console.log("worker calling wasm.greet");
  wasm.greet("in-worker");
};

/**
 * Once the Web Worker was spawned we ask the main thread to fetch the bytes
 * for the WebAssembly module. Once fetched it will send the bytes back via
 * a `postMessage` (see above).
 */

console.log("worker started");
//await new Promise(r => setTimeout(r, 2000)); // wait for 2 sec for workers to start
console.log("posting FETCH_WASM");
self.postMessage({ type: "FETCH_WASM" });
