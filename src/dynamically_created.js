
import * as o_mod_toml from "https://deno.land/std@0.206.0/toml/mod.ts";

import {
    f_o_command 
} from "https://deno.land/x/o_command@0.9/mod.js"

import {
    a_a_o_acc_property__combinations_with_required, 
    f_rs__function,
} from "./mod.js"
let s_path_folder_current_file = import.meta.url
    .split('://')
    .pop()
    ?.split('/')
    .slice(0, -1)
    .join('/')
// console.log(s_path_folder_current_file);
// Deno.exit();
let o_cargo_dot_toml = 
o_mod_toml.parse(await Deno.readTextFile(`${s_path_folder_current_file}/../Cargo.toml`));


let s_rust_code = a_a_o_acc_property__combinations_with_required.map(
    a_o_acc_property =>{
        return ['u8','i8','u16','i16','u32','i32','u64','i64','f32','f64'].map(
            s_type =>{
                return f_rs__function(
                    1,
                    a_o_acc_property,
                    s_type
                )
            }
        )
    }
).flat(1).join('\n')
let s_path_file_out = `${s_path_folder_current_file}/dynamically_created.rs`
await Deno.writeTextFile(
    `${s_path_file_out}`, 
    `
    // all code in this file was automatically generated! 
    use wasm_bindgen::prelude::*;

    ${s_rust_code}

    `
)
// create proxy functions 
let s_js_code = 
`
//this code was automatically generated
import * as o_wasm from "./../pkg/${o_cargo_dot_toml.package.name}.js";
await o_wasm.default();

${
    a_a_o_acc_property__combinations_with_required.map(
        a_o_acc_property__combinations_with_required =>{
            let s_name_function = `f_o_${a_o_acc_property__combinations_with_required.map(o=>{
                return o.s_prop
            }).join('_')}`
            let o_s_name_typed_array_s_type = {
                'Int8Array': 'i8' ,
                'Uint8Array': 'u8' ,
                'Uint8ClampedArray': 'u8' ,
                'Int16Array': 'i16' ,
                'Uint16Array': 'u16' ,
                'Int32Array': 'i32' ,
                'Uint32Array': 'u32' ,
                'Float32Array': 'f32' ,
                'Float64Array': 'f64' ,
                'BigInt64Array': 'i64' ,
                'BigUint64Array': 'u64' 
            }
            return `
                let ${s_name_function} = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    ${Object.keys(o_s_name_typed_array_s_type).map(
                        s_name_typed_array =>{
                            let s_type = o_s_name_typed_array_s_type[s_name_typed_array];
                            return `v_s_type = (a_v instanceof ${s_name_typed_array}) ? '${s_type}': v_s_type;`
                        }
                    ).join('\n')}
    
                    if(!v_s_type){
                        throw Error(\`array type not allowed, please pass a typed array an instance of one of the following arrays \$\{Object.values(o_s_type_s_name_typed_array)\}\`)
                    }
                    let s_name_function = \`${s_name_function}_for_\$\{v_s_type\}\`
                    let o = o_wasm[s_name_function](a_v);
                    const a_s_prop = Object.getOwnPropertyNames(Object.getPrototypeOf(o));
    
                    let o2 = Object.assign(
                        {}, 
                        
                        ...a_s_prop.map((s_prop)=>{
                            return {
                                [s_prop]: o[s_prop]
                            }
                        })
                    )
                    return o2
    
                        
                }
                export {
                    ${s_name_function}
                }
            `
        }
    ).join('\n')
    
}
`


await Deno.writeTextFile(
    `${s_path_folder_current_file}/functions.module.js`, 
    s_js_code
)

let o = await f_o_command(`cargo fmt -- ${s_path_file_out}`);

//example 
/*

// Define the struct
#[wasm_bindgen]
pub struct O_n_min_n_avg__for_u64 {
    pub n_min: u64,
    pub n_avg: u64,
}

#[wasm_bindgen]
pub fn f_a_n_min_n_avg__from_a_n_u64(a_n: &[u64]) -> Vec<u64> {
    let init = (u64::MAX,);

    let result = a_n_u64.iter().fold(init, |(min, n_avg), &val| {
        (
            min.min(val),
            max.max(val),
            sum.wrapping_add(val),
            count + 1,
        )
    });

    vec![0]
}
*/