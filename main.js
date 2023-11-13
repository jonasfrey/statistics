// let a_n_u8__wasm = await Deno.readFile('./pkg/wasm_speed_test_bg.wasm')
import * as wasm from "./pkg/wasm_speed_test.js";
import {
    f_measure_time
} from "https://deno.land/x/date_functions@1.3/mod.js"

await wasm.default();

//write test arrays 


let n_its = 30_000_000;

var a = new Uint8Array(new Array(1_000_000).fill(0).map(n=>parseInt(Math.random()*255)))
console.log(a)
f_measure_time('wasm manipulate_each_element_with_expression')
var a2 = a.slice();
var res = wasm.manipulate_each_element_with_expression(
    a2, 
    "{}*2.5"
);
console.log(a2)
f_measure_time()
f_measure_time('wasm native expression')
var a3 = a.slice();
for(let n in a3){
    a3[n] = a3[n]*2.5;
}
console.log(a3)

f_measure_time()

let a_n_u8 = new Uint8Array(new Array(n_its).fill(0).map(n=>parseInt(Math.random()*256)))
console.log(a_n_u8)



f_measure_time('native deno js min max')
var n_min = a_n_u8[0];
var n_max = a_n_u8[0];
for(let n_it = 0; n_it < a_n_u8.length; n_it+=1){
    let n = a_n_u8[n_it]
    if(n < n_min){
        n_min = n
    } 
    if(n > n_max){
        n_max = n
    }
}
console.log([n_min, n_max])
f_measure_time()


f_measure_time('wasm f_a_n_min_n_max')
var a_n_min_n_max = wasm.f_a_n_min_n_max(a_n_u8);
console.log(a_n_min_n_max)
f_measure_time()




f_measure_time('native deno js sum')
let n_sum = a_n_u8[0];
let n_avg = a_n_u8[0];
for(let n_it = 0; n_it < a_n_u8.length; n_it+=1){
    let n = a_n_u8[n_it]
    n_sum+=n;
}
n_avg = n_sum / a_n_u8.length
console.log([n_sum, n_avg])
f_measure_time()


f_measure_time('wasm sum')
let a_n_sum_n_avg = wasm.f_a_n_sum_n_avg(a_n_u8);
console.log(a_n_sum_n_avg)
f_measure_time()


// u64

let a_n_u64 = new BigUint64Array(new Array(n_its).fill(0).map(n=>BigInt(parseInt(Math.random()*(Math.pow(2,64)-1)))))
console.log(a_n_u64)

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


f_measure_time('wasm f_a_n_min_n_max tuple')
var a_n_min_n_max2 = wasm.f_a_n_min_n_max__u64__tuple(a_n_u64);
console.log(a_n_min_n_max2)
f_measure_time()

f_measure_time('wasm f_a_n_min_n_max struct')
var a_n_min_n_max3 = wasm.f_a_n_min_n_max__u64__struct(a_n_u64);
console.log(a_n_min_n_max3)
f_measure_time()


f_measure_time('wasm f_a_n_min_n_max array')
var a_n_min_n_max4 = wasm.f_a_n_min_n_max__u64__array(a_n_u64);
console.log(a_n_min_n_max4)
f_measure_time()

f_measure_time('wasm f_a_n_f64_normalized__from_a_n_u64__iermapcollect array')
var a_n_min_n_max4 = wasm.f_a_n_f64_normalized__from_a_n_u64__iermapcollect(a_n_u64, BigInt(233), BigInt(231234));
console.log(a_n_min_n_max4)
f_measure_time()


f_measure_time('wasm f_a_n_f64_normalized__from_a_n_u64__loop array')
var a_n_min_n_max4 = wasm.f_a_n_f64_normalized__from_a_n_u64__loop(a_n_u64, BigInt(233), BigInt(231234));
console.log(a_n_min_n_max4)
f_measure_time()



f_measure_time('wasm f_a_n_u64__sorted array')
var a_n_min_n_max4 = wasm.f_a_n_u64__sorted(a_n_u64);
console.log(a_n_min_n_max4)
f_measure_time()



//to get statistics on a 1d array representing a 2d array 
// eg. [r,g,b,a,r,g,b,a] 
// we can call 
let f_a_a_n_min_n_max__u64__array = function(
    a_n,
    n_channels
){

    let a_n_result = wasm.f_a_a_n_min_n_max__u64__array(
        a_n, 
        n_channels
    );

    let n_y = n_channels;
    let n_x = 2;// a_ 'n_min' , 'n_max' // depends on what the user wants
    

    for(let n = 1; n<n_channels; n+=1){
        let a_n_res = new Array(1).fill(0);
        a_n_res_old[0] = (a_n_res)
        a_n_res_old = a_n_res
    }
    console.log(a_n_res_old)

}

