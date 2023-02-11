
async function init() {
  const response = await fetch("static/add.wasm");
  const buffer = await response.arrayBuffer();
  const wasm = await WebAssembly.instantiate(buffer);
  const add = wasm.instance.exports.add;

  console.log(add(110, 89));
}

init();