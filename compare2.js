import {
    min, 
    max,
    minSorted, 
    mean
} from "https://deno.land/x/simplestatistics@v7.8.3/index.js"
import {
    f_n_measure_time
} from "https://deno.land/x/date_functions@1.4/mod.js"
import {
    mean as mean2
} from "https://deno.land/x/stats@v0.1.0/mod.ts"
import {
    f_s_n_beautified
} from "https://deno.land/x/handyhelpers@1.2/mod.js"

import * as wasm from "./pkg/wasm_speed_test.js";
await wasm.default()
let a_n_u8__2560000_random_bytes =  await Deno.readFile('./random_test_data/2560000_random_bytes');
let a = new Uint8Array(a_n_u8__2560000_random_bytes.buffer);


let o_s_n_bytes_o_run={

}
let n_runs = 200;

for(let n_run = 0; n_run< n_runs; n_run+=1){
    let s_prop_run= `run_${n_run}`;
    console.log(s_prop_run)
    let n = 1000;
    while(n < a.length){
        n*=10;
        let a_sub = a.subarray(0, n)

        let s_n_bytes =`${f_s_n_beautified(a_sub.length, '_')}`;
        let s_prop_bytes = `bytes_${s_n_bytes}`
        if(!o_s_n_bytes_o_run[s_prop_bytes]){
            o_s_n_bytes_o_run[s_prop_bytes] = {}
        }

        console.log(s_prop_bytes)
        let n_ms_native = window.performance.now();
        var n_min = (min(a_sub))
        n_ms_native = window.performance.now()-n_ms_native;
        let n_ms_wasm = window.performance.now();
        var n_min = (wasm.f_n_min__for_u8(a_sub))
        n_ms_wasm = window.performance.now()-n_ms_wasm;
        let n_wasm = f_n_measure_time()

        if(!o_s_n_bytes_o_run[s_prop_bytes][s_prop_run]){
            o_s_n_bytes_o_run[s_prop_bytes][s_prop_run] = {
                'n_ms_native': parseInt(n_ms_native).toString().padStart(5, ' ')      , 
                'n_ms_wasm': parseInt(n_ms_wasm).toString().padStart(5, ' ')
            }
        }

    }

}
console.log(o_s_n_bytes_o_run)
Deno.writeTextFile(`./o_s_n_bytes_o_run_last.json`, JSON.stringify(o_s_n_bytes_o_run, null, 4))
Deno.writeTextFile(`./o_s_n_bytes_o_run_${new Date().getTime()}.json`, JSON.stringify(o_s_n_bytes_o_run, null, 4))