use log::warn;
use proxy_wasm::{traits::*, types::*};
use serde::Deserialize;
use serde_json_wasm::de;

// -----------------------------------------------------------------------------
// Config
// -----------------------------------------------------------------------------

#[derive(Deserialize, Clone, Debug)]
struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    my_greeting: Option<String>,
}

// -----------------------------------------------------------------------------
// Root Context
// -----------------------------------------------------------------------------

struct MyFilterRoot {
    config: Option<Config>,
}

struct MyFilter {
    config: Config,
}

impl Context for MyFilterRoot {
//    fn on_http_call_response(
//        &mut self,
//        token_id: u32,
//        num_headers: usize,
//        body_size: usize,
//        _num_trailers: usize,
//    ) {
//    }
//
//    fn on_done(&mut self) -> bool {
//        true
//    }
}

impl RootContext for MyFilterRoot {
//    fn on_vm_start(&mut self, config_size: usize) -> bool {
//        true
//    }
//
//    fn on_tick(&mut self) {
//    }

    fn on_configure(&mut self, _config_size: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            match de::from_slice::<Config>(&config_bytes) {
                Ok(config) => {
                    self.config = Some(config);

                    true
                }
                Err(err) => {
                    warn!(
                        "on_configure: failed parsing configuration: {}: {}",
                        String::from_utf8(config_bytes).unwrap(), err
                    );

                    false
                }
            }
        } else {
            warn!("on_configure: failed getting configuration");

            false
        }
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        if let Some(config) = &self.config {
            Some(Box::new(MyFilter {
                config: config.clone(),
            }))
        } else {
            None
        }
    }
}

// -----------------------------------------------------------------------------
// Plugin Context
// -----------------------------------------------------------------------------

impl Context for MyFilter {
//    fn on_http_call_response(
//        &mut self,
//        token_id: u32,
//        nheaders: usize,
//        body_size: usize,
//        _num_trailers: usize,
//    ) {}
}

impl HttpContext for MyFilter {
   fn on_http_request_headers(&mut self, nheaders: usize, _eof: bool) -> Action {
       Action::Continue
   }
//
//    fn on_http_request_body(&mut self, body_size: usize, eof: bool) -> Action {
//        Action::Continue
//    }
//
    fn on_http_response_headers(&mut self, _nheaders: usize, _eof: bool) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            // if name is status then dump it out
            println!("#ha -> {}: {}", name, value);
        }
        println!("{}", self.get_http_response_header(":status").unwrap()); 
        let new_body: String = format!("status: {}\n", self.get_http_response_header(":status").unwrap());
        let body_size = new_body.len();
        // self.set_http_response_body(0, body_size, &new_body.into_bytes());
        self.set_http_response_header("content-length", None);
        Action::Continue
    }
//
    fn on_http_response_body(&mut self, body_size: usize, eof: bool) -> Action {
        println!("Test message");
        let new_body: String = format!("status: {}\n", self.get_http_response_header(":status").unwrap());
        self.set_http_response_body(0, body_size, &new_body.into_bytes());
        Action::Continue
    }
//
//    fn on_log(&mut self) {
//    }
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(MyFilterRoot {
            config: None,
        })
    });
}}
