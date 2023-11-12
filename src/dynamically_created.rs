
    // all code in this file was automatically generated! 
    

    #[wasm_bindgen]
    pub struct O_n_min_return_for_u8 {
        n_min_nor: f64
    }


#[wasm_bindgen]
pub fn f__o_n_min(
    a_n: &[u8]
) -> O_n_min_return_for_u8{

    

    
    let v_init =  vec![
        
a_n[0]

    ];
    
    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc[0] = v_acc[0].min(v_in_fold);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_min_return_for_u8{
            n_min_nor:v_result[0] as f64 / u8::MAX

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_max_return_for_u8 {
        n_max_nor: f64
    }


#[wasm_bindgen]
pub fn f__o_n_max(
    a_n: &[u8]
) -> O_n_max_return_for_u8{

    

    
    let v_init =  vec![
        
a_n[0]

    ];
    
    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc[0] = v_acc[0].max(v_in_fold);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_max_return_for_u8{
            n_max_nor:v_result[0] as f64 / u8::MAX

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_min_n_max_return_for_u8 {
        n_min_nor: f64,
n_max_nor: f64
    }


#[wasm_bindgen]
pub fn f__o_n_min_n_max(
    a_n: &[u8]
) -> O_n_min_n_max_return_for_u8{

    

    
    let v_init =  vec![
        
a_n[0],
a_n[0]

    ];
    
    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc[0] = v_acc[0].min(v_in_fold);
v_acc[1] = v_acc[1].max(v_in_fold);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_min_n_max_return_for_u8{
            n_min_nor:v_result[0] as f64 / u8::MAX,
n_max_nor:v_result[0] as f64 / u8::MAX

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_mean_return_for_u8 {
        n_sum: f64
    }


#[wasm_bindgen]
pub fn f__o_n_mean(
    a_n: &[u8]
) -> O_n_mean_return_for_u8{

    let n_len = a_n.len() as f64;

    
    let v_init =  vec![
        
0.0

    ];
    
    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc[0] = v_acc[0]+(v_in_fold as f64 / n_len);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_mean_return_for_u8{
            n_sum:v_result[0] * n_len

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_min_n_mean_return_for_u8 {
        n_min_nor: f64,
n_sum: f64
    }


    #[wasm_bindgen]
    pub struct O_n_min_n_mean_accumulator_for_u8 {
        n_min: u8,
n_mean: f64
    }
    
#[wasm_bindgen]
pub fn f__o_n_min_n_mean(
    a_n: &[u8]
) -> O_n_min_n_mean_return_for_u8{

    let n_len = a_n.len() as f64;

    
let v_init: O_n_min_n_mean_accumulator_for_u8 = O_n_min_n_mean_accumulator_for_u8 {
    n_min:a_n[0],
n_mean:0.0
};

    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc.n_min = v_acc.n_min.min(v_in_fold);
v_acc.n_mean = v_acc.n_mean+(v_in_fold as f64 / n_len);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_min_n_mean_return_for_u8{
            n_min_nor:v_result.n_min as f64 / u8::MAX,
n_sum:v_result.n_mean * n_len

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_max_n_mean_return_for_u8 {
        n_max_nor: f64,
n_sum: f64
    }


    #[wasm_bindgen]
    pub struct O_n_max_n_mean_accumulator_for_u8 {
        n_max: u8,
n_mean: f64
    }
    
#[wasm_bindgen]
pub fn f__o_n_max_n_mean(
    a_n: &[u8]
) -> O_n_max_n_mean_return_for_u8{

    let n_len = a_n.len() as f64;

    
let v_init: O_n_max_n_mean_accumulator_for_u8 = O_n_max_n_mean_accumulator_for_u8 {
    n_max:a_n[0],
n_mean:0.0
};

    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc.n_max = v_acc.n_max.max(v_in_fold);
v_acc.n_mean = v_acc.n_mean+(v_in_fold as f64 / n_len);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_max_n_mean_return_for_u8{
            n_max_nor:v_result.n_max as f64 / u8::MAX,
n_sum:v_result.n_mean * n_len

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_min_n_max_n_mean_return_for_u8 {
        n_min_nor: f64,
n_max_nor: f64,
n_sum: f64
    }


    #[wasm_bindgen]
    pub struct O_n_min_n_max_n_mean_accumulator_for_u8 {
        n_min: u8,
n_max: u8,
n_mean: f64
    }
    
#[wasm_bindgen]
pub fn f__o_n_min_n_max_n_mean(
    a_n: &[u8]
) -> O_n_min_n_max_n_mean_return_for_u8{

    let n_len = a_n.len() as f64;

    
let v_init: O_n_min_n_max_n_mean_accumulator_for_u8 = O_n_min_n_max_n_mean_accumulator_for_u8 {
    n_min:a_n[0],
n_max:a_n[0],
n_mean:0.0
};

    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc.n_min = v_acc.n_min.min(v_in_fold);
v_acc.n_max = v_acc.n_max.max(v_in_fold);
v_acc.n_mean = v_acc.n_mean+(v_in_fold as f64 / n_len);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_min_n_max_n_mean_return_for_u8{
            n_min_nor:v_result.n_min as f64 / u8::MAX,
n_max_nor:v_result.n_max as f64 / u8::MAX,
n_sum:v_result.n_mean * n_len

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_mean_n_variance_return_for_u8 {
        n_sum: f64,
n_variance: f64
    }


#[wasm_bindgen]
pub fn f__o_n_mean_n_variance(
    a_n: &[u8]
) -> O_n_mean_n_variance_return_for_u8{

    let n_len = a_n.len() as f64;

    
    let v_init =  vec![
        
0.0,
0.0

    ];
    
    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc[0] = v_acc[0]+(v_in_fold as f64 / n_len);
v_acc[1] = v_acc[1] + (v_in_fold as f64).powi(2);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_mean_n_variance_return_for_u8{
            n_sum:v_result[0] * n_len,
n_variance:(v_result[0] / n_len) - n_mean.powi(2)

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_min_n_mean_n_variance_return_for_u8 {
        n_min_nor: f64,
n_sum: f64,
n_variance: f64
    }


    #[wasm_bindgen]
    pub struct O_n_min_n_mean_n_variance_accumulator_for_u8 {
        n_min: u8,
n_mean: f64,
n_variance: f64
    }
    
#[wasm_bindgen]
pub fn f__o_n_min_n_mean_n_variance(
    a_n: &[u8]
) -> O_n_min_n_mean_n_variance_return_for_u8{

    let n_len = a_n.len() as f64;

    
let v_init: O_n_min_n_mean_n_variance_accumulator_for_u8 = O_n_min_n_mean_n_variance_accumulator_for_u8 {
    n_min:a_n[0],
n_mean:0.0,
n_variance:0.0
};

    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc.n_min = v_acc.n_min.min(v_in_fold);
v_acc.n_mean = v_acc.n_mean+(v_in_fold as f64 / n_len);
v_acc.n_variance = v_acc.n_variance + (v_in_fold as f64).powi(2);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_min_n_mean_n_variance_return_for_u8{
            n_min_nor:v_result.n_min as f64 / u8::MAX,
n_sum:v_result.n_mean * n_len,
n_variance:(v_result.n_variance / n_len) - n_mean.powi(2)

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_max_n_mean_n_variance_return_for_u8 {
        n_max_nor: f64,
n_sum: f64,
n_variance: f64
    }


    #[wasm_bindgen]
    pub struct O_n_max_n_mean_n_variance_accumulator_for_u8 {
        n_max: u8,
n_mean: f64,
n_variance: f64
    }
    
#[wasm_bindgen]
pub fn f__o_n_max_n_mean_n_variance(
    a_n: &[u8]
) -> O_n_max_n_mean_n_variance_return_for_u8{

    let n_len = a_n.len() as f64;

    
let v_init: O_n_max_n_mean_n_variance_accumulator_for_u8 = O_n_max_n_mean_n_variance_accumulator_for_u8 {
    n_max:a_n[0],
n_mean:0.0,
n_variance:0.0
};

    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc.n_max = v_acc.n_max.max(v_in_fold);
v_acc.n_mean = v_acc.n_mean+(v_in_fold as f64 / n_len);
v_acc.n_variance = v_acc.n_variance + (v_in_fold as f64).powi(2);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_max_n_mean_n_variance_return_for_u8{
            n_max_nor:v_result.n_max as f64 / u8::MAX,
n_sum:v_result.n_mean * n_len,
n_variance:(v_result.n_variance / n_len) - n_mean.powi(2)

        }
    );
}

    a_o[0]
}



    #[wasm_bindgen]
    pub struct O_n_min_n_max_n_mean_n_variance_return_for_u8 {
        n_min_nor: f64,
n_max_nor: f64,
n_sum: f64,
n_variance: f64
    }


    #[wasm_bindgen]
    pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_u8 {
        n_min: u8,
n_max: u8,
n_mean: f64,
n_variance: f64
    }
    
#[wasm_bindgen]
pub fn f__o_n_min_n_max_n_mean_n_variance(
    a_n: &[u8]
) -> O_n_min_n_max_n_mean_n_variance_return_for_u8{

    let n_len = a_n.len() as f64;

    
let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_u8 = O_n_min_n_max_n_mean_n_variance_accumulator_for_u8 {
    n_min:a_n[0],
n_max:a_n[0],
n_mean:0.0,
n_variance:0.0
};

    
let v_result = a_n
    .iter()
    .fold(
        v_init,
        |mut v_acc, &v_in_fold|
        {
            v_acc.n_min = v_acc.n_min.min(v_in_fold);
v_acc.n_max = v_acc.n_max.max(v_in_fold);
v_acc.n_mean = v_acc.n_mean+(v_in_fold as f64 / n_len);
v_acc.n_variance = v_acc.n_variance + (v_in_fold as f64).powi(2);
            v_acc
        }
);

    
let mut a_o= Vec::new();
for n_channel in 0..1{
    a_o.push(
        O_n_min_n_max_n_mean_n_variance_return_for_u8{
            n_min_nor:v_result.n_min as f64 / u8::MAX,
n_max_nor:v_result.n_max as f64 / u8::MAX,
n_sum:v_result.n_mean * n_len,
n_variance:(v_result.n_variance / n_len) - n_mean.powi(2)

        }
    );
}

    a_o[0]
}

    