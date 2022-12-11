mod authentication;

use std::str::FromStr;

use authentication::is_authenticated;
use console_error_panic_hook::set_once as set_panic_hook;
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
struct UrlPayload {
    pub url: String,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _context: Context) -> Result<Response> {
    set_console_error_hook();

    let router = Router::new();

    router
        .get("/", |_req, _ctx| Response::ok("owo"))
        .get_async("/:slug", |_req, ctx| async move {
            match ctx.param("slug") {
                Some(slug) => {
                    let to = ctx
                        .kv("url-slugs")?
                        .get(slug)
                        .text()
                        .await?
                        .unwrap_or_default();

                    if to.is_empty() {
                        return Response::ok("UNKNOWN_SLUG");
                    }

                    Response::redirect(Url::from_str(&to)?)
                }
                None => Response::ok("NO_SLUG"),
            }
        })
        .post_async("/:slug", |mut req, ctx| async move {
            if !is_authenticated(&req, &ctx).await? {
                return Response::error("NOT_AUTHENTICATED", 403);
            }

            match ctx.param("slug") {
                Some(slug) => {
                    let payload = req.json::<UrlPayload>().await?;

                    ctx.kv("url-slugs")?
                        .put(slug, payload.url)?
                        .execute()
                        .await?;

                    Response::ok("OK")
                }
                None => Response::error("NO_SLUG", 400),
            }
        })
        .delete_async("/:slug", |req, ctx| async move {
            if !is_authenticated(&req, &ctx).await? {
                return Response::error("NOT_AUTHENTICATED", 403);
            }

            match ctx.param("slug") {
                Some(slug) => {
                    match ctx.kv("url-slugs")?.delete(slug).await {
                        Ok(_) => {}
                        Err(_) => return Response::error("UNKNOWN_SLUG", 400),
                    }

                    Response::ok("OK")
                }
                None => Response::ok("NO_SLUG"),
            }
        })
        .run(req, env)
        .await
}

fn set_console_error_hook() {
    set_panic_hook()
}
