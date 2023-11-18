const port = 8081;

const handler = async (request) => {
    let o_url = new URL(request.url);
    let s_path = o_url.pathname;
    let s_type = 'text/plain'

    if(o_url.pathname == '/'){
        s_path = 'client.html';
    }
    let o ={
        'html': "text/html", 
        'js': 'text/javascript', 
        'wasm': "application/wasm"
    }
    s_type = o[s_path.split('.').pop()];
    if(!s_type){
        s_type = 'application/binary'
    }
    let a_n_u8 = await Deno.readFile(`.`+s_path);
  const body = a_n_u8;

  return new Response(body, { status: 200, headers: {
    'Content-type': s_type
  }
 });
};

console.log(`HTTP server running. Access it at: http://localhost:8080/`);
Deno.serve({ port }, handler);