import {
    f_a_a_v__combinations
} from "https://deno.land/x/handyhelpers@1.1/mod.js"

class O_resulting_value { 
    constructor(
        s_prop, 
        s_type, 
        s_rs__assignment_in_struct_return,
        s_rs__before_assignment_in_struct
    ){
        this.s_prop = s_prop, 
        this.s_type = s_type, 
        this.s_rs__assignment_in_struct_return = s_rs__assignment_in_struct_return
        this.s_rs__before_assignment_in_struct = s_rs__before_assignment_in_struct
    }
}
class O_acc_property{
    constructor(
        s_prop, 
        a_s_prop_depending_on,
        s_type_lowest_init, 
        // lowest possible init type
        s_rs__initvalue,
        s_rs__before_assignment_initvalue,
        s_rs__inside_fold, 
        a_o_resulting_value_additional
    ){
        this.s_prop = s_prop,
        this.a_s_prop_depending_on = a_s_prop_depending_on
        this.s_type_lowest_init = s_type_lowest_init, 
        this.s_rs__initvalue = s_rs__initvalue
        this.s_rs__inside_fold = s_rs__inside_fold
        this.s_rs__before_assignment_initvalue = s_rs__before_assignment_initvalue
        this.a_o_resulting_value_additional = a_o_resulting_value_additional
    }
}
let a_o_acc_property = [
    
    new O_acc_property(
        'n_min',
        [],
        'u8',
        'a_n[0]',
        '',
        '`${s_accessor} = ${s_accessor}.min(v_in_fold);`',
        [
            new O_resulting_value(
                'n_min_nor',
                'f64',
                '`${s_accessor} as f64 / u8::MAX`' // n_min: o_res.n_min
            )
        ]
    ),
    new O_acc_property(
        'n_max',
        [],
        'u8',
        'a_n[0]',
        '',
        '`${s_accessor} = ${s_accessor}.max(v_in_fold);`',
        [
            new O_resulting_value(
                'n_max_nor',
                'f64',
                '`${s_accessor} as f64 / u8::MAX`' // n_min: o_res.n_min
            )
        ]
    ),
    new O_acc_property(
        'n_mean',
        [],

        'f64',
        '0.0',
        'let n_len = a_n.len() as f64;',
        '`${s_accessor} = ${s_accessor}+(v_in_fold as f64 / n_len);`',
        [
            new O_resulting_value(
                'n_sum',
                'f64',
                '`${s_accessor} * n_len`' // n_min: o_res.n_min
            )
        ]
    ),
    new O_acc_property(
        'n_variance',
        ['n_mean'],
        'f64',
        '0.0',
        '',
        '`${s_accessor} = ${s_accessor} + (v_in_fold as f64).powi(2);`',
        [
            new O_resulting_value(
                'n_variance',
                'f64',
                '`(${s_accessor} / n_len) - n_mean.powi(2)`' // n_min: o_res.n_min
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
    s_for_type = 'u8'
){

    let n_channels = (n_dimensions == 1) ? 1 : 2;
    let o_acc_property__first = a_o_acc_property[0];

    let a_o_acc_property_same_type = a_o_acc_property.filter(
        o=>{
            return o.s_type_lowest_init == o_acc_property__first.s_type_lowest_init
        }
    );
    let b_all_same_type = a_o_acc_property.length == a_o_acc_property_same_type.length;
    // we now write the function for an input array with the simplest data type 'u8'
    // later this can be extended by using code_autoextender
    let s_name_struct_accumulator =  `O_${a_o_acc_property.map(o=>o.s_prop).join('_')}_accumulator_for_${s_for_type}`
    let s_rs_code_struct_accumulator = `
    #[wasm_bindgen]
    pub struct ${s_name_struct_accumulator} {
        ${a_o_acc_property.map(o_acc_property=>
                {return `${o_acc_property.s_prop}: ${o_acc_property.s_type_lowest_init}`}
            ).join(',\n')}
    }
    `
    let s_name_struct_return =  `O_${a_o_acc_property.map(o=>o.s_prop).join('_')}_return_for_${s_for_type}`
    let s_rs_code_struct_return = `
    #[wasm_bindgen]
    pub struct ${s_name_struct_return} {
        ${a_o_acc_property.map(o_acc_property=>
            o_acc_property.a_o_resulting_value_additional.map(o_resulting_value=>{
                {return `${o_resulting_value.s_prop}: ${o_resulting_value.s_type}`}
            })
            ).flat(2).join(',\n')}
    }
`
let s_rs__initvalue_struct = `
let v_init: ${s_name_struct_accumulator} = ${s_name_struct_accumulator} {
    ${a_o_acc_property.map(o_acc_property=>
        {
            return `${o_acc_property.s_prop}:${o_acc_property.s_rs__initvalue}`
        }
    ).join(',\n')}
};
`
let s_initvec_content = `
${a_o_acc_property.map(o_acc_property=>
    {
        return `${o_acc_property.s_rs__initvalue}`
    }
).join(',\n')}
`
let s_rs_initvalue_array = `
let v_init = std::iter::repeat([
${s_initvec_content}
])
.take(${(n_dimensions > 1) ? 'n_channels': 1})
.flatten()
.collect::<Vec<${o_acc_property__first.s_type_lowest_init}>>();
`
if(n_dimensions == 1){
    s_rs_initvalue_array = `
    let v_init =  vec![
        ${s_initvec_content}
    ];
    `
}
let s_rs__fold_1d = `
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            ${a_o_acc_property.map((o_acc_property,n_idx)=>
                {
                    let s_accessor = `v_acc[${n_idx}]`
                    if(!b_all_same_type){
                        s_accessor = `v_acc.${o_acc_property.s_prop}`
                    }
                    return eval(o_acc_property.s_rs__inside_fold)
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
                o_acc_property.a_o_resulting_value_additional.map((o_resulting_value, n_idx)=>{
                    let s_accessor = `v_result[${n_idx}]`
                    if(!b_all_same_type){
                        s_accessor = `v_result.${o_acc_property.s_prop}`
                    }
                    return `${o_resulting_value.s_prop}:${eval(o_resulting_value.s_rs__assignment_in_struct_return)}`
                })
                ).flat(2).join(',\n')}

        }
    );
}
`;

let s_name_function = `f_${(n_dimensions > 1)?'_a_': ''}_o_${a_o_acc_property.map(o=>o.s_prop).join('_')}`
let s_rs__all = `
${s_rs_code_struct_return}
${(b_all_same_type) ? '': s_rs_code_struct_accumulator}
#[wasm_bindgen]
pub fn ${s_name_function}(
    ${[
        `a_n: &[${s_for_type}]`,
        ...`${(n_dimensions > 1) ? 'n_channels: usize': []}`
    ].join(',')}
) -> ${(n_dimensions > 1)? `Vec<${s_name_struct_return}>`: `${s_name_struct_return}`}{

    ${a_o_acc_property.map(o=>{
        return o.s_rs__before_assignment_initvalue
    }).filter(v=>v.trim()!='').join(';')}

    ${(b_all_same_type) ? `${s_rs_initvalue_array}` : `${s_rs__initvalue_struct}`}
    ${s_rs__fold_1d}
    ${s_rs_returnvalue_1d}
    a_o${(n_channels == 1)? '[0]': ''}
}
`
return s_rs__all

}
let s_rust_code = a_a_o_acc_property__combinations_with_required.map(
    a_o_acc_property =>{
        return f_rs__function(
            1,
            a_o_acc_property,
            'u8'
        )
    }
).join('\n')
await Deno.writeTextFile(
    'dynamically_created.rs', 
    `
    // all code in this file was automatically generated! 
    ${s_rust_code}
    `
)
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