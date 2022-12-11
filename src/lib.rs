use console_error_panic_hook::set_once as set_panic_hook;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _context: Context) -> Result<Response> {
    set_console_error_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("owo"))
        .run(req, env)
        .await
}

fn set_console_error_hook() {
    set_panic_hook()
}
