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

import * as o_wasm from "./pkg/wasm_speed_test.js";
await o_wasm.default()
let a_n_u8__2560000_random_bytes =  await Deno.readFile('./random_test_data/2560000_random_bytes');
let a_n_u8 = new Uint8Array(a_n_u8__2560000_random_bytes.buffer);

let o_s_n_bytes_o_run={

}
let n_runs = 1;

for(let n_run = 0; n_run< n_runs; n_run+=1){
    let s_prop_run= `run_${n_run}`;
    console.log(s_prop_run)
    let n = 1000;
    while(n < 10090000){//a.length){
        n*=10;
        let a_sub = a_n_u8.subarray(0, n)

        let s_n_bytes =`${f_s_n_beautified(a_sub.length, '_')}`;
        let s_prop_bytes = `bytes_${s_n_bytes}`
        if(!o_s_n_bytes_o_run[s_prop_bytes]){
            o_s_n_bytes_o_run[s_prop_bytes] = {}
        }

        f_n_measure_time('wasm sort')
        var a_res = o_wasm.f_a_n_u8__sorted(a_sub);
        let n_ms_wasm = f_n_measure_time()
        f_n_measure_time('native sort')
        var a_res = a_sub.sort((n1,n2)=>{return n1-n2})
        let n_ms_native = f_n_measure_time()

        if(!o_s_n_bytes_o_run[s_prop_bytes][s_prop_run]){
            o_s_n_bytes_o_run[s_prop_bytes][s_prop_run] = {
                'n_ms_native': n_ms_native, 
                'n_ms_wasm': n_ms_wasm
            }
        }

    }

}
console.log(o_s_n_bytes_o_run)
Deno.writeTextFile(`./sort_o_s_n_bytes_o_run_last.json`, JSON.stringify(o_s_n_bytes_o_run, null, 4))
Deno.writeTextFile(`./sort_o_s_n_bytes_o_run_${new Date().getTime()}.json`, JSON.stringify(o_s_n_bytes_o_run, null, 4))