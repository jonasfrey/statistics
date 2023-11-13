//This file was automatically generated on Tue Nov 14 2023 00:35:13 GMT+0100 (Central European Standard Time),(1699918513004)
// all code in this file was automatically generated!
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct O_n_min_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_u8(a_n: &[u8]) -> O_n_min_return_for_u8 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_u8 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (u8::MAX - u8::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_return_for_u16 {
    pub n_min: u16,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_u16(a_n: &[u16]) -> O_n_min_return_for_u16 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_u16 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (u16::MAX - u16::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_return_for_u32 {
    pub n_min: u32,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_u32(a_n: &[u32]) -> O_n_min_return_for_u32 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_u32 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (u32::MAX - u32::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_return_for_u64 {
    pub n_min: u64,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_u64(a_n: &[u64]) -> O_n_min_return_for_u64 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_u64 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (u64::MAX - u64::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_return_for_f32 {
    pub n_min: f32,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_f32(a_n: &[f32]) -> O_n_min_return_for_f32 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_f32 { n_min: v_result[0] });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_return_for_f64 {
    pub n_min: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_f64(a_n: &[f64]) -> O_n_min_return_for_f64 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_f64 { n_min: v_result[0] });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_return_for_u8 {
    pub n_max: u8,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_u8(a_n: &[u8]) -> O_n_max_return_for_u8 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_u8 {
            n_max: v_result[0],
            n_max_nor: (v_result[0]) as f64 / (u8::MAX - u8::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_return_for_u16 {
    pub n_max: u16,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_u16(a_n: &[u16]) -> O_n_max_return_for_u16 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_u16 {
            n_max: v_result[0],
            n_max_nor: (v_result[0]) as f64 / (u16::MAX - u16::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_return_for_u32 {
    pub n_max: u32,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_u32(a_n: &[u32]) -> O_n_max_return_for_u32 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_u32 {
            n_max: v_result[0],
            n_max_nor: (v_result[0]) as f64 / (u32::MAX - u32::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_return_for_u64 {
    pub n_max: u64,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_u64(a_n: &[u64]) -> O_n_max_return_for_u64 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_u64 {
            n_max: v_result[0],
            n_max_nor: (v_result[0]) as f64 / (u64::MAX - u64::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_return_for_f32 {
    pub n_max: f32,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_f32(a_n: &[f32]) -> O_n_max_return_for_f32 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_f32 { n_max: v_result[0] });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_return_for_f64 {
    pub n_max: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_f64(a_n: &[f64]) -> O_n_max_return_for_f64 {
    let v_init = vec![a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_f64 { n_max: v_result[0] });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
    pub n_max: u8,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_u8(a_n: &[u8]) -> O_n_min_n_max_return_for_u8 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_u8 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (u8::MAX - u8::MIN) as f64,
            n_max: v_result[1],
            n_max_nor: (v_result[1]) as f64 / (u8::MAX - u8::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_return_for_u16 {
    pub n_min: u16,
    pub n_min_nor: f64,
    pub n_max: u16,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_u16(a_n: &[u16]) -> O_n_min_n_max_return_for_u16 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_u16 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (u16::MAX - u16::MIN) as f64,
            n_max: v_result[1],
            n_max_nor: (v_result[1]) as f64 / (u16::MAX - u16::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_return_for_u32 {
    pub n_min: u32,
    pub n_min_nor: f64,
    pub n_max: u32,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_u32(a_n: &[u32]) -> O_n_min_n_max_return_for_u32 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_u32 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (u32::MAX - u32::MIN) as f64,
            n_max: v_result[1],
            n_max_nor: (v_result[1]) as f64 / (u32::MAX - u32::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_return_for_u64 {
    pub n_min: u64,
    pub n_min_nor: f64,
    pub n_max: u64,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_u64(a_n: &[u64]) -> O_n_min_n_max_return_for_u64 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_u64 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (u64::MAX - u64::MIN) as f64,
            n_max: v_result[1],
            n_max_nor: (v_result[1]) as f64 / (u64::MAX - u64::MIN) as f64,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_return_for_f32 {
    pub n_min: f32,
    pub n_max: f32,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_f32(a_n: &[f32]) -> O_n_min_n_max_return_for_f32 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_f32 {
            n_min: v_result[0],
            n_max: v_result[1],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_return_for_f64 {
    pub n_min: f64,
    pub n_max: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_f64(a_n: &[f64]) -> O_n_min_n_max_return_for_f64 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_f64 {
            n_min: v_result[0],
            n_max: v_result[1],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_u8 {
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_u8(a_n: &[u8]) -> O_n_mean_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_u8 {
            n_sum: v_result[0] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_u16 {
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_u16(a_n: &[u16]) -> O_n_mean_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_u16 {
            n_sum: v_result[0] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_u32 {
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_u32(a_n: &[u32]) -> O_n_mean_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_u32 {
            n_sum: v_result[0] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_u64 {
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_u64(a_n: &[u64]) -> O_n_mean_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_u64 {
            n_sum: v_result[0] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_f32 {
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_f32(a_n: &[f32]) -> O_n_mean_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_f32 {
            n_sum: v_result[0] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_f64 {
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_f64(a_n: &[f64]) -> O_n_mean_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_f64 {
            n_sum: v_result[0] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_u8 {
    pub n_min: u8,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_u8(a_n: &[u8]) -> O_n_min_n_mean_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_u8 = O_n_min_n_mean_accumulator_for_u8 {
        n_min: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_return_for_u8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u8::MAX - u8::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_u16 {
    pub n_min: u16,
    pub n_min_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_u16 {
    pub n_min: u16,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_u16(a_n: &[u16]) -> O_n_min_n_mean_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_u16 = O_n_min_n_mean_accumulator_for_u16 {
        n_min: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_return_for_u16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u16::MAX - u16::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_u32 {
    pub n_min: u32,
    pub n_min_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_u32 {
    pub n_min: u32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_u32(a_n: &[u32]) -> O_n_min_n_mean_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_u32 = O_n_min_n_mean_accumulator_for_u32 {
        n_min: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_return_for_u32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u32::MAX - u32::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_u64 {
    pub n_min: u64,
    pub n_min_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_u64 {
    pub n_min: u64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_u64(a_n: &[u64]) -> O_n_min_n_mean_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_u64 = O_n_min_n_mean_accumulator_for_u64 {
        n_min: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_return_for_u64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u64::MAX - u64::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_f32 {
    pub n_min: f32,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_f32 {
    pub n_min: f32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_f32(a_n: &[f32]) -> O_n_min_n_mean_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_f32 = O_n_min_n_mean_accumulator_for_f32 {
        n_min: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_return_for_f32 {
            n_min: v_result.n_min,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_f64 {
    pub n_min: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_f64(a_n: &[f64]) -> O_n_min_n_mean_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![a_n[0], 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_return_for_f64 {
            n_min: v_result[0],
            n_sum: v_result[1] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_u8 {
    pub n_max: u8,
    pub n_max_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_u8 {
    pub n_max: u8,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_u8(a_n: &[u8]) -> O_n_max_n_mean_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_u8 = O_n_max_n_mean_accumulator_for_u8 {
        n_max: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_return_for_u8 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u8::MAX - u8::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_u16 {
    pub n_max: u16,
    pub n_max_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_u16 {
    pub n_max: u16,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_u16(a_n: &[u16]) -> O_n_max_n_mean_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_u16 = O_n_max_n_mean_accumulator_for_u16 {
        n_max: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_return_for_u16 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u16::MAX - u16::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_u32 {
    pub n_max: u32,
    pub n_max_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_u32 {
    pub n_max: u32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_u32(a_n: &[u32]) -> O_n_max_n_mean_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_u32 = O_n_max_n_mean_accumulator_for_u32 {
        n_max: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_return_for_u32 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u32::MAX - u32::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_u64 {
    pub n_max: u64,
    pub n_max_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_u64 {
    pub n_max: u64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_u64(a_n: &[u64]) -> O_n_max_n_mean_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_u64 = O_n_max_n_mean_accumulator_for_u64 {
        n_max: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_return_for_u64 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u64::MAX - u64::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_f32 {
    pub n_max: f32,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_f32 {
    pub n_max: f32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_f32(a_n: &[f32]) -> O_n_max_n_mean_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_f32 = O_n_max_n_mean_accumulator_for_f32 {
        n_max: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_return_for_f32 {
            n_max: v_result.n_max,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_f64 {
    pub n_max: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_f64(a_n: &[f64]) -> O_n_max_n_mean_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![a_n[0], 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].max(v_in_fold);
        v_acc[1] = v_acc[1] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_return_for_f64 {
            n_max: v_result[0],
            n_sum: v_result[1] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
    pub n_max: u8,
    pub n_max_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_u8 {
    pub n_min: u8,
    pub n_max: u8,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_u8(a_n: &[u8]) -> O_n_min_n_max_n_mean_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_u8 = O_n_min_n_max_n_mean_accumulator_for_u8 {
        n_min: a_n[0],
        n_max: a_n[0],
        n_mean: 0.0,
    };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_return_for_u8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u8::MAX - u8::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u8::MAX - u8::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_u16 {
    pub n_min: u16,
    pub n_min_nor: f64,
    pub n_max: u16,
    pub n_max_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_u16 {
    pub n_min: u16,
    pub n_max: u16,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_u16(a_n: &[u16]) -> O_n_min_n_max_n_mean_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_u16 =
        O_n_min_n_max_n_mean_accumulator_for_u16 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_return_for_u16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u16::MAX - u16::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u16::MAX - u16::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_u32 {
    pub n_min: u32,
    pub n_min_nor: f64,
    pub n_max: u32,
    pub n_max_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_u32 {
    pub n_min: u32,
    pub n_max: u32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_u32(a_n: &[u32]) -> O_n_min_n_max_n_mean_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_u32 =
        O_n_min_n_max_n_mean_accumulator_for_u32 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_return_for_u32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u32::MAX - u32::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u32::MAX - u32::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_u64 {
    pub n_min: u64,
    pub n_min_nor: f64,
    pub n_max: u64,
    pub n_max_nor: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_u64 {
    pub n_min: u64,
    pub n_max: u64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_u64(a_n: &[u64]) -> O_n_min_n_max_n_mean_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_u64 =
        O_n_min_n_max_n_mean_accumulator_for_u64 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_return_for_u64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u64::MAX - u64::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u64::MAX - u64::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_f32 {
    pub n_min: f32,
    pub n_max: f32,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_f32 {
    pub n_min: f32,
    pub n_max: f32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_f32(a_n: &[f32]) -> O_n_min_n_max_n_mean_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_f32 =
        O_n_min_n_max_n_mean_accumulator_for_f32 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_return_for_f32 {
            n_min: v_result.n_min,
            n_max: v_result.n_max,
            n_sum: v_result.n_mean * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_f64 {
    pub n_min: f64,
    pub n_max: f64,
    pub n_sum: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_f64(a_n: &[f64]) -> O_n_min_n_max_n_mean_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![a_n[0], a_n[0], 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc[2] = v_acc[2] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_return_for_f64 {
            n_min: v_result[0],
            n_max: v_result[1],
            n_sum: v_result[2] * n_len,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_u8 {
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_u8(a_n: &[u8]) -> O_n_mean_n_variance_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[0] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_u8 {
            n_sum: v_result[0] * n_len,
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_u16 {
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_u16(a_n: &[u16]) -> O_n_mean_n_variance_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[0] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_u16 {
            n_sum: v_result[0] * n_len,
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_u32 {
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_u32(a_n: &[u32]) -> O_n_mean_n_variance_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[0] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_u32 {
            n_sum: v_result[0] * n_len,
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_u64 {
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_u64(a_n: &[u64]) -> O_n_mean_n_variance_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[0] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_u64 {
            n_sum: v_result[0] * n_len,
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_f32 {
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_f32(a_n: &[f32]) -> O_n_mean_n_variance_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[0] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_f32 {
            n_sum: v_result[0] * n_len,
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_f64 {
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_f64(a_n: &[f64]) -> O_n_mean_n_variance_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[0] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_f64 {
            n_sum: v_result[0] * n_len,
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_u8 {
    pub n_min: u8,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_u8(a_n: &[u8]) -> O_n_min_n_mean_n_variance_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_u8 =
        O_n_min_n_mean_n_variance_accumulator_for_u8 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_u8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u8::MAX - u8::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_u16 {
    pub n_min: u16,
    pub n_min_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_u16 {
    pub n_min: u16,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_u16(
    a_n: &[u16],
) -> O_n_min_n_mean_n_variance_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_u16 =
        O_n_min_n_mean_n_variance_accumulator_for_u16 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_u16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u16::MAX - u16::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_u32 {
    pub n_min: u32,
    pub n_min_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_u32 {
    pub n_min: u32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_u32(
    a_n: &[u32],
) -> O_n_min_n_mean_n_variance_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_u32 =
        O_n_min_n_mean_n_variance_accumulator_for_u32 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_u32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u32::MAX - u32::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_u64 {
    pub n_min: u64,
    pub n_min_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_u64 {
    pub n_min: u64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_u64(
    a_n: &[u64],
) -> O_n_min_n_mean_n_variance_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_u64 =
        O_n_min_n_mean_n_variance_accumulator_for_u64 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_u64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u64::MAX - u64::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_f32 {
    pub n_min: f32,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_f32 {
    pub n_min: f32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_f32(
    a_n: &[f32],
) -> O_n_min_n_mean_n_variance_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_f32 =
        O_n_min_n_mean_n_variance_accumulator_for_f32 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_f32 {
            n_min: v_result.n_min,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_f64 {
    pub n_min: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_f64(
    a_n: &[f64],
) -> O_n_min_n_mean_n_variance_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![a_n[0], 0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[1] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_f64 {
            n_min: v_result[0],
            n_sum: v_result[1] * n_len,
            n_variance: (v_result[2] / n_len) - v_result[1].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_u8 {
    pub n_max: u8,
    pub n_max_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_u8 {
    pub n_max: u8,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_u8(a_n: &[u8]) -> O_n_max_n_mean_n_variance_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_u8 =
        O_n_max_n_mean_n_variance_accumulator_for_u8 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_u8 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u8::MAX - u8::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_u16 {
    pub n_max: u16,
    pub n_max_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_u16 {
    pub n_max: u16,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_u16(
    a_n: &[u16],
) -> O_n_max_n_mean_n_variance_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_u16 =
        O_n_max_n_mean_n_variance_accumulator_for_u16 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_u16 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u16::MAX - u16::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_u32 {
    pub n_max: u32,
    pub n_max_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_u32 {
    pub n_max: u32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_u32(
    a_n: &[u32],
) -> O_n_max_n_mean_n_variance_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_u32 =
        O_n_max_n_mean_n_variance_accumulator_for_u32 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_u32 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u32::MAX - u32::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_u64 {
    pub n_max: u64,
    pub n_max_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_u64 {
    pub n_max: u64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_u64(
    a_n: &[u64],
) -> O_n_max_n_mean_n_variance_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_u64 =
        O_n_max_n_mean_n_variance_accumulator_for_u64 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_u64 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u64::MAX - u64::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_f32 {
    pub n_max: f32,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_f32 {
    pub n_max: f32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_f32(
    a_n: &[f32],
) -> O_n_max_n_mean_n_variance_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_f32 =
        O_n_max_n_mean_n_variance_accumulator_for_f32 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_f32 {
            n_max: v_result.n_max,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_f64 {
    pub n_max: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_f64(
    a_n: &[f64],
) -> O_n_max_n_mean_n_variance_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![a_n[0], 0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].max(v_in_fold);
        v_acc[1] = v_acc[1] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[1] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_f64 {
            n_max: v_result[0],
            n_sum: v_result[1] * n_len,
            n_variance: (v_result[2] / n_len) - v_result[1].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
    pub n_max: u8,
    pub n_max_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_u8 {
    pub n_min: u8,
    pub n_max: u8,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_u8(
    a_n: &[u8],
) -> O_n_min_n_max_n_mean_n_variance_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_u8 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_u8 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_u8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u8::MAX - u8::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u8::MAX - u8::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_u16 {
    pub n_min: u16,
    pub n_min_nor: f64,
    pub n_max: u16,
    pub n_max_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_u16 {
    pub n_min: u16,
    pub n_max: u16,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_u16(
    a_n: &[u16],
) -> O_n_min_n_max_n_mean_n_variance_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_u16 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_u16 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_u16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u16::MAX - u16::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u16::MAX - u16::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_u32 {
    pub n_min: u32,
    pub n_min_nor: f64,
    pub n_max: u32,
    pub n_max_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_u32 {
    pub n_min: u32,
    pub n_max: u32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_u32(
    a_n: &[u32],
) -> O_n_min_n_max_n_mean_n_variance_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_u32 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_u32 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_u32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u32::MAX - u32::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u32::MAX - u32::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_u64 {
    pub n_min: u64,
    pub n_min_nor: f64,
    pub n_max: u64,
    pub n_max_nor: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_u64 {
    pub n_min: u64,
    pub n_max: u64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_u64(
    a_n: &[u64],
) -> O_n_min_n_max_n_mean_n_variance_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_u64 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_u64 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_u64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u64::MAX - u64::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u64::MAX - u64::MIN) as f64,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_f32 {
    pub n_min: f32,
    pub n_max: f32,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_f32 {
    pub n_min: f32,
    pub n_max: f32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_f32(
    a_n: &[f32],
) -> O_n_min_n_max_n_mean_n_variance_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_f32 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_f32 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_f32 {
            n_min: v_result.n_min,
            n_max: v_result.n_max,
            n_sum: v_result.n_mean * n_len,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_f64 {
    pub n_min: f64,
    pub n_max: f64,
    pub n_sum: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_f64(
    a_n: &[f64],
) -> O_n_min_n_max_n_mean_n_variance_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![a_n[0], a_n[0], 0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc[2] = v_acc[2] + (v_in_fold as f64 / n_len);
        v_acc[2] = v_acc[2] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_f64 {
            n_min: v_result[0],
            n_max: v_result[1],
            n_sum: v_result[2] * n_len,
            n_variance: (v_result[3] / n_len) - v_result[2].powi(2),
        });
    }

    a_o.remove(0)
}
