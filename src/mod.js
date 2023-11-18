
import {
    f_a_a_v__combinations
} from "https://deno.land/x/handyhelpers@1.1/mod.js"
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
        s_js_s_rs__singlevaluereturn, 
        a_o_resulting_value_additional
    ){
        this.s_prop = s_prop,
        this.a_s_prop_depending_on = a_s_prop_depending_on
        this.s_js_s_type = s_js_s_type, 
        this.s_js_s_rs__initvalue = s_js_s_rs__initvalue
        this.s_js_s_rs__inside_fold = s_js_s_rs__inside_fold
        this.s_js_s_rs__before_assignment_initvalue = s_js_s_rs__before_assignment_initvalue
        this.s_js_s_rs__singlevaluereturn = s_js_s_rs__singlevaluereturn
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
        '`let n_min = *a_n.iter().min().unwrap();`',
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
        '`let n_max = *a_n.iter().max().unwrap();`',
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
            //'`let n_sum: ${s_type_max} = a_n.iter().sum::<${s_type_max}>();',
            // '`let n_sum: ${s_type_max} = a_n.iter().map(|&val| val as ${s_type_max}).sum();',
            '`let n_sum: ${s_char_sign}64 = a_n.iter().map(|&val| val as ${s_char_sign}64).sum();',
            'let n_mean = (n_sum / a_n.len() as ${s_char_sign}64) as f64;`',
        ].join('\n'),
        [
            new O_resulting_value(
                'n_mean',
                '`f64`',
                '`${f_s_accessor_result("n_mean")}`' // n_min: o_res.n_min
            ),
            // new O_resulting_value(
            //     'n_sum',
            //     '`f64`',
            //     '`${f_s_accessor_result("n_mean")} * n_len`' // n_min: o_res.n_min
            // )
        ]
    ),
    new O_acc_property(
        'n_variance',
        ['n_mean'],
        '`f64`',
        '0.0',
        '',
        '`${f_s_accessor_accumulator("n_variance")} = ${f_s_accessor_accumulator("n_mean")} + (v_in_fold as f64).powi(2);`',
        ``,
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
    let s_char_sign = s_type[0];
    let s_type_max = 'u128'
    if(s_type.includes('i')){
        s_type_max = 'i128'
    }
    if(s_type.includes('f')){
        s_type_max = 'f64'
    }

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
                                    return [

                                        `${o_resulting_value.s_prop}_nor:`,
                                        `(${eval(o_resulting_value.s_js_s_rs__assignment_in_struct_return)})`,
                                        `as f64 /`,
                                        `(${s_type2}::MAX${(s_char_sign == 'i')?' as i128' : ''}`,
                                        `-${s_type2}::MIN${(s_char_sign == 'i')?' as i128 + 1' : ''})`, 
                                        `as f64`
                                    ].join('')
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

    ${(()=>{
        if(
            n_dimensions == 1
            && a_o_acc_property.length == 1
            && ((
                a_o_acc_property[0].s_prop == 'n_mean'
                && 
                ['u8', 'i8', 'u16', 'i16'].includes(s_type)
            ) || 
            (
                ['n_min', 'n_max'].includes(a_o_acc_property[0].s_prop)
                && 
                s_type.indexOf('f') == -1
            ))
            ){
            
            return `
            ${eval(a_o_acc_property[0].s_js_s_rs__singlevaluereturn)}
            let v_result = [${a_o_acc_property[0].s_prop} as ${eval(a_o_acc_property[0].s_js_s_type)}];
            `
        }else{
            return `
            ${a_o_acc_property.map(o=>{
                return o.s_js_s_rs__before_assignment_initvalue
            }).filter(v=>v.trim()!='').join(';')}
        
            ${(b_all_same_type) ? `${s_rs_initvalue_array}` : `${s_js_s_rs__initvalue_struct}`}
            ${s_js_s_rs__fold_1d}
            `
        }
    })()}

    ${s_rs_returnvalue_1d}
    a_o${(n_channels == 1)? '.remove(0)': ''}
}
`
return s_js_s_rs__all

}

export {
    O_resulting_value,
    O_acc_property,
    a_o_acc_property,
    a_a_o_acc_property__combinations_with_required,
    f_rs__function,
}