//This file was automatically generated on Fri Nov 17 2023 15:28:32 GMT+0100 (Central European Standard Time),(1700231312213)
// all code in this file was automatically generated!
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct O_n_min_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_u8(a_n: &[u8]) -> O_n_min_return_for_u8 {
    let n_min = *a_n.iter().min().unwrap();
    let v_result = [n_min as u8];

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
pub struct O_n_min_return_for_i8 {
    pub n_min: i8,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_i8(a_n: &[i8]) -> O_n_min_return_for_i8 {
    let n_min = *a_n.iter().min().unwrap();
    let v_result = [n_min as i8];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_i8 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
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
    let n_min = *a_n.iter().min().unwrap();
    let v_result = [n_min as u16];

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
pub struct O_n_min_return_for_i16 {
    pub n_min: i16,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_i16(a_n: &[i16]) -> O_n_min_return_for_i16 {
    let n_min = *a_n.iter().min().unwrap();
    let v_result = [n_min as i16];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_i16 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
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
    let n_min = *a_n.iter().min().unwrap();
    let v_result = [n_min as u32];

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
pub struct O_n_min_return_for_i32 {
    pub n_min: i32,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_i32(a_n: &[i32]) -> O_n_min_return_for_i32 {
    let n_min = *a_n.iter().min().unwrap();
    let v_result = [n_min as i32];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_i32 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
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
    let n_min = *a_n.iter().min().unwrap();
    let v_result = [n_min as u64];

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
pub struct O_n_min_return_for_i64 {
    pub n_min: i64,
    pub n_min_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_for_i64(a_n: &[i64]) -> O_n_min_return_for_i64 {
    let n_min = *a_n.iter().min().unwrap();
    let v_result = [n_min as i64];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_return_for_i64 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
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
    let n_max = *a_n.iter().max().unwrap();
    let v_result = [n_max as u8];

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
pub struct O_n_max_return_for_i8 {
    pub n_max: i8,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_i8(a_n: &[i8]) -> O_n_max_return_for_i8 {
    let n_max = *a_n.iter().max().unwrap();
    let v_result = [n_max as i8];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_i8 {
            n_max: v_result[0],
            n_max_nor: (v_result[0]) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
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
    let n_max = *a_n.iter().max().unwrap();
    let v_result = [n_max as u16];

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
pub struct O_n_max_return_for_i16 {
    pub n_max: i16,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_i16(a_n: &[i16]) -> O_n_max_return_for_i16 {
    let n_max = *a_n.iter().max().unwrap();
    let v_result = [n_max as i16];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_i16 {
            n_max: v_result[0],
            n_max_nor: (v_result[0]) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
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
    let n_max = *a_n.iter().max().unwrap();
    let v_result = [n_max as u32];

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
pub struct O_n_max_return_for_i32 {
    pub n_max: i32,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_i32(a_n: &[i32]) -> O_n_max_return_for_i32 {
    let n_max = *a_n.iter().max().unwrap();
    let v_result = [n_max as i32];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_i32 {
            n_max: v_result[0],
            n_max_nor: (v_result[0]) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
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
    let n_max = *a_n.iter().max().unwrap();
    let v_result = [n_max as u64];

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
pub struct O_n_max_return_for_i64 {
    pub n_max: i64,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_for_i64(a_n: &[i64]) -> O_n_max_return_for_i64 {
    let n_max = *a_n.iter().max().unwrap();
    let v_result = [n_max as i64];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_return_for_i64 {
            n_max: v_result[0],
            n_max_nor: (v_result[0]) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
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
pub struct O_n_min_n_max_return_for_i8 {
    pub n_min: i8,
    pub n_min_nor: f64,
    pub n_max: i8,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_i8(a_n: &[i8]) -> O_n_min_n_max_return_for_i8 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_i8 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_max: v_result[1],
            n_max_nor: (v_result[1]) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
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
pub struct O_n_min_n_max_return_for_i16 {
    pub n_min: i16,
    pub n_min_nor: f64,
    pub n_max: i16,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_i16(a_n: &[i16]) -> O_n_min_n_max_return_for_i16 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_i16 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_max: v_result[1],
            n_max_nor: (v_result[1]) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
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
pub struct O_n_min_n_max_return_for_i32 {
    pub n_min: i32,
    pub n_min_nor: f64,
    pub n_max: i32,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_i32(a_n: &[i32]) -> O_n_min_n_max_return_for_i32 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_i32 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_max: v_result[1],
            n_max_nor: (v_result[1]) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
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
pub struct O_n_min_n_max_return_for_i64 {
    pub n_min: i64,
    pub n_min_nor: f64,
    pub n_max: i64,
    pub n_max_nor: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_for_i64(a_n: &[i64]) -> O_n_min_n_max_return_for_i64 {
    let v_init = vec![a_n[0], a_n[0]];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0].min(v_in_fold);
        v_acc[1] = v_acc[1].max(v_in_fold);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_return_for_i64 {
            n_min: v_result[0],
            n_min_nor: (v_result[0]) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_max: v_result[1],
            n_max_nor: (v_result[1]) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
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
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_u8(a_n: &[u8]) -> O_n_mean_return_for_u8 {
    let n_sum: u64 = a_n.iter().map(|&val| val as u64).sum();
    let n_mean = (n_sum / a_n.len() as u64) as f64;
    let v_result = [n_mean as f64];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_u8 {
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_i8 {
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_i8(a_n: &[i8]) -> O_n_mean_return_for_i8 {
    let n_sum: i64 = a_n.iter().map(|&val| val as i64).sum();
    let n_mean = (n_sum / a_n.len() as i64) as f64;
    let v_result = [n_mean as f64];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_i8 {
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_u16 {
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_u16(a_n: &[u16]) -> O_n_mean_return_for_u16 {
    let n_sum: u64 = a_n.iter().map(|&val| val as u64).sum();
    let n_mean = (n_sum / a_n.len() as u64) as f64;
    let v_result = [n_mean as f64];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_u16 {
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_i16 {
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_i16(a_n: &[i16]) -> O_n_mean_return_for_i16 {
    let n_sum: i64 = a_n.iter().map(|&val| val as i64).sum();
    let n_mean = (n_sum / a_n.len() as i64) as f64;
    let v_result = [n_mean as f64];

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_i16 {
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_u32 {
    pub n_mean: f64,
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
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_i32 {
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_i32(a_n: &[i32]) -> O_n_mean_return_for_i32 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_i32 {
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_u64 {
    pub n_mean: f64,
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
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_i64 {
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_for_i64(a_n: &[i64]) -> O_n_mean_return_for_i64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_return_for_i64 {
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_f32 {
    pub n_mean: f64,
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
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_return_for_f64 {
    pub n_mean: f64,
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
            n_mean: v_result[0],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_i8 {
    pub n_min: i8,
    pub n_min_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_i8 {
    pub n_min: i8,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_i8(a_n: &[i8]) -> O_n_min_n_mean_return_for_i8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_i8 = O_n_min_n_mean_accumulator_for_i8 {
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
        a_o.push(O_n_min_n_mean_return_for_i8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_u16 {
    pub n_min: u16,
    pub n_min_nor: f64,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_i16 {
    pub n_min: i16,
    pub n_min_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_i16 {
    pub n_min: i16,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_i16(a_n: &[i16]) -> O_n_min_n_mean_return_for_i16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_i16 = O_n_min_n_mean_accumulator_for_i16 {
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
        a_o.push(O_n_min_n_mean_return_for_i16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_u32 {
    pub n_min: u32,
    pub n_min_nor: f64,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_i32 {
    pub n_min: i32,
    pub n_min_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_i32 {
    pub n_min: i32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_i32(a_n: &[i32]) -> O_n_min_n_mean_return_for_i32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_i32 = O_n_min_n_mean_accumulator_for_i32 {
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
        a_o.push(O_n_min_n_mean_return_for_i32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_u64 {
    pub n_min: u64,
    pub n_min_nor: f64,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_i64 {
    pub n_min: i64,
    pub n_min_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_accumulator_for_i64 {
    pub n_min: i64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_for_i64(a_n: &[i64]) -> O_n_min_n_mean_return_for_i64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_accumulator_for_i64 = O_n_min_n_mean_accumulator_for_i64 {
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
        a_o.push(O_n_min_n_mean_return_for_i64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_f32 {
    pub n_min: f32,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_return_for_f64 {
    pub n_min: f64,
    pub n_mean: f64,
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
            n_mean: v_result[1],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_u8 {
    pub n_max: u8,
    pub n_max_nor: f64,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_i8 {
    pub n_max: i8,
    pub n_max_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_i8 {
    pub n_max: i8,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_i8(a_n: &[i8]) -> O_n_max_n_mean_return_for_i8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_i8 = O_n_max_n_mean_accumulator_for_i8 {
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
        a_o.push(O_n_max_n_mean_return_for_i8 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_u16 {
    pub n_max: u16,
    pub n_max_nor: f64,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_i16 {
    pub n_max: i16,
    pub n_max_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_i16 {
    pub n_max: i16,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_i16(a_n: &[i16]) -> O_n_max_n_mean_return_for_i16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_i16 = O_n_max_n_mean_accumulator_for_i16 {
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
        a_o.push(O_n_max_n_mean_return_for_i16 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_u32 {
    pub n_max: u32,
    pub n_max_nor: f64,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_i32 {
    pub n_max: i32,
    pub n_max_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_i32 {
    pub n_max: i32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_i32(a_n: &[i32]) -> O_n_max_n_mean_return_for_i32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_i32 = O_n_max_n_mean_accumulator_for_i32 {
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
        a_o.push(O_n_max_n_mean_return_for_i32 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_u64 {
    pub n_max: u64,
    pub n_max_nor: f64,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_i64 {
    pub n_max: i64,
    pub n_max_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_accumulator_for_i64 {
    pub n_max: i64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_for_i64(a_n: &[i64]) -> O_n_max_n_mean_return_for_i64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_accumulator_for_i64 = O_n_max_n_mean_accumulator_for_i64 {
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
        a_o.push(O_n_max_n_mean_return_for_i64 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_f32 {
    pub n_max: f32,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_return_for_f64 {
    pub n_max: f64,
    pub n_mean: f64,
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
            n_mean: v_result[1],
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
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_i8 {
    pub n_min: i8,
    pub n_min_nor: f64,
    pub n_max: i8,
    pub n_max_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_i8 {
    pub n_min: i8,
    pub n_max: i8,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_i8(a_n: &[i8]) -> O_n_min_n_max_n_mean_return_for_i8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_i8 = O_n_min_n_max_n_mean_accumulator_for_i8 {
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
        a_o.push(O_n_min_n_max_n_mean_return_for_i8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
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
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_i16 {
    pub n_min: i16,
    pub n_min_nor: f64,
    pub n_max: i16,
    pub n_max_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_i16 {
    pub n_min: i16,
    pub n_max: i16,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_i16(a_n: &[i16]) -> O_n_min_n_max_n_mean_return_for_i16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_i16 =
        O_n_min_n_max_n_mean_accumulator_for_i16 {
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
        a_o.push(O_n_min_n_max_n_mean_return_for_i16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
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
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_i32 {
    pub n_min: i32,
    pub n_min_nor: f64,
    pub n_max: i32,
    pub n_max_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_i32 {
    pub n_min: i32,
    pub n_max: i32,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_i32(a_n: &[i32]) -> O_n_min_n_max_n_mean_return_for_i32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_i32 =
        O_n_min_n_max_n_mean_accumulator_for_i32 {
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
        a_o.push(O_n_min_n_max_n_mean_return_for_i32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
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
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_i64 {
    pub n_min: i64,
    pub n_min_nor: f64,
    pub n_max: i64,
    pub n_max_nor: f64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_accumulator_for_i64 {
    pub n_min: i64,
    pub n_max: i64,
    pub n_mean: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_for_i64(a_n: &[i64]) -> O_n_min_n_max_n_mean_return_for_i64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_accumulator_for_i64 =
        O_n_min_n_max_n_mean_accumulator_for_i64 {
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
        a_o.push(O_n_min_n_max_n_mean_return_for_i64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_f32 {
    pub n_min: f32,
    pub n_max: f32,
    pub n_mean: f64,
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
            n_mean: v_result.n_mean,
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_return_for_f64 {
    pub n_min: f64,
    pub n_max: f64,
    pub n_mean: f64,
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
            n_mean: v_result[2],
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_u8 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_u8(a_n: &[u8]) -> O_n_mean_n_variance_return_for_u8 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_u8 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_i8 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_i8(a_n: &[i8]) -> O_n_mean_n_variance_return_for_i8 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_i8 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_u16 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_u16(a_n: &[u16]) -> O_n_mean_n_variance_return_for_u16 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_u16 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_i16 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_i16(a_n: &[i16]) -> O_n_mean_n_variance_return_for_i16 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_i16 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_u32 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_u32(a_n: &[u32]) -> O_n_mean_n_variance_return_for_u32 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_u32 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_i32 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_i32(a_n: &[i32]) -> O_n_mean_n_variance_return_for_i32 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_i32 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_u64 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_u64(a_n: &[u64]) -> O_n_mean_n_variance_return_for_u64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_u64 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_i64 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_i64(a_n: &[i64]) -> O_n_mean_n_variance_return_for_i64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_i64 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_f32 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_f32(a_n: &[f32]) -> O_n_mean_n_variance_return_for_f32 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_f32 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_mean_n_variance_return_for_f64 {
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_mean_n_variance_for_f64(a_n: &[f64]) -> O_n_mean_n_variance_return_for_f64 {
    let n_len = a_n.len() as f64;

    let v_init = vec![0.0, 0.0];

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc[0] = v_acc[0] + (v_in_fold as f64 / n_len);
        v_acc[1] = v_acc[0] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_mean_n_variance_return_for_f64 {
            n_mean: v_result[0],
            n_variance: (v_result[1] / n_len) - v_result[0].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_u8 {
    pub n_min: u8,
    pub n_min_nor: f64,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_u8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u8::MAX - u8::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_i8 {
    pub n_min: i8,
    pub n_min_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_i8 {
    pub n_min: i8,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_i8(a_n: &[i8]) -> O_n_min_n_mean_n_variance_return_for_i8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_i8 =
        O_n_min_n_mean_n_variance_accumulator_for_i8 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_i8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_u16 {
    pub n_min: u16,
    pub n_min_nor: f64,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_u16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u16::MAX - u16::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_i16 {
    pub n_min: i16,
    pub n_min_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_i16 {
    pub n_min: i16,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_i16(
    a_n: &[i16],
) -> O_n_min_n_mean_n_variance_return_for_i16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_i16 =
        O_n_min_n_mean_n_variance_accumulator_for_i16 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_i16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_u32 {
    pub n_min: u32,
    pub n_min_nor: f64,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_u32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u32::MAX - u32::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_i32 {
    pub n_min: i32,
    pub n_min_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_i32 {
    pub n_min: i32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_i32(
    a_n: &[i32],
) -> O_n_min_n_mean_n_variance_return_for_i32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_i32 =
        O_n_min_n_mean_n_variance_accumulator_for_i32 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_i32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_u64 {
    pub n_min: u64,
    pub n_min_nor: f64,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_u64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u64::MAX - u64::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_i64 {
    pub n_min: i64,
    pub n_min_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_accumulator_for_i64 {
    pub n_min: i64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_mean_n_variance_for_i64(
    a_n: &[i64],
) -> O_n_min_n_mean_n_variance_return_for_i64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_mean_n_variance_accumulator_for_i64 =
        O_n_min_n_mean_n_variance_accumulator_for_i64 {
            n_min: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_i64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_f32 {
    pub n_min: f32,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_f32 {
            n_min: v_result.n_min,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_mean_n_variance_return_for_f64 {
    pub n_min: f64,
    pub n_mean: f64,
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
        v_acc[2] = v_acc[1] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_mean_n_variance_return_for_f64 {
            n_min: v_result[0],
            n_mean: v_result[1],
            n_variance: (v_result[2] / n_len) - v_result[1].powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_u8 {
    pub n_max: u8,
    pub n_max_nor: f64,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_u8 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u8::MAX - u8::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_i8 {
    pub n_max: i8,
    pub n_max_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_i8 {
    pub n_max: i8,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_i8(a_n: &[i8]) -> O_n_max_n_mean_n_variance_return_for_i8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_i8 =
        O_n_max_n_mean_n_variance_accumulator_for_i8 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_i8 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_u16 {
    pub n_max: u16,
    pub n_max_nor: f64,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_u16 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u16::MAX - u16::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_i16 {
    pub n_max: i16,
    pub n_max_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_i16 {
    pub n_max: i16,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_i16(
    a_n: &[i16],
) -> O_n_max_n_mean_n_variance_return_for_i16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_i16 =
        O_n_max_n_mean_n_variance_accumulator_for_i16 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_i16 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_u32 {
    pub n_max: u32,
    pub n_max_nor: f64,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_u32 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u32::MAX - u32::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_i32 {
    pub n_max: i32,
    pub n_max_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_i32 {
    pub n_max: i32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_i32(
    a_n: &[i32],
) -> O_n_max_n_mean_n_variance_return_for_i32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_i32 =
        O_n_max_n_mean_n_variance_accumulator_for_i32 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_i32 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_u64 {
    pub n_max: u64,
    pub n_max_nor: f64,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_u64 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u64::MAX - u64::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_i64 {
    pub n_max: i64,
    pub n_max_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_accumulator_for_i64 {
    pub n_max: i64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_max_n_mean_n_variance_for_i64(
    a_n: &[i64],
) -> O_n_max_n_mean_n_variance_return_for_i64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_max_n_mean_n_variance_accumulator_for_i64 =
        O_n_max_n_mean_n_variance_accumulator_for_i64 {
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_i64 {
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_f32 {
    pub n_max: f32,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_f32 {
            n_max: v_result.n_max,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_max_n_mean_n_variance_return_for_f64 {
    pub n_max: f64,
    pub n_mean: f64,
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
        v_acc[2] = v_acc[1] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_max_n_mean_n_variance_return_for_f64 {
            n_max: v_result[0],
            n_mean: v_result[1],
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
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_u8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u8::MAX - u8::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u8::MAX - u8::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_i8 {
    pub n_min: i8,
    pub n_min_nor: f64,
    pub n_max: i8,
    pub n_max_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_i8 {
    pub n_min: i8,
    pub n_max: i8,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_i8(
    a_n: &[i8],
) -> O_n_min_n_max_n_mean_n_variance_return_for_i8 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_i8 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_i8 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_i8 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i8::MAX as i128 - i8::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
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
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_u16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u16::MAX - u16::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u16::MAX - u16::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_i16 {
    pub n_min: i16,
    pub n_min_nor: f64,
    pub n_max: i16,
    pub n_max_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_i16 {
    pub n_min: i16,
    pub n_max: i16,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_i16(
    a_n: &[i16],
) -> O_n_min_n_max_n_mean_n_variance_return_for_i16 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_i16 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_i16 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_i16 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i16::MAX as i128 - i16::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
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
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_u32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u32::MAX - u32::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u32::MAX - u32::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_i32 {
    pub n_min: i32,
    pub n_min_nor: f64,
    pub n_max: i32,
    pub n_max_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_i32 {
    pub n_min: i32,
    pub n_max: i32,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_i32(
    a_n: &[i32],
) -> O_n_min_n_max_n_mean_n_variance_return_for_i32 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_i32 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_i32 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_i32 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i32::MAX as i128 - i32::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
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
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_u64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (u64::MAX - u64::MIN) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (u64::MAX - u64::MIN) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_i64 {
    pub n_min: i64,
    pub n_min_nor: f64,
    pub n_max: i64,
    pub n_max_nor: f64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_accumulator_for_i64 {
    pub n_min: i64,
    pub n_max: i64,
    pub n_mean: f64,
    pub n_variance: f64,
}

#[wasm_bindgen]
pub fn f_o_n_min_n_max_n_mean_n_variance_for_i64(
    a_n: &[i64],
) -> O_n_min_n_max_n_mean_n_variance_return_for_i64 {
    let n_len = a_n.len() as f64;

    let v_init: O_n_min_n_max_n_mean_n_variance_accumulator_for_i64 =
        O_n_min_n_max_n_mean_n_variance_accumulator_for_i64 {
            n_min: a_n[0],
            n_max: a_n[0],
            n_mean: 0.0,
            n_variance: 0.0,
        };

    let v_result = a_n.iter().fold(v_init, |mut v_acc, &v_in_fold| {
        v_acc.n_min = v_acc.n_min.min(v_in_fold);
        v_acc.n_max = v_acc.n_max.max(v_in_fold);
        v_acc.n_mean = v_acc.n_mean + (v_in_fold as f64 / n_len);
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_i64 {
            n_min: v_result.n_min,
            n_min_nor: (v_result.n_min) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_max: v_result.n_max,
            n_max_nor: (v_result.n_max) as f64 / (i64::MAX as i128 - i64::MIN as i128 + 1) as f64,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_f32 {
    pub n_min: f32,
    pub n_max: f32,
    pub n_mean: f64,
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
        v_acc.n_variance = v_acc.n_mean + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_f32 {
            n_min: v_result.n_min,
            n_max: v_result.n_max,
            n_mean: v_result.n_mean,
            n_variance: (v_result.n_variance / n_len) - v_result.n_mean.powi(2),
        });
    }

    a_o.remove(0)
}

#[wasm_bindgen]
pub struct O_n_min_n_max_n_mean_n_variance_return_for_f64 {
    pub n_min: f64,
    pub n_max: f64,
    pub n_mean: f64,
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
        v_acc[3] = v_acc[2] + (v_in_fold as f64).powi(2);
        v_acc
    });

    let mut a_o = Vec::new();
    for n_channel in 0..1 {
        a_o.push(O_n_min_n_max_n_mean_n_variance_return_for_f64 {
            n_min: v_result[0],
            n_max: v_result[1],
            n_mean: v_result[2],
            n_variance: (v_result[3] / n_len) - v_result[2].powi(2),
        });
    }

    a_o.remove(0)
}

