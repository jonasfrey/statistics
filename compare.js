import {
    min, 
    max,
    minSorted, 
    mean
} from "https://deno.land/x/simplestatistics@v7.8.3/index.js"
import {
    f_measure_time
} from "https://deno.land/x/date_functions@1.3/mod.js"
import {
    mean as mean2
} from "https://deno.land/x/stats@v0.1.0/mod.ts"
import * as wasm from "./pkg/wasm_speed_test.js";
await wasm.default()

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



f_measure_time('simplestatistics min')
console.log(min(o_test_array_u8.a_n_t))
f_measure_time()
f_measure_time('wasm min')
console.log(wasm.f_n_min__for_u8(o_test_array_u8.a_n_t))
f_measure_time()


f_measure_time('simplestatistics min max')
console.log(
    [min(o_test_array_u8.a_n_t), max(o_test_array_u8.a_n_t)]
)
f_measure_time()
f_measure_time('wasm min max')
console.log(wasm.f_a_n_min_n_max__for_u8(o_test_array_u8.a_n_t))
f_measure_time()



// f_measure_time('wasm min max mean sum, with tuple')
// console.log(
    // since the version with the tuple was slower i will take structs only in the future
//     wasm.f_a_n_mean_n_sum_n_min_n_max__for_u8_tuple(o_test_array_u8.a_n_t)
// )
// f_measure_time()
// f_measure_time('wasm min max mean sum, with struct')
// console.log(
//     wasm.f_a_n_mean_n_sum_n_min_n_max__for_u8(o_test_array_u8.a_n_t)
// )
// f_measure_time()
f_measure_time('wasm min... max... meansum...')
console.log(
    [

        wasm.f_n_min__for_u8(o_test_array_u8.a_n_t),
        wasm.f_n_max__for_u8(o_test_array_u8.a_n_t),
        wasm.f_o_n_mean_n_sum__for_u8(o_test_array_u8.a_n_t),
    ]
)
f_measure_time()

f_measure_time('wasm minmax... meansum...')
// i tested, 
// min max mean all in one loop, is not 
// min, max, mean functions all seperatly 
console.log(
    [
        ...wasm.f_a_n_min_n_max__for_u8(o_test_array_u8.a_n_t),
        wasm.f_o_n_mean_n_sum__for_u8(o_test_array_u8.a_n_t),
    ]
)
f_measure_time()


f_measure_time('simplestatistics min max mean sum')
console.log(
    [
        min(o_test_array_u8.a_n_t),
        max(o_test_array_u8.a_n_t),
        mean(o_test_array_u8.a_n_t)
    ]
)
f_measure_time()




f_measure_time('simplestatistics mean')
console.log(mean(o_test_array_u32.a_n_t))
f_measure_time()

f_measure_time('denostats module mean')
console.log(mean2(o_test_array_u32.a_n_t))
f_measure_time()

// f_measure_time('wasm mean')
// var o = (wasm.f_o_n_mean_n_sum__for_u32(o_test_array_u32.a_n_t))

// console.log(`o.n_mean: ${o.n_mean}`)
// console.log(`o.n_mean_nor: ${o.n_mean_nor}`)
// console.log(`o.n_sum: ${o.n_sum}`)
// f_measure_time()


// f_measure_time('wasm mean f32')
// console.log(o_test_array_f32_small.a_n_t.length)
// let a_f32 = new Float32Array(
//     new Array(2_000_000).fill(0).map(()=>
//     (Math.random()-.5)*Math.pow(2,32)
//     ))
    
//     console.log(o_test_array_f32_small.a_n_t);
//     console.log(a_f32)
// var o = wasm.f_o_n_mean_n_sum__for_f32(a_f32)
// console.log(`o.n_mean: ${o.n_mean}`)
// console.log(`o.n_mean_nor: ${o.n_mean_nor}`)
// console.log(`o.n_sum: ${o.n_sum}`)
// f_measure_time()

// now let the fun start
f_measure_time('simplestatistics mean for 4 channel array')
// ohhh on... simple statistics cannot do statistics on a multi channel array so we have to split into
// multiple arrays and loose time :'(
var reds = new Float32Array(o_test_array_f32_small.a_n_t.length/4);
var greens = new Float32Array(o_test_array_f32_small.a_n_t.length/4);
var blues = new Float32Array(o_test_array_f32_small.a_n_t.length/4);
var alphas = new Float32Array(o_test_array_f32_small.a_n_t.length/4);

for (let i = 0; i < o_test_array_f32_small.a_n_t.length; i += 4) {
    reds[i/4] = (o_test_array_f32_small.a_n_t[i]);     // Red channel
    greens[i/4] = (o_test_array_f32_small.a_n_t[i+1]); // Green channel
    blues[i/4] = (o_test_array_f32_small.a_n_t[i+2]);  // Blue channel
    alphas[i/4] = (o_test_array_f32_small.a_n_t[i+3]); // Alpha channel
}
console.log(mean(reds))
console.log(mean(greens))
console.log(mean(blues))
console.log(mean(alphas))
f_measure_time()
// f_measure_time('wasm mean, check this shit out')
// console.log(o_test_array_f32_small.a_n_t.length/4)
// var a_o = (wasm.f_a_o_n_mean_n_sum__for_f32(a_f32, 4))
// a_o.map(o=>{
//     console.log(o)
//     console.log(`o.n_mean: ${o.n_mean}`)
//     console.log(`o.n_mean_nor: ${o.n_mean_nor}`)
//     console.log(`o.n_sum: ${o.n_sum}`)

// })
// f_measure_time()


// // console.log(
// //     o_test_array_u64
// // );
// // Deno.exit()

// f_measure_time('native min max ')
// var n_min = o_test_array_u64.a_n_t[0];
// var n_max = o_test_array_u64.a_n_t[0];

// for(let n_it = 0; n_it < o_test_array_u64.a_n_t.length; n_it+=1){
//     let n = o_test_array_u64.a_n_t[n_it]
//     if(n < n_min){
//         n_min = n
//     } 
//     if(n > n_max){
//         n_max = n
//     }
// }
// console.log([n_min, n_max])
// f_measure_time()


// f_measure_time('wasm f_a_n_min_n_max array')
// var a_n_min_n_max2 = wasm.f_a_n_min_n_max__u64__array(o_test_array_u64.a_n_t);
// console.log(a_n_min_n_max2)
// f_measure_time()

// f_measure_time('deno simplestatistic module')
// console.log(min(o_test_array_u64.a_n_t))
// f_measure_time()





// // f_measure_time('wasm f_o_test_array_u64.a_n_t array')
// // var a_res = wasm.f_o_test_array_u64.a_n_t(o_test_array_u64.a_n_t);
// // // console.log(a_res);
// // console.log(a_res[0])
// // console.log(a_res.at(-1))
// // f_measure_time()


// // f_measure_time('native sort')
// // var a_res = o_test_array_u64.a_n_t.sort((a, b) => {
// //     if (a > b) return 1;
// //     if (a < b) return -1;
// //     return 0;
// // });
// // // console.log(o_test_array_u64.a_n_t)
// // console.log(a_res)
// // // console.log(a_res[0])
// // // console.log(a_res.at(-1))
// // f_measure_time()


// f_measure_time('wasm f_a_n_f32__sorted array')
// var a_res = wasm.f_a_n_f32__sorted(o_test_array_f32.a_n_t);
// console.log(a_res);
// f_measure_time()




// f_measure_time('wasm f_a_n_u16__sorted array')
// var a_res = wasm.f_a_n_u16__sorted(o_test_array_u16.a_n_t);
// console.log(a_res);
// f_measure_time()

