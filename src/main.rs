use wasm_bindgen::prelude::*;


struct AggregatedData {
    min: u64,
    max: u64,
}



#[wasm_bindgen]

pub fn f_a_a_n_min_n_max__u64__array(
    a_n_u64: &[u64],
    n_channels: usize
) -> Vec<u64> {

    

    let a_n_init = vec![u64::MAX, u64::MIN];
    let a_n_u64__init: Vec<u64> = (0..n_channels)
        .flat_map(|_| a_n_init.iter().cloned())
        .collect();


    let a_n_u64__result = a_n_u64
        .iter()
        .enumerate()
        .fold(
            a_n_u64__init,
            |mut a_a_n_acc, (n_idx, &val)|
            {
                let n_idx_channel = n_idx % n_channels;
                a_a_n_acc[n_idx_channel*n_channels+0] = a_a_n_acc[n_idx_channel*n_channels+0].min(val);
                a_a_n_acc[n_idx_channel*n_channels+1] = a_a_n_acc[n_idx_channel*n_channels+1].max(val);
                a_a_n_acc
        }
    );

    return a_n_u64__result.to_vec();
}


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
pub fn f_a_n_min_n_max__u64__array(a_n_u64: &[u64]) -> Vec<u64> {

    let a_n_u64__init = [u64::MAX, u64::MIN];
    let a_n_u64__result = a_n_u64.iter().fold(a_n_u64__init, |mut a_n_acc, &val| {
        a_n_acc[0] = a_n_acc[0].min(val);
        a_n_acc[1] = a_n_acc[1].max(val);
        a_n_u64__init
    });

    return a_n_u64__result.to_vec()
}



#[wasm_bindgen]
pub fn f_a_n_min_n_max__u64__struct(a_n_u64: &[u64]) -> Vec<u64> {

    let initial_data = AggregatedData {
        min: u64::MAX,
        max: u64::MIN,
    };

    let result = a_n_u64.iter().fold(initial_data, |mut acc, &val| {
        acc.min = acc.min.min(val);
        acc.max = acc.max.max(val);
        acc
    });

    return vec![result.min, result.max]
}

#[wasm_bindgen]
pub fn f_a_n_min_n_max__u64__tuple(a_n_u64: &[u64]) -> Vec<u64> {
    let (min, max) = a_n_u64.iter().fold((u64::MAX, u64::MIN), |(min, max), &val| {
        (min.min(val), max.max(val))
    });
    return vec![min, max]
}

#[wasm_bindgen]
pub fn f_a_n_min_n_max(
    a_n_u8: &[u8]
)->Vec<u8>{
    if a_n_u8.is_empty(){
        return vec![]
    }

    let mut n_min = a_n_u8[0];
    let mut n_max = a_n_u8[0];
    for n_u8 in a_n_u8.iter(){
        // println!("{:?}", n_u8);
        if(n_u8 < &n_min){
            n_min = *n_u8
        }
        if n_u8 > &n_max{
            n_max = *n_u8
        }
    }
    return vec![n_min, n_max]

    // [
    //     *a_n_u8.iter().min().unwrap(),
    //     *a_n_u8.iter().max().unwrap(),
    // ].to_vec()
}
#[wasm_bindgen]
pub fn f_a_n_sum_n_avg(
    a_n_u8: &[u8]
)->Vec<f64>{
    if a_n_u8.is_empty(){
        return vec![]
    }
    let n_sum = a_n_u8.iter().map(|&x| x as u32).sum::<u32>() as f64;

    [
        n_sum,
        n_sum as f64 / a_n_u8.len() as f64 ,
    ].to_vec()
}

fn main() {
    println!("Hello, world!");
    let a_n_u8__test = [1,2,3,4,5,6,1,2,3,4,4,5];
    let a_n_min_n_max = f_a_n_min_n_max(&a_n_u8__test);
    println!("a_n_min_n_max {:?}", a_n_min_n_max);
    let a_n_sum_n_avg = f_a_n_sum_n_avg(&a_n_u8__test);
    println!("a_n_sum_n_avg {:?}", a_n_sum_n_avg);
}
