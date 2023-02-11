
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

  console.log(add(110, 89));
}

init();