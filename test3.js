import {
    min, 
    max,
    // minmax,
    minSorted, 
    variance,
    mean
} from "https://deno.land/x/simplestatistics@v7.8.3/index.js"
import {
    f_measure_time
} from "https://deno.land/x/date_functions@1.3/mod.js"
import {
    mean as mean2
} from "https://deno.land/x/stats@v0.1.0/mod.ts"
//this code was automatically generated
import * as o_wasm from "./pkg/wasm_speed_test.js";
await o_wasm.default();
import * as o_mod_functions from './src/functions.module.js'

import {
    f_o_testdata
}from "./functions.module.js"
let o_testdata = await f_o_testdata();

let o_test_array_u64 = o_testdata.a_o_test_array
                .filter(o=>o.s_type =='u64')
                .sort((o1, o2)=>{return o1.n_len_array - o2.n_len_array}).at(-1);
let o_test_array_u64_small = o_testdata.a_o_test_array
                .filter(o=>o.s_type =='u64')
                .sort((o1, o2)=>{return o1.n_len_array - o2.n_len_array}).at(0);
let o_test_array_u32 = o_testdata.a_o_test_array
                .filter(o=>o.s_type =='u32')
                .sort((o1, o2)=>{return o1.n_len_array - o2.n_len_array}).at(-1);
// console.log(`len o_test_array_u32: ${o_test_array_u32.a_n_t.byteLength}`)
let o_test_array_u16 = o_testdata.a_o_test_array
                .filter(o=>o.s_type =='u16')
                .sort((o1, o2)=>{return o1.n_len_array - o2.n_len_array}).at(-1);
let o_test_array_u8 = o_testdata.a_o_test_array
                .filter(o=>o.s_type =='u8')
                .sort((o1, o2)=>{return o1.n_len_array - o2.n_len_array}).at(-1);
let o_test_array_f32 = o_testdata.a_o_test_array
                .filter(o=>o.s_type =='f32')
                .sort((o1, o2)=>{return o1.n_len_array - o2.n_len_array}).at(-1);

let o_test_array_f32_small = o_testdata.a_o_test_array
                .filter(o=>o.s_type =='f32')
                .sort((o1, o2)=>{return o1.n_len_array - o2.n_len_array}).at(0);


// let a_n_u8_2000mb_random =  await Deno.readFile('./random_test_data/random_bytes_2000mb.bin');
// let a_n_u8_20G_random =  await Deno.readFile('./random_test_data/random_bytes_20G.bin');
// let a_n_u8_10G_random =  await Deno.readFile('./random_test_data/random_bytes_10G.bin');

// var o = o_mod_functions.f_o_n_max(new Float32Array([
//     2.123, 0.231, -231.23, 2.123 -0.33, 1012039.22
// ]));

// console.log(o)


// let a = o_test_array_u64.a_n_t
let a = o_test_array_u16.a_n_t
// let a = o_test_array_u8.a_n_t
// let a = a_n_u8_500mb_random
// let a = new Uint8Array(a_n_u8_2000mb_random.buffer);
// let a = new Uint8Array(a_n_u8_20G_random.buffer);
// let a = new Uint32Array(a_n_u8_10G_random.buffer);
// f_measure_time('my')
// // var o = o_mod_functions.f_o_n_min_n_max_n_mean_n_variance(a);
// var o = o_mod_functions.f_o_n_min(a);
// console.log(o)
// // var o = o_wasm.f_o_n_max_for_u8(a);
// // console.log(o.n_max)
// // var a2 = o_wasm.f_a_n_min_n_max__for_u8(a);
// // console.log(a2)
// f_measure_time()

f_measure_time('them')
// minmax(a)
console.log(min(a))
// console.log(max(a))
// console.log(mean(a))
// console.log(variance(a))
f_measure_time()


let f_n_min_single = async function(
    a_v, 
    n_idx_start, 
    n_idx_end
){
    n_idx_end = Math.min(a_v.length, n_idx_end)
    // console.log(`len: ${a_v.length}, start:${n_idx_start}, end:${n_idx_end}`)
    return new Promise((f_res)=>{

        var n_min = a_v[0];
        for(var n = n_idx_start; n<n_idx_end; n++){
            if(a_v[n] < n_min) {
                n_min = a_v[n]
            }
        }
        return f_res(n_min)
    })
}
// ok wtf i wasted my life 
let f_n_min = async function(a_v, n_parallel){
    let n_idx_per_parallel = a_v.length / n_parallel;
    let a_n_min = await Promise.all([
        ...new Array(n_parallel).fill(0).map(
            (v, n_idx)=>{
                let n_idx_start = n_idx*n_idx_per_parallel
                let n_idx_end = Math.min(
                    a_v.length,
                    n_idx*n_idx_per_parallel + n_idx+1*n_idx_per_parallel)
                let a_sub = a_v.subarray(
                    n_idx_start, 
                    n_idx_end
                            
                    )
                    console.log(n_idx_start, n_idx_end)
                return f_n_min_single(
                    a_v, 
                    n_idx_start, 
                    n_idx_end
                    )
            }
        )
    ]
    );
    console.log(a_n_min)
    return f_n_min_single(a_n_min, 0, n_parallel);
    
}
// ok wtf i wasted my life 
let f_n_min_ss = async function(a_v, n_parallel){
    let n_idx_per_parallel = a_v.length / n_parallel;
    let a_n_min = await Promise.all([
        ...new Array(n_parallel).fill(0).map(
            (v, n_idx)=>{
                let n_idx_start = n_idx*n_idx_per_parallel
                let n_idx_end = Math.min(
                    a_v.length,
                    n_idx*n_idx_per_parallel + n_idx+1*n_idx_per_parallel)
                let a_sub = a_v.subarray(
                    n_idx_start, 
                    n_idx_end
                            
                    )
                    console.log(n_idx_start, n_idx_end)
                return new Promise(
                    (async f_res=>{
                        let n = await min(
                            a_sub
                        )
                        return f_res(n)
                    })
                )

            }
        )
    ]
    );
    return min(a_n_min);
    
}

// f_measure_time('newest 2parallel')
// // minmax(a)
// console.log(await f_n_min(a, 2))
// f_measure_time()

// f_measure_time('newest 4parallel')
// // minmax(a)
// console.log(await f_n_min(a, 4))
// f_measure_time()

// f_measure_time('newest 8parallel')
// // minmax(a)
// console.log(await f_n_min(a, 8))
// f_measure_time()

// f_measure_time('newest 16parallel')
// // minmax(a)
// console.log(await f_n_min(a, 16))
// f_measure_time()

f_measure_time('newest 1parallel')
// minmax(a)
console.log(await f_n_min(a, 16))
f_measure_time()

// f_measure_time('simplestat')
// // minmax(a)
// console.log(min(a))
// f_measure_time()
// f_measure_time('simplestat async')
// // minmax(a)
// console.log(await f_n_min_ss(a, 4))
// f_measure_time()