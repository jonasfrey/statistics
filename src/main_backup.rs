use wasm_bindgen::prelude::*;
// use rand::Rng; // rand crate is required
use evalexpr::*;


//code_autoextender_start
////    return [ 'u16', 'u32', 'u64', 'f32', 'f64'].map(
////     (s_type) =>{
////         return s_code.replaceAll('u8', s_type)
////    }).join('\n')
#[wasm_bindgen]
pub struct O_n_mean_n_sum__for_u8 {
    pub n_mean: f64,
    // pub: f64,
    pub n_sum: f64,
}
#[wasm_bindgen]
pub fn f_o_n_mean_n_sum__for_u8(a_n: &[u8]) -> O_n_mean_n_sum__for_u8 {

    let n_len = a_n.len() as f64;
    let a_n_t__init = [0.0];
    let a_n_t__result = a_n.iter().fold(a_n_t__init, |mut a_n_acc, &val| {
        a_n_acc[0] += (val as f64 / n_len);
        a_n_acc
    });

    let n_mean = a_n_t__result[0];//as float
    // let = n_mean / (u8::MAX as f64); // float has no normalization so we cannot calculate it
    let n_sum = n_mean * n_len;
  
    O_n_mean_n_sum__for_u8{
        n_mean,
        //,
        n_sum
    }
}
#[wasm_bindgen]
pub fn f_a_o_n_mean_n_sum__for_u8(
    a_n: &[u8],
    n_channels: usize
) -> Vec<O_n_mean_n_sum__for_u8> {


    let n_len = a_n.len() as f64 / n_channels as f64;
    let a_n_t__init = vec![0.0; n_channels];
    let a_n_t__result = a_n.iter()
    .enumerate()
    .fold(
        a_n_t__init,
        |mut a_n_acc, (n_idx,&val)|
        {
            let n_idx_channel = n_idx % n_channels;

            a_n_acc[n_idx_channel+0] += (val as f64 / n_len);
            a_n_acc
        }
    );

    let mut a_o= Vec::new();
    for n_channel in 0..n_channels{
        let n_mean = a_n_t__result[n_channel+0];//as float
        // let n_mean_nor = n_mean / (u8::MAX as f64);
        let n_sum = n_mean * n_len;
        a_o.push(
            O_n_mean_n_sum__for_u8{
                n_mean,
                // n_mean_nor,
                n_sum
            }
        );
    }

    a_o

}
//code_autoextender_end


//code_autoextender_start
////    return [ 'u16', 'u32', 'u64', 'i8', 'i16', 'i32', 'i64'].map(
////     (s_type) =>{
////         return s_code.replaceAll('u8', s_type)
////    }).join('\n')
#[wasm_bindgen]
pub fn f_n_min__for_u8(a_n: &[u8]) -> u8 {
    let a_n__init = [u8::MAX];
    let a_n__result = a_n.iter().fold(a_n__init, |mut a_n_acc, &val| {
        a_n_acc[0] = a_n_acc[0].min(val);
        a_n_acc
    });
    a_n__result[0]
}
#[wasm_bindgen]
pub fn f_a_n_min__for_u8(
    a_n: &[u8],
    n_channels: usize
) -> Vec<u8> {

    let n_len = a_n.len() as f64 / n_channels as f64;
    let a_n_t__init = vec![u8::MAX; n_channels];
    let a_n_t__result = a_n.iter()
    .enumerate()
    .fold(
        a_n_t__init,
        |mut a_n_acc, (n_idx,&val)|
        {
            let n_idx_channel = n_idx % n_channels;
            a_n_acc[n_idx_channel+0] = a_n_acc[n_idx_channel+0].min(val);
            a_n_acc
        }
    );

    let mut a_n= Vec::new();
    for n_channel in 0..n_channels{
        a_n.push(a_n_t__result[n_channel+0]);
    }
    a_n
}

#[wasm_bindgen]
pub fn f_n_max__for_u8(a_n: &[u8]) -> u8 {
    let a_n__init = [u8::MIN];
    let a_n__result = a_n.iter().fold(a_n__init, |mut a_n_acc, &val| {
        a_n_acc[0] = a_n_acc[0].max(val);
        a_n_acc
    });
    a_n__result[0]
}

