import {
    min, 
    max
} from "https://deno.land/x/simplestatistics@v7.8.3/index.js"
import {
    f_measure_time
} from "https://deno.land/x/date_functions@1.3/mod.js"
import * as wasm from "./pkg/wasm_speed_test.js";
await wasm.default()
let n_its = 1_000_000;
let a_n_u64 = new BigUint64Array(new Array(n_its).fill(0).map(n=>BigInt(parseInt(Math.random()*(Math.pow(2,64)-1)))))
// console.log(a_n_u64)

let f_a_n_min_n_max = function(a_n){
    if(a_n instanceof BigInt64Array){
        return wasm.f_a_n_min_n_max__u64__array(a_n)
    }
}
f_measure_time('native deno js min max u64')

var n_min = a_n_u64[0];
var n_max = a_n_u64[0];

for(let n_it = 0; n_it < a_n_u64.length; n_it+=1){
    let n = a_n_u64[n_it]
    if(n < n_min){
        n_min = n
    } 
    if(n > n_max){
        n_max = n
    }
}
console.log([n_min, n_max])
f_measure_time()


f_measure_time('wasm f_a_n_min_n_max array')
var a_n_min_n_max2 = wasm.f_a_n_min_n_max__u64__array(a_n_u64);
console.log(a_n_min_n_max2)
f_measure_time()

f_measure_time('deno simplestatistic module')

console.log(min(a_n_u64))
f_measure_time()