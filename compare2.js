import * as o_mod_simplestatistics from "https://deno.land/x/simplestatistics@v7.8.3/index.js"

import {
    mean as mean2
} from "https://deno.land/x/stats@v0.1.0/mod.ts"
import {
    f_s_n_beautified
} from "https://deno.land/x/handyhelpers@1.2/mod.js"

import * as wasm from "./pkg/wasm_speed_test.js";
await wasm.default()

function f_s_type(typedArray) {
    const typeName = Object.prototype.toString.call(typedArray);
    const matches = typeName.match(/\[object (\w+)\]/);

    if (matches && matches[1]) {
        const type = matches[1];
        // Extract the relevant parts from the type name
        const arrayType = type.replace('Array', '');
        const typeChar = arrayType.startsWith('Uint') ? 'u' : 'i';
        const typeNumber = arrayType.match(/\d+/)[0];

        return `${typeChar}${typeNumber}`;
    } else {
        return 'unknown';
    }
}
let O_typed_array = Uint32Array

let a_n_u8__2048000_random_bytes =  await Deno.readFile('./random_test_data/2048000_random_bytes');
let a_n_u8__512000_random_bytes =  await Deno.readFile('./random_test_data/512000_random_bytes');
let a_orig = a_n_u8__512000_random_bytes


function generateLargeUintArrayWithRandomMean(size) {
    const array = new O_typed_array(size);
    for (let i = 0; i < size; i++) {
        // Randomly decide the range of the number to be generated
        if (Math.random() < 0.5) {
            // 50% chance to generate a number in the lower range (0-127)
            array[i] = Math.floor(Math.random() * 128);
        } else {
            // 50% chance to generate a number in the upper range (128-255)
            array[i] = Math.floor(Math.random() * 128) + 128;
        }
    }
    return array;
}

let f_o_run_comparison = function(
    O_typed_array, 
    a_n_u8, 
    s_prop, 
    s_f_wasm, 
    s_f_native,
    n_runs
){
    let a = new O_typed_array(a_n_u8);

    let s_type = f_s_type(a);

    let o_s_n_elements_o_run={

    }
    
    for(let n_run = 0; n_run< n_runs; n_run+=1){
        let s_prop_run= `o_run_${n_run}`;
        // console.log(s_prop_run)
        let n = 1000;
        while(n < a.length){
            n*=10;
            let a_sub = a.subarray(0, n)
    
            let s_n_elements =`${f_s_n_beautified(a_sub.length, '_')}`;
            let s_prop_elements = `o_${s_type}_elements_${s_n_elements}`
            if(!o_s_n_elements_o_run[s_prop_elements]){
                o_s_n_elements_o_run[s_prop_elements] = {}
            }
    
            console.log(s_prop_elements)
            let n_ms_native = window.performance.now();
            var n_res = (o_mod_simplestatistics[eval(s_f_native)](a_sub))
            console.log(n_res)
            n_ms_native = window.performance.now()-n_ms_native;
            let n_ms_wasm = window.performance.now();
            let s = eval(s_f_wasm);
            console.log(s)
            var o_res = (wasm[s](a_sub))
            console.log(o_res[`n_${s_prop}`])
    
            n_ms_wasm = window.performance.now()-n_ms_wasm;
    
            if(!o_s_n_elements_o_run[s_prop_elements][s_prop_run]){
                o_s_n_elements_o_run[s_prop_elements][s_prop_run] = {
                    n_ms_native : n_ms_native,
                    n_ms_wasm : n_ms_wasm,
                    // 'n_ms_native': parseInt(n_ms_native).toString().padStart(5, ' ')      , 
                    // 'n_ms_wasm': parseInt(n_ms_wasm).toString().padStart(5, ' ')
                }
            }
    
        }
    
    }
    for(let s_prop_elements in o_s_n_elements_o_run){
        let n_mean_native = o_mod_simplestatistics.mean(Object.values(o_s_n_elements_o_run[s_prop_elements]).map(o=>o.n_ms_native));
        let n_mean_wasm = o_mod_simplestatistics.mean(Object.values(o_s_n_elements_o_run[s_prop_elements]).map(o=>o.n_ms_wasm));
    
        o_s_n_elements_o_run[s_prop_elements][`o_${s_prop}`] = {
            n_mean_native, 
            n_mean_wasm
        }
    }
    return {
        [`${s_prop}_${s_type}`]:o_s_n_elements_o_run
    }
}
let a_O_typed_array = [
    Uint8Array, 
    Uint16Array, 
    Uint32Array, 
    BigUint64Array, 
    Int8Array, 
    Int32Array, 
    BigInt64Array, 
    Float32Array, 
    Float64Array
]
let a_o_comparison = [
    a_O_typed_array.map(O_typed_array=>{
        return [
             f_o_run_comparison(
                O_typed_array, 
                a_orig.buffer, 
                'min', 
                '`f_o_n_${s_prop}_for_${s_type}`',
                '`${s_prop}`',
                10
            ),
            f_o_run_comparison(
                O_typed_array, 
                a_orig.buffer, 
                'max', 
                '`f_o_n_${s_prop}_for_${s_type}`',
                '`${s_prop}`',
                10
            ),
            f_o_run_comparison(
                O_typed_array, 
                a_orig.buffer, 
                'mean', 
                '`f_o_n_${s_prop}_for_${s_type}`',
                '`${s_prop}`',
                1
            )
        ]
    }).flat(2)
]


Deno.writeTextFile(`./a_o_comparison.json`, JSON.stringify(a_o_comparison, null, 4))
Deno.writeTextFile(`./a_o_comparison${new Date().getTime()}.json`, JSON.stringify(a_o_comparison, null, 4))