#[wasm_bindgen]
pub fn f_a_n_min_n_max__for_u8(a_n: &[u8]) -> Vec<u8> {
    let a_n__init = [u8::MAX, u8::MIN];
    let a_n__result = a_n.iter().fold(a_n__init, |mut a_n_acc, &val| {
        a_n_acc[0] = a_n_acc[0].min(val);
        a_n_acc[1] = a_n_acc[1].max(val);
        a_n_acc
    });

    return a_n__result.to_vec()
}
#[wasm_bindgen]
pub struct O_n_min_n_max__for_u8 {
    pub n_min: u8,
    pub n_max: u8,
}

#[wasm_bindgen]
pub fn f_a_o_n_min_n_max__for_u8(
    a_n: &[u8],
    n_channels: usize
) -> Vec<O_n_min_n_max__for_u8> {

    let n_len = a_n.len() as f64 / n_channels as f64;
    let a_n_t__init = std::iter::repeat([u8::MAX, u8::MIN])
        .take(n_channels)
        .flatten()
        .collect::<Vec<u8>>();
    let a_n_t__result = a_n.iter()
    .enumerate()
    .fold(
        a_n_t__init,
        |mut a_n_acc, (n_idx,&val)|
        {
            let n_idx_channel = n_idx % n_channels;
            a_n_acc[n_idx_channel+0] = a_n_acc[n_idx_channel+0].min(val);
            a_n_acc[n_idx_channel+1] = a_n_acc[n_idx_channel+1].max(val);
            a_n_acc
        }
    );

    let mut a_o= Vec::new();
    for n_channel in 0..n_channels{
        a_o.push(
            O_n_min_n_max__for_u8{
                n_min: a_n_t__result[n_channel+0],
                n_max: a_n_t__result[n_channel+1],
            }
        );
    }
    a_o
}

// #[wasm_bindgen]
// pub fn f_a_n_mean_n_sum_n_min_n_max__for_u8_tuple(a_n: &[u8]) -> Vec<f64> {
    // the version with the struct is faster than the version with the tuple !!!
//     let n_len = a_n.len() as f64;
//     let a_n_t__result = a_n.iter().fold(
//         (0.0,u8::MAX, u8::MIN)
//         , |(n_mean, n_min, n_max), &val| {
//             (
//                 n_mean+(val as f64 / n_len),
//                 n_min.min(val),
//                 n_max.max(val)
//             )

//     });

//     let n_mean = a_n_t__result.0;//as float
//     // let = n_mean / (u8::MAX as f64); // float has no normalization so we cannot calculate it
//     let n_sum = n_mean * n_len;
  
//     vec![
//         n_mean, 
//         n_sum, 
//         a_n_t__result.1 as f64,
//         a_n_t__result.2 as f64
//     ]
// }
#[wasm_bindgen]
#[derive(Clone)]
pub struct O_n_mean_n_min_n_max_for_u8 {
    n_mean: f64,
    n_min: u8,
    n_max: u8,
}
#[wasm_bindgen]
pub struct O_n_mean_n_sum_n_min_n_max_for_u8 {
    n_mean: f64,
    n_sum: f64,
    n_min: u8,
    n_max: u8,
}
#[wasm_bindgen]
pub fn f_o_mean_n_sum_n_min_n_max__for_u8(a_n: &[u8]) -> O_n_mean_n_sum_n_min_n_max_for_u8 {

    let n_len = a_n.len() as f64;
    let o_init: O_n_mean_n_min_n_max_for_u8 = O_n_mean_n_min_n_max_for_u8 {
        n_mean: 0.0,
        n_min: u8::MAX,
        n_max: u8::MIN
    };
        let o_result = a_n.iter().fold(o_init, |mut o_acc, &val| {
        o_acc.n_mean = o_acc.n_mean+(val as f64 / n_len);
        o_acc.n_min = o_acc.n_min.min(val);
        o_acc.n_max = o_acc.n_max.max(val);
        o_acc
    });


    let n_mean = o_result.n_mean;//as float
    // let = n_mean / (u8::MAX as f64); // float has no normalization so we cannot calculate it
    let n_sum = n_mean * n_len;
    O_n_mean_n_sum_n_min_n_max_for_u8{
        n_mean:o_result.n_mean, 
        n_sum:n_sum, 
        n_min:o_result.n_min,
        n_max:o_result.n_max
    }

}
#[wasm_bindgen]
pub fn f_a_o_mean_n_sum_n_min_n_max__for_u8(
    a_n: &[u8],
    n_channels: usize
) -> Vec<O_n_mean_n_sum_n_min_n_max_for_u8> {

    let n_len = a_n.len() as f64;
    let a_o_init = vec![
        O_n_mean_n_min_n_max_for_u8 {
            n_mean: 0.0,
            n_min: u8::MAX,
            n_max: u8::MIN
        }; n_channels
    ];


    let a_o_result = a_n.iter()
    .enumerate()
    .fold(
        a_o_init,
         |mut a_o_acc, (n_idx,&val)|
         {
            let n_idx_channel = n_idx % n_channels;
            let o_acc = &mut a_o_acc[n_idx_channel];
            o_acc.n_mean = o_acc.n_mean+(val as f64 / n_len);
            o_acc.n_min = o_acc.n_min.min(val);
            o_acc.n_max = o_acc.n_max.max(val);
            a_o_acc
        }
    );
    let mut a_o= Vec::new();
    for n_channel in 0..n_channels{
        let o_result = &a_o_result[n_channel];
        a_o.push(
            O_n_mean_n_sum_n_min_n_max_for_u8{
                n_mean:o_result.n_mean, 
                n_sum:o_result.n_mean * n_len, 
                n_min:o_result.n_min,
                n_max:o_result.n_max
            }
        );
    }
    a_o

}

