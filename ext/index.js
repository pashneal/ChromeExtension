console.log("yep");
import init from "./pkg/dafuq.js";

console.log("uhuh");
init("./pkg/dafuq_bg.wasm").then(wasm => {
  
  if (wasm.__wbindgen_start == undefined) {
    console.log("done");
    wasm.main();
  }
  console.log("already defined");

});
