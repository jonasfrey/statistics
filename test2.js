// let a_n_u8__wasm = await Deno.readFile('./pkg/wasm_speed_test_bg.wasm')
import * as wasm from "./pkg/wasm_speed_test.js";
import {
    f_measure_time
} from "https://deno.land/x/date_functions@1.3/mod.js"

await wasm.default();
let a_n_u64 = new BigUint64Array(
    new Array(4096*4096).fill(BigInt(1234))
)
f_measure_time('wasm f_a_n_f64_normalized__from_a_n_u64__loop array')
var a_n_min_n_max4 = wasm.f_a_n_f64_normalized__from_a_n_u64__loop(a_n_u64, BigInt(0), BigInt(18_000));
console.log(a_n_min_n_max4)
f_measure_time()

