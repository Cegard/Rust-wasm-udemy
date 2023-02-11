
async function init() {
  const add_console = {
    console: {
      log: () => {
      console.log("Logging from the .wasm file.");
      },
      error: () => {
        console.log("Oops... That's all.");
      }
    }
  };

  const response = await fetch("static/add.wasm");
  const buffer = await response.arrayBuffer();
  const wasm = await WebAssembly.instantiate(buffer, add_console);
  const add = wasm.instance.exports.add;
  const wasmMemory = wasm.instance.exports.mem;
  const uInt8Array = new Uint8Array(wasmMemory.buffer, 0, 2);
  const decodedMessage = new TextDecoder().decode(uInt8Array);

  console.log(add(110, 89), decodedMessage);
}

init();