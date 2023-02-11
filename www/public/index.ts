
async function init() {
  const memory = new WebAssembly.Memory({initial: 1});

  const add_console = {
    js: {
      mem: memory
    },
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
  const uInt8Array = new Uint8Array(memory.buffer, 0, 3);
  const decodedMessage = new TextDecoder().decode(uInt8Array);

  console.log(add(110, 89), decodedMessage);
}

init();