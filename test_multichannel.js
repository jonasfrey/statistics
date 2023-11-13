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

let o = wasm.f__o_n_max_for_f32(o_test_array_f32.a_n_t);
console.log(o.n_max)