import fs from "node:fs";

let mem = undefined;

const toJsString = (offset) => {
  const view = new DataView(mem.buffer);
  const chars = [];
  const len = view.getInt32(offset, true);

  for (let i = 0; i < len; i++) {
    chars.push(view.getInt32(offset + (i + 1) * 4, true));
  }

  return String.fromCharCode(...chars);
}

const __consoleLog = (offset) => { console.log(toJsString(offset)) };

const code = new Uint8Array(fs.readFileSync("./hello.wasm"));
const mod = new WebAssembly.Module(code);
const instance = new WebAssembly.Instance(mod, { waferImports: { __consoleLog }});

mem = instance.exports.$waferMemory;
instance.exports.main();
