
use crate::{Result, handler::{HtmlView, render, log_error}, view::{frontend::index::Index}};


pub async fn index() -> Result<HtmlView> {
    let handle_name = "frontend/index/index";
    let tmpl = Index{};
    render(tmpl).map_err(log_error(handle_name))
}