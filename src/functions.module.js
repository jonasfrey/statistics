
//this code was automatically generated
import * as o_wasm from "./../pkg/wasm_speed_test.js";
await o_wasm.default();


                let f_o_n_min = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_min_for_${v_s_type}`
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
                    f_o_n_min
                }
            

                let f_o_n_max = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_max_for_${v_s_type}`
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
                    f_o_n_max
                }
            

                let f_o_n_min_n_max = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_min_n_max_for_${v_s_type}`
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
                    f_o_n_min_n_max
                }
            

                let f_o_n_mean = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_mean_for_${v_s_type}`
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
                    f_o_n_mean
                }
            

                let f_o_n_min_n_mean = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_min_n_mean_for_${v_s_type}`
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
                    f_o_n_min_n_mean
                }
            

                let f_o_n_max_n_mean = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_max_n_mean_for_${v_s_type}`
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
                    f_o_n_max_n_mean
                }
            

                let f_o_n_min_n_max_n_mean = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_min_n_max_n_mean_for_${v_s_type}`
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
                    f_o_n_min_n_max_n_mean
                }
            

                let f_o_n_mean_n_variance = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_mean_n_variance_for_${v_s_type}`
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
                    f_o_n_mean_n_variance
                }
            

                let f_o_n_min_n_mean_n_variance = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_min_n_mean_n_variance_for_${v_s_type}`
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
                    f_o_n_min_n_mean_n_variance
                }
            

                let f_o_n_max_n_mean_n_variance = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_max_n_mean_n_variance_for_${v_s_type}`
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
                    f_o_n_max_n_mean_n_variance
                }
            

                let f_o_n_min_n_max_n_mean_n_variance = function(a_v){
                    if(a_v.length == 0){
                        throw Error('array cannot be empty');
                    }
                    let v_s_type = false
                    v_s_type = (a_v instanceof Int8Array) ? 'i8': v_s_type;
v_s_type = (a_v instanceof Uint8Array) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Uint8ClampedArray) ? 'u8': v_s_type;
v_s_type = (a_v instanceof Int16Array) ? 'i16': v_s_type;
v_s_type = (a_v instanceof Uint16Array) ? 'u16': v_s_type;
v_s_type = (a_v instanceof Int32Array) ? 'i32': v_s_type;
v_s_type = (a_v instanceof Uint32Array) ? 'u32': v_s_type;
v_s_type = (a_v instanceof Float32Array) ? 'f32': v_s_type;
v_s_type = (a_v instanceof Float64Array) ? 'f64': v_s_type;
v_s_type = (a_v instanceof BigInt64Array) ? 'i64': v_s_type;
v_s_type = (a_v instanceof BigUint64Array) ? 'u64': v_s_type;
    
                    if(!v_s_type){
                        throw Error(`array type not allowed, please pass a typed array an instance of one of the following arrays ${Object.values(o_s_type_s_name_typed_array)}`)
                    }
                    let s_name_function = `f_o_n_min_n_max_n_mean_n_variance_for_${v_s_type}`
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
                    f_o_n_min_n_max_n_mean_n_variance
                }
            
