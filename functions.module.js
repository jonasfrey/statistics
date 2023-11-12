import { ensureDir } from "https://deno.land/std@0.206.0/fs/mod.ts";

let a_n_bytes = [
    64_000,
    6_400_000,
    // 25_600_000,
    32_000_000, 
    // 128_000_000 // heap overflow
];
let f_s_path_file__from_n_bytes = function(n_bytes){
    let s_path_folder = './random_test_data'

    let s_path_file = import.meta.url.split('//').pop().split('/').slice(0,-1).join('/');
    ensureDir(s_path_folder)
    
    let s_name_file = `a_n_u8__len_${n_bytes}`
    return `${s_path_folder}/${s_name_file}`
}

let f_write_random_test_data = async function(
){




    for(let n_bytes of a_n_bytes){
        let s_path_file = f_s_path_file__from_n_bytes(n_bytes)
        await Deno.writeFile(
            s_path_file, 
            new Uint8Array(
                new Array(n_bytes)
                 .fill(0)
                 .map(n=>parseInt(Math.random()*255))
            )
        )
    }
}

class O_test_array{
    constructor(
        s_type,
        n_len_array,
        a_n_t
    ){
        this.s_type = s_type 
        this.n_len_array = n_len_array 
        this.a_n_t = a_n_t 
    }
}
class O_testdata{
    constructor(
        a_o_test_array
    ){
        this.a_o_test_array = a_o_test_array
    }
}
let f_o_testdata = async function(){
    let o_testdata = new O_testdata([]);
    let o_s_type_a_n_t={}
    let o_s_type_s_function = {
        'u8': Uint8Array, 
        'u16': Uint16Array, 
        'u32': Uint32Array, 
        'u64': BigUint64Array,
        'i8': Int8Array, 
        'i16': Int16Array, 
        'i32': Int32Array, 
        'i64': BigInt64Array,
        'f32': Float32Array,
        'f64': Float64Array
    }
    let o_s_nbytes_a_n_u8 = {}
    for(let n_bytes of a_n_bytes){
        let s_path_file =f_s_path_file__from_n_bytes(n_bytes)
        o_s_nbytes_a_n_u8[`n_${n_bytes}`] = await Deno.readFile(s_path_file)
    }

    for(let s_type in o_s_type_s_function){
        let O_typed_array = o_s_type_s_function[s_type];
        for(let s_prop in o_s_nbytes_a_n_u8){
            // console.log(s_prop)
            let n_bytes = parseInt((s_prop).split('_').pop());
            let n_bits = parseInt((s_type).slice(1));
            
            // console.log(n_bits)
            o_testdata.a_o_test_array.push(
                new O_test_array(
                    s_type, 
                    n_bytes/n_bits,
                    new O_typed_array(o_s_nbytes_a_n_u8[s_prop].buffer)
                )
            )
        }
    }
    return o_testdata
}

export {
    f_write_random_test_data, 
    f_o_testdata
}