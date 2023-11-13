import {
    f_a_a_v__combinations
} from "https://deno.land/x/handyhelpers@1.1/mod.js"
import * as o_mod_toml from "https://deno.land/std@0.206.0/toml/mod.ts";

import {
    f_o_command 
} from "https://deno.land/x/o_command@0.9/mod.js"

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

class O_resulting_value { 
    constructor(
        s_prop, 
        s_js_s_type, 
        s_js_s_rs__assignment_in_struct_return,
        s_js_s_rs__before_assignment_in_struct
    ){
        this.s_prop = s_prop, 
        this.s_js_s_type = s_js_s_type, 
        this.s_js_s_rs__assignment_in_struct_return = s_js_s_rs__assignment_in_struct_return
        this.s_js_s_rs__before_assignment_in_struct = s_js_s_rs__before_assignment_in_struct
    }
}
class O_acc_property{
    constructor(
        s_prop, 
        a_s_prop_depending_on,
        s_js_s_type, 
        // lowest possible init type
        s_js_s_rs__initvalue,
        s_js_s_rs__before_assignment_initvalue,
        s_js_s_rs__inside_fold, 
        a_o_resulting_value_additional
    ){
        this.s_prop = s_prop,
        this.a_s_prop_depending_on = a_s_prop_depending_on
        this.s_js_s_type = s_js_s_type, 
        this.s_js_s_rs__initvalue = s_js_s_rs__initvalue
        this.s_js_s_rs__inside_fold = s_js_s_rs__inside_fold
        this.s_js_s_rs__before_assignment_initvalue = s_js_s_rs__before_assignment_initvalue
        this.a_o_resulting_value_additional = a_o_resulting_value_additional
    }
}
let a_o_acc_property = [
    
    new O_acc_property(
        'n_min',
        [],
        '`${s_type}`',
        'a_n[0]',
        '',
        '`${f_s_accessor_accumulator("n_min")} = ${f_s_accessor_accumulator("n_min")}.min(v_in_fold);`',
        [
            new O_resulting_value(
                'n_min',
                '`${s_type}`',
                '`${f_s_accessor_result("n_min")}`',
            )
        ]
    ),
    new O_acc_property(
        'n_max',
        [],
        '`${s_type}`',
        'a_n[0]',
        '',
        '`${f_s_accessor_accumulator("n_max")} = ${f_s_accessor_accumulator("n_max")}.max(v_in_fold);`',
        [
            new O_resulting_value(
                'n_max',
                '`${s_type}`',
                '`${f_s_accessor_result("n_max")}`', 
                ),
        ]
    ),
    new O_acc_property(
        'n_mean',
        [],

        '`f64`',
        '0.0',
        'let n_len = a_n.len() as f64;',
        '`${f_s_accessor_accumulator("n_mean")} = ${f_s_accessor_accumulator("n_mean")}+(v_in_fold as f64 / n_len);`',
        [
            new O_resulting_value(
                'n_sum',
                '`f64`',
                '`${f_s_accessor_result("n_mean")} * n_len`' // n_min: o_res.n_min
            )
        ]
    ),
    new O_acc_property(
        'n_variance',
        ['n_mean'],
        '`f64`',
        '0.0',
        '',
        '`${f_s_accessor_accumulator("n_mean")} = ${f_s_accessor_accumulator("n_mean")} + (v_in_fold as f64).powi(2);`',
        [
            new O_resulting_value(
                'n_variance',
                '`f64`',
                '`(${f_s_accessor_result("n_variance")} / n_len) - ${f_s_accessor_result("n_mean")}.powi(2)`' // n_min: o_res.n_min
            )
        ]
    ),
]

