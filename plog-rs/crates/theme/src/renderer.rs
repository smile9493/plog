//! 模板渲染引擎
//!
//! 基于 Tera 实现 Rust 主题模板渲染

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Serialize;
use tera::Tera;

use crate::ThemeError;

/// 模板渲染器
pub struct TemplateRenderer {
    tera: Tera,
    template_dir: PathBuf,
}

impl TemplateRenderer {
    /// 创建新的模板渲染器
    pub fn new(template_dir: impl Into<PathBuf>) -> Result<Self, ThemeError> {
        let template_dir = template_dir.into();
        let pattern = template_dir.join("**/*.html").to_string_lossy().to_string();

        let mut tera = Tera::new(&pattern)
            .map_err(|e| ThemeError::ParseError(format!("Failed to parse templates: {}", e)))?;

        // 注册自定义过滤器
        tera.register_filter("truncate", truncate_filter);
        tera.register_filter("date_format", date_format_filter);
        tera.register_function("url_for", url_for_function);

        Ok(Self { tera, template_dir })
    }

    /// 渲染模板
    pub fn render(
        &self,
        template_name: &str,
        context: &impl Serialize,
    ) -> Result<String, ThemeError> {
        let json = serde_json::to_value(context)
            .map_err(|e| ThemeError::ParseError(format!("Failed to serialize context: {}", e)))?;

        let mut tera_context = tera::Context::new();
        tera_context.insert("page", &json);

        // 也支持扁平化字段
        if let serde_json::Value::Object(map) = &json {
            for (key, value) in map {
                tera_context.insert(key, value);
            }
        }

        self.tera
            .render(template_name, &tera_context)
            .map_err(|e| ThemeError::ParseError(format!("Failed to render template: {}", e)))
    }

    /// 渲染字符串模板
    pub fn render_str(
        &self,
        template: &str,
        context: &impl Serialize,
    ) -> Result<String, ThemeError> {
        let json = serde_json::to_value(context)
            .map_err(|e| ThemeError::ParseError(format!("Failed to serialize context: {}", e)))?;

        let mut tera_context = tera::Context::new();
        tera_context.insert("page", &json);

        if let serde_json::Value::Object(map) = &json {
            for (key, value) in map {
                tera_context.insert(key, value);
            }
        }

        Tera::one_off(template, &tera_context, false)
            .map_err(|e| ThemeError::ParseError(format!("Failed to render string template: {}", e)))
    }

    /// 获取所有可用模板名称
    pub fn get_template_names(&self) -> Vec<&str> {
        self.tera.get_template_names().collect()
    }

    /// 检查模板是否存在
    pub fn has_template(&self, name: &str) -> bool {
        self.tera.get_template_names().any(|t| t == name)
    }
}

/// 截断过滤器
fn truncate_filter(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = tera::try_get_value!("truncate", "value", String, value);
    let length = args.get("length").and_then(|v| v.as_u64()).unwrap_or(100) as usize;

    if s.len() <= length {
        Ok(tera::Value::String(s))
    } else {
        Ok(tera::Value::String(format!("{}...", &s[..length])))
    }
}

/// 日期格式化过滤器
fn date_format_filter(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let s = tera::try_get_value!("date_format", "value", String, value);
    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d");

    // 尝试解析 Unix 时间戳
    if let Ok(timestamp) = s.parse::<i64>() {
        let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp, 0)
            .ok_or_else(|| tera::Error::msg("Invalid timestamp"))?;
        Ok(tera::Value::String(dt.format(format).to_string()))
    } else {
        Ok(tera::Value::String(s))
    }
}

/// URL 生成函数
fn url_for_function(args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("url_for requires a 'path' argument"))?;

    Ok(tera::Value::String(format!(
        "/{}",
        path.trim_start_matches('/')
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_render_simple_template() {
        let temp_dir = std::env::temp_dir().join("plog_theme_test");
        let templates_dir = temp_dir.join("templates");
        fs::create_dir_all(&templates_dir).unwrap();
        fs::write(templates_dir.join("hello.html"), "Hello, {{ name }}!").unwrap();

        let renderer = TemplateRenderer::new(&templates_dir).unwrap();
        let context = serde_json::json!({"name": "World"});
        let result = renderer.render("hello.html", &context).unwrap();

        assert_eq!(result, "Hello, World!");

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_render_with_page_context() {
        let temp_dir = std::env::temp_dir().join("plog_theme_test2");
        let templates_dir = temp_dir.join("templates");
        fs::create_dir_all(&templates_dir).unwrap();
        fs::write(
            templates_dir.join("post.html"),
            "<h1>{{ page.title }}</h1><p>{{ page.content }}</p>",
        )
        .unwrap();

        let renderer = TemplateRenderer::new(&templates_dir).unwrap();
        let context = serde_json::json!({
            "title": "Test Post",
            "content": "Hello World"
        });
        let result = renderer.render("post.html", &context).unwrap();

        assert!(result.contains("<h1>Test Post</h1>"));
        assert!(result.contains("<p>Hello World</p>"));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_truncate_filter() {
        let temp_dir = std::env::temp_dir().join("plog_theme_test3");
        let templates_dir = temp_dir.join("templates");
        fs::create_dir_all(&templates_dir).unwrap();
        fs::write(
            templates_dir.join("excerpt.html"),
            "{{ text | truncate(length=10) }}",
        )
        .unwrap();

        let renderer = TemplateRenderer::new(&templates_dir).unwrap();
        let context = serde_json::json!({"text": "This is a long text that should be truncated"});
        let result = renderer.render("excerpt.html", &context).unwrap();

        assert_eq!(result, "This is a ...");

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_has_template() {
        let temp_dir = std::env::temp_dir().join("plog_theme_test4");
        let templates_dir = temp_dir.join("templates");
        fs::create_dir_all(&templates_dir).unwrap();
        fs::write(templates_dir.join("index.html"), "Index").unwrap();

        let renderer = TemplateRenderer::new(&templates_dir).unwrap();

        assert!(renderer.has_template("index.html"));
        assert!(!renderer.has_template("missing.html"));

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_render_str() {
        let renderer = TemplateRenderer::new("/tmp/nonexistent").unwrap_or_else(|_| {
            TemplateRenderer::new(std::env::temp_dir().join("plog_empty")).unwrap()
        });

        let context = serde_json::json!({"name": "Rust"});
        let result = renderer.render_str("Hello, {{ name }}!", &context).unwrap();
        assert_eq!(result, "Hello, Rust!");
    }
}
