import { 
    PromiseArray
} from "https://deno.land/x/promise_array_parallel@0.0.1-alpha.6/mod.ts"

import {
    f_measure_time
} from "https://deno.land/x/date_functions@1.3/mod.js"
let a_n_u8__2560000_random_bytes =  await Deno.readFile('./random_test_data/2560000_random_bytes');
// let a_n_u8__5120000_random_bytes =  await Deno.readFile('./random_test_data/5120000_random_bytes');
var a_v = new Uint16Array(a_n_u8__2560000_random_bytes.buffer)
import * as o_wasm from "./pkg/wasm_speed_test.js";
await o_wasm.default();


let n_parallel = 2;
let n_idx_per_parallel = a_v.length / n_parallel;


let f_n_min = function(a){
    var n_min = a[0];
    for(var n = 0; n<a.length; n++){
        if(a[n] < n_min) {
            n_min = a[n]
        }
    }
    return n_min;
}
// a_v = new Uint8Array(new Array(2000*2000*5).fill(0).map(
//     ()=>{return parseInt(Math.random()*255)}
// ))

f_measure_time("serial")
console.log(f_n_min(a_v))
f_measure_time()

f_measure_time("wasm min")
console.log(o_wasm.f_o_n_min_for_u16(a_v))
f_measure_time()


f_measure_time("async")
let a_n_min2 = await Promise.all([
    ...new Array(n_parallel).fill(0).map(
        (v, n_idx)=>{

            let n_idx_start = n_idx*n_idx_per_parallel
            let n_idx_end = Math.min(
                a_v.length,
                n_idx*n_idx_per_parallel + n_idx+1*n_idx_per_parallel)

            return f_n_min(a_v.subarray(n_idx_start, n_idx_end)) 

        }
    )
]
)
console.log(`min from async ${f_n_min(a_n_min2)}`)
f_measure_time()


f_measure_time("paralllel")
    let a_n_min = await PromiseArray.from([
        ...new Array(n_parallel).fill(0).map(
            (v, n_idx)=>{

                let n_idx_start = n_idx*n_idx_per_parallel
                let n_idx_end = Math.min(
                    a_v.length,
                    n_idx*n_idx_per_parallel + n_idx+1*n_idx_per_parallel)

                return a_v.subarray(n_idx_start, n_idx_end) 

            }
        )
    ]
    ).parallelWork(async({idx, value}) => {
        let n_min= f_n_min(value);
        console.log(idx)
        if(idx == (n_parallel-1)){
            console.log(a2)
            console.log('done')
            console.log(f_n_min(a2))
            console.log(`nmin ${f_n_min(a2)}`)
            f_measure_time()

        }
        console.log(n_min)
        a2.push(n_min)
        
    })