let a_a_o_acc_property__combinations = f_a_a_v__combinations(
    a_o_acc_property
);
let a_a_o_acc_property__combinations_with_required = a_a_o_acc_property__combinations.map(
    a_o=>{
        for(let o of a_o){
            // a_a_o_acc_property__combinations already contains all possible combinations
            // if a o_acc_property is depending on another and that other one is not present in the current combination 
            // this combination is impossible therefore return false
            if(o.a_s_prop_depending_on.length == 0){
                continue
            }
            let a_o_filtered = a_o.filter(
                o2=>o.a_s_prop_depending_on.includes(o2.s_prop)
            )
            if(a_o_filtered.length != o.a_s_prop_depending_on.length){
                return false;
            }

        }
        return a_o
    }
).filter(v=>v)

console.log(a_a_o_acc_property__combinations_with_required)
console.log(
    a_a_o_acc_property__combinations_with_required.map(a_o=>{
        console.log(`O_${a_o.map(o=>o.s_prop).join('_')}`)
    })
)
let f_rs__function = function(
    n_dimensions = 1, 
    a_o_acc_property, 
    s_type = 'u8'
){

    let f_s_accessor_result = function(s_prop){
        let o_acc_property = a_o_acc_property.find(o=>o.s_prop == s_prop);
        let s = `v_result[${a_o_acc_property.indexOf(o_acc_property)}]`
        if(!b_all_same_type){
            s = `v_result.${o_acc_property.s_prop}`
        }
        return s
    }
    let f_s_accessor_accumulator = function(s_prop){
        let o_acc_property = a_o_acc_property.find(o=>o.s_prop == s_prop);
        let s = `v_acc[${a_o_acc_property.indexOf(o_acc_property)}]`
        if(!b_all_same_type){
            s = `v_acc.${o_acc_property.s_prop}`
        }
        return s
    }

    let n_channels = (n_dimensions == 1) ? 1 : 2;
    let o_acc_property__first = a_o_acc_property[0];

    let a_o_acc_property_same_type = a_o_acc_property.filter(
        o=>{
            return eval(o.s_js_s_type) == eval(o_acc_property__first.s_js_s_type)
        }
    );
    let b_all_same_type = a_o_acc_property.length == a_o_acc_property_same_type.length;
    // we now write the function for an input array with the simplest data type 'u8'
    // later this can be extended by using code_autoextender
    let s_name_struct_accumulator =  `O_${a_o_acc_property.map(o=>o.s_prop).join('_')}_accumulator_for_${s_type}`
    let s_rs_code_struct_accumulator = `
    #[wasm_bindgen]
    pub struct ${s_name_struct_accumulator} {
        ${a_o_acc_property.map(o_acc_property=>
                {
                    return `pub ${o_acc_property.s_prop}: ${eval(o_acc_property.s_js_s_type)}`
                }
            ).join(',\n')}
    }
    `
    let s_name_struct_return =  `O_${a_o_acc_property.map(o=>o.s_prop).join('_')}_return_for_${s_type}`
    let s_rs_code_struct_return = `
    #[wasm_bindgen]
    pub struct ${s_name_struct_return} {
        ${a_o_acc_property.map(o_acc_property=>
            {
                return [
                    ...o_acc_property.a_o_resulting_value_additional.map(o_resulting_value=>{
                        {
                            return `pub ${o_resulting_value.s_prop}: ${eval(o_resulting_value.s_js_s_type)}`
                        }
                    }),
                    ...o_acc_property.a_o_resulting_value_additional.map(o_resulting_value=>{

                        let s_type2 = eval(o_resulting_value.s_js_s_type)
                        if(
                            s_type2.includes('u')
                            ||
                            s_type2.includes('i')
                            ){
                            console.log(o_resulting_value)
                            // we can also ad a {s_prop}_nor which is the normalized value 
                            // (value / range) => value / (s_type::MAX - s_type::MIN)
                            {return `pub ${o_resulting_value.s_prop}_nor: f64`}
                        }
                        return false
                    }).filter(v=>v)

                ]
            }
            ).flat(2).join(',\n')}
    }
`
let s_js_s_rs__initvalue_struct = `
let v_init: ${s_name_struct_accumulator} = ${s_name_struct_accumulator} {
    ${a_o_acc_property.map(o_acc_property=>
        {
            return `${o_acc_property.s_prop}:${o_acc_property.s_js_s_rs__initvalue}`
        }
    ).join(',\n')}
};
`
let s_initvec_content = `
${a_o_acc_property.map(o_acc_property=>
    {
        return `${o_acc_property.s_js_s_rs__initvalue}`
    }
).join(',\n')}
`
let s_rs_initvalue_array = `
let v_init = std::iter::repeat([
${s_initvec_content}
])
.take(${(n_dimensions > 1) ? 'n_channels': 1})
.flatten()
.collect::<Vec<${eval(o_acc_property__first.s_js_s_type)}>>();
`
if(n_dimensions == 1){
    s_rs_initvalue_array = `
    let v_init =  vec![
        ${s_initvec_content}
    ];
    `
}

let s_js_s_rs__fold_1d = `
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            ${a_o_acc_property.map((o_acc_property,n_idx)=>
                {
                    
                    return eval(o_acc_property.s_js_s_rs__inside_fold)
                }
            ).join('\n')}
            v_acc
        }
);
`
let s_rs_returnvalue_1d = `
let mut a_o= Vec::new();
for n_channel in 0..${(n_dimensions > 1) ? 'n_channels': 1}{
    a_o.push(
        ${s_name_struct_return}{
            ${a_o_acc_property.map(o_acc_property=>
                {
                    return [
                        ...o_acc_property.a_o_resulting_value_additional.map((o_resulting_value, n_idx)=>{
                            return `${o_resulting_value.s_prop}:${eval(o_resulting_value.s_js_s_rs__assignment_in_struct_return)}`
                        }),
                        ...o_acc_property.a_o_resulting_value_additional.map(o_resulting_value=>{
                            
                            let s_type2 = eval(o_resulting_value.s_js_s_type)
                            if(
                                s_type2.includes('u')
                                ||
                                s_type2.includes('i')
                            ){
                                // we can also ad a {s_prop}_nor which is the normalized value 
                                // (value / range) => value / (s_type::MAX - s_type::MIN)
                                {
                                    return `${o_resulting_value.s_prop}_nor:(${eval(o_resulting_value.s_js_s_rs__assignment_in_struct_return)}) as f64 / (${s_type2}::MAX-${s_type2}::MIN) as f64`
                                }
                            }
                            return false
                        }).filter(v=>v)
                    ]
                }
            ).flat(2).join(',\n')}
        }
    );
}
`;

let s_name_function = `f_${(n_dimensions > 1)?'a_': ''}o_${a_o_acc_property.map(o=>o.s_prop).join('_')}_for_${s_type}`
let s_js_s_rs__all = `
${s_rs_code_struct_return}
${(b_all_same_type) ? '': s_rs_code_struct_accumulator}
#[wasm_bindgen]
pub fn ${s_name_function}(
    ${[
        `a_n: &[${s_type}]`,
        ...`${(n_dimensions > 1) ? 'n_channels: usize': []}`
    ].join(',')}
) -> ${(n_dimensions > 1)? `Vec<${s_name_struct_return}>`: `${s_name_struct_return}`}{

    ${a_o_acc_property.map(o=>{
        return o.s_js_s_rs__before_assignment_initvalue
    }).filter(v=>v.trim()!='').join(';')}

    ${(b_all_same_type) ? `${s_rs_initvalue_array}` : `${s_js_s_rs__initvalue_struct}`}
    ${s_js_s_rs__fold_1d}
    ${s_rs_returnvalue_1d}
    a_o${(n_channels == 1)? '.remove(0)': ''}
}
`
return s_js_s_rs__all

}
let s_rust_code = a_a_o_acc_property__combinations_with_required.map(
    a_o_acc_property =>{
        return ['u8','u16', 'u32', 'u64', 'f32', 'f64'].map(
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