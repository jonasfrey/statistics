import {
  f_measure_time
} from "https://deno.land/x/date_functions@1.3/mod.js"
import { readAll } from "https://deno.land/std/streams/mod.ts";
if(Deno.args[0]== 'f_n_min'){

  const content = await readAll(Deno.stdin);
  // console.log(content)
  // let o_reader = Deno.stdin.readableStream;
  
  
  // console.log(o_reader);
  
  let f_n_min = function(a){
      var n_min = a[0];
      for(var n = 0; n<a.length; n++){
          if(a[n] < n_min) {
              n_min = a[n]
          }
      }
      return n_min;
  }
  // console.log(f_n_min(content))
  await Deno.stdout.write(new Uint8Array([f_n_min(content)]).buffer)
  Deno.exit(0)
  // console.log('afds');
  // Deno.exit(0)
}
let a_n_u8__512000_random_bytes =  await Deno.readFile('./random_test_data/512000_random_bytes');
// let a_n_u8__1280000_random_bytes =  await Deno.readFile('./random_test_data/1280000_random_bytes');

let a_v_small = new Uint8Array(
  new Array(
    10_000
    ).fill(0)
    .map(
      ()=>{
      return parseInt(Math.random()*255)
    }
    )
  )
  var a_v = new Uint8Array(a_n_u8__512000_random_bytes.buffer)



let f_n_min = function(a){
  var n_min = a[0];
  for(var n = 0; n<a.length; n++){
      if(a[n] < n_min) {
          n_min = a[n]
      }
  }
  return n_min;
}
// a_v = new Uint8Array(new Array(2000*2000*5).fill(0).map(
//     ()=>{return parseInt(Math.random()*255)}
// ))

// a_v = a_v_small
a_v = a_n_u8__512000_random_bytes
f_measure_time("serial")
console.log(f_n_min(a_v))
f_measure_time()

f_measure_time("multiprocessing")
let f_a_n_min__from_spawned_process = async function(a){

    // let a = new Uint8Array([10,20,30,40,55, 7]);
    const command = new Deno.Command(Deno.execPath(), {
        args: [
        //   "deno run /home/ubuntuuser/code/statistics/f_n_min.js",
          "run", 
          import.meta.url.split("//").pop(), 
          "f_n_min"
        ],
        stdin: "piped",
        stdout: "piped",
        stderr: 'piped'
      });
      const child = command.spawn();
      let o_writer = await (child.stdin.getWriter())
    await o_writer.write(a.buffer)
      // Write the data to the stdin of the process
    //     await command.stdin.write(a.buffer);
    await o_writer.close();
    //   // open a file and pipe the subprocess output to it.
    
      const { code, stdout, stderr } = await child.output();
      return stdout
      // let o_tdec = new TextDecoder();
    // console.log(stdout)
    //   console.log(`out${o_tdec.decode(stdout)}`)
}

let n_parallel = 4;
let n_idx_per_parallel = a_v.length / n_parallel;

let a_n_min2 = await Promise.all([
    ...new Array(n_parallel).fill(0).map(
        (v, n_idx)=>{
            let n_idx_start = n_idx*n_idx_per_parallel
            let n_idx_end = Math.min(
                a_v.length,
                n_idx*n_idx_per_parallel + n_idx+1*n_idx_per_parallel)
              // console.log(`start ${n_idx_start}, ${n_idx_end}`)
            return f_a_n_min__from_spawned_process(a_v.subarray(n_idx_start,n_idx_end))
        }
    )
]
)
f_measure_time()

console.log(a_n_min2)