use std::io;
use std::fs;

use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

wasi::http::proxy::export!(Component);

struct Component;

#[allow(unused)]
const HTML_BODY: &str = r#"<html>
    <head>
        <title>Hello from WebAssembly!</title>
    </head>
    <body>
      {{ENV_VARS}}
    </body>
</html>"#;

impl wasi::exports::http::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let hdrs = Fields::new();
        // let variables = format!("{:?}", get_environment());
        // let mesg = String::from(HTML_BODY).replace("{{ENV_VARS}}", &variables);
        // let mesg = fs::read_to_string("/tmp/hello").expect("failed to read file");
        let mesg_vec = fs::read_dir("/shenanigans").unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>().unwrap();

        let mesg = mesg_vec.into_iter().map(|boop| {
            boop.into_os_string().into_string().unwrap_or(String::from(""))
        }).collect::<Vec<String>>().join("\n");

        let _try = hdrs.set(
            &"Content-Type".to_string(),
            &[b"text/html; charset=utf-8".to_vec()],
        );
        let _try = hdrs.set(
            &"Content-Length".to_string(),
            &[mesg.len().to_string().as_bytes().to_vec()],
        );

        let resp = OutgoingResponse::new(hdrs);
        resp.set_status_code(200).unwrap();

        let body = resp.body().unwrap();
        ResponseOutparam::set(response_out, Ok(resp));

        let out = body.write().unwrap();
        out.blocking_write_and_flush(mesg.as_bytes()).unwrap();
        drop(out);

        OutgoingBody::finish(body, None).unwrap();
    }
}
