// import * as dna from 'dnaviz';
import init from "./pkg/index.js"

// import("../pkg/index.js").then(rustWasm => {

//     console.log("Write in JS, Read in Wasm, Index 0:");
  
//     rustWasm.transform('ATGC');
  
//     let wasmMemory = new Float32Array(rustWasm.memory.buffer);
  
//     let bufferPointer = rustWasm.get_wasm_memory_buffer_pointer();
  
//     console.log(wasmMemory[bufferPointer + 0]);

// })

const runWasm = async () => {

    const rustWasm = await init("./pkg/index_bg.wasm");
  
    rustWasm.transform('ATGC');
  
    let wasmMemory = new Float32Array(rustWasm.memory.buffer);
  
    let bufferPointer = rustWasm.get_wasm_memory_buffer_pointer();
  
    console.log(wasmMemory[bufferPointer + 0]);

  };
  runWasm();