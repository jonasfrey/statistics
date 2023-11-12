
class O_fold_item{
    constructor(
        s_prop,
        s_init, 
        s_fold_code, 
        s_struct_assignement_expression
    ){
        this.s_prop = s_prop,
        this.s_init = s_init, 
        this.s_fold_code = s_fold_code, 
        this.s_struct_assignement_expression = s_struct_assignement_expression
    }
}
let a_o_fold_item = [
    
    new O_fold_item(
        'n_min',
        'a_n[0]',
        '${s_prop}.min(val)',
    ),
    new O_fold_item(
        'n_max',
        'a_n[0]',
        '${s_prop}.max(val)',
    ),
    // this is unfortunately more complicated than i thought....
    // new O_fold_item(
    //     'n_sum',
    //     '0u128',
    //     '${s_prop} + val',
    //     // '${s_prop}.wraping_add(val)', // i dont think we need this because n_sum is u128
    // ),
    // new O_fold_item(
    //     'n_avg',
    //     '0',
    //     '${s_prop} + val', 
    //     'a_n__result[${n_idx_result}]/a_n.len() as ${s_type}'
    // ),
]

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

pub fn f_a_n_min_n_avg__from_a_n_u64(a_n: &[u64]) -> Vec<u64> {
    let init = (a_n[0],a_n[0]);

    let result = a_n_u64.iter().fold(init, |(min, max), &val| {
        (
            min.min(val),
            max.max(val),
        )
    });

    vec![result[0], result[1]]
}