#[wasm_bindgen]
pub fn f_a_n_u8__sorted(
    a_n: &[u8]
) -> Vec<u8> {
    let mut a_n_sorted = a_n.to_vec();
    a_n_sorted.sort();
    a_n_sorted
}
#[wasm_bindgen]
pub fn f_a_n_u8__sorted_unstable(
    a_n: &[u8]
) -> Vec<u8> {
    let mut a_n_sorted = a_n.to_vec();
    a_n_sorted.sort_unstable();
    a_n_sorted
}

//code_autoextender_end

//code_autoextender_start
////    return [ 'f64'].map(
////     (s_type) =>{
////         return s_code.replaceAll('f32', s_type)
////    }).join('\n')
#[wasm_bindgen]
pub fn f_a_n_f32__sorted(
    a_n: &[f32]
) -> Vec<f32> {
    let mut a_n_sorted = a_n.to_vec();

    a_n_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    a_n_sorted
}
//code_autoextender_end




#[wasm_bindgen]
pub fn f_a_n_f64_normalized__from_a_n_u64__loop(
    a_n_u64: &[u64],
    n_min: u64, 
    n_max: u64
) -> Vec<f64> {

    let mut a_n_u64__normalized = Vec::with_capacity(a_n_u64.len());
    let n_range = n_max - n_min; 
    for &n_value in a_n_u64{
        a_n_u64__normalized.push(
            ((n_value as f64 - n_min as f64) / n_range as f64)
        )
    }
    a_n_u64__normalized

}

#[wasm_bindgen]
pub fn f_a_n_f64_normalized__from_a_n_u64__iermapcollect(
    a_n_u64: &[u64],
    n_min: u64, 
    n_max: u64
) -> Vec<f64> {

    let n_range = n_max - n_min; 
    a_n_u64.iter().map(
        |&n_value|{
            ((n_value as f64 - n_min as f64) / n_range as f64)
        })
        .collect()
}

fn remap_values(values: &[u64], new_max: u64) -> Vec<u64> {
    let min = *values.iter().min().expect("Array should not be empty");
    let max = *values.iter().max().expect("Array should not be empty");

    values.iter()
        .map(|&value| ((value - min) * new_max) / (max - min))
        .collect()
}



#[wasm_bindgen]
pub fn manipulate_each_element_with_expression(arr: &mut [u8], expression_template: &str) -> Result<(), JsValue> {
    for element in arr.iter_mut() {
        let expression = expression_template.replace("{}", &element.to_string());
        match eval(&expression).and_then(|result| result.as_number()) {
            Ok(new_value) => {
                // let clamped_value = new_value.max(0.0).min(255.0) as u8; // Clamping the value
                *element = new_value as u8;
            }
            Err(e) => return Err(JsValue::from_str(&format!("Error in expression evaluation: {}", e))),
        }
    }
    Ok(())
}


fn main() {
    println!("Hello, world!");
    let a_n_u8__test = [1,2,3,4,4,3,2,1,1,2,3,4,4,3,2,1];
    let a_o = f_a_o_n_mean_n_sum__for_u8(&a_n_u8__test,4);
    println!("a_o {:?}", a_o[0].n_mean);

    // let length = 4096 * 4096;
    // let mut rng = rand::thread_rng();
    // let random_numbers: Vec<u64> = (0..length).map(|_| rng.gen()).collect();
    // match product(&random_numbers) {
    //     Some(prod) => println!("The product of the array is: {}", prod),
    //     None => println!("Overflow occurred"),
    // }
}
