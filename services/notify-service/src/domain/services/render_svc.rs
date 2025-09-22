use tera::{Context, Tera};
use anyhow::Context as _;

pub struct RenderSvc {
    tera: std::sync::Mutex<Tera>,
}

impl RenderSvc {
    pub fn new() -> Self {
        Self {
            tera: std::sync::Mutex::new(Tera::default()),
        }
    }

    pub fn render(&self, tmpl: &str, vars: &serde_json::Value) -> anyhow::Result<String> {
        let mut ctx = Context::new();
        if let Some(obj) = vars.as_object() {
            for (k, v) in obj {
                ctx.insert(k, v);
            }
        }
        let mut tera = self.tera.lock().unwrap();
        Ok(tera.render_str(tmpl, &ctx).context("render fail")?)
    }
}
