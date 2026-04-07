//! 主题迁移工具
//!
//! 帮助将 PHP 主题模板迁移到 Tera 模板语法

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 迁移报告
#[derive(Debug, Clone)]
pub struct MigrationReport {
    pub source_path: PathBuf,
    pub files: Vec<FileMigrationReport>,
    pub total_files: usize,
    pub migrated_files: usize,
    pub error_files: usize,
}

/// 单个文件的迁移报告
#[derive(Debug, Clone)]
pub struct FileMigrationReport {
    pub file_path: PathBuf,
    pub migrations: Vec<Migration>,
    pub has_errors: bool,
    pub error_message: Option<String>,
}

/// 单个迁移项
#[derive(Debug, Clone)]
pub struct Migration {
    pub line: usize,
    pub original: String,
    pub migrated: String,
    pub migration_type: MigrationType,
}

/// 迁移类型
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationType {
    Variable,
    Conditional,
    Loop,
    Function,
    Include,
    Comment,
    Echo,
}

impl MigrationType {
    fn as_str(&self) -> &str {
        match self {
            MigrationType::Variable => "variable",
            MigrationType::Conditional => "conditional",
            MigrationType::Loop => "loop",
            MigrationType::Function => "function",
            MigrationType::Include => "include",
            MigrationType::Comment => "comment",
            MigrationType::Echo => "echo",
        }
    }
}

/// 迁移分析器
pub struct ThemeMigrator;

impl ThemeMigrator {
    /// 分析目录中的 PHP 模板文件
    pub fn analyze(source_dir: impl Into<PathBuf>) -> Result<MigrationReport, String> {
        let source_dir = source_dir.into();
        let mut report = MigrationReport {
            source_path: source_dir.clone(),
            files: Vec::new(),
            total_files: 0,
            migrated_files: 0,
            error_files: 0,
        };

        if !source_dir.exists() {
            return Err(format!("Source directory not found: {:?}", source_dir));
        }

        let entries = Self::find_php_templates(&source_dir);
        report.total_files = entries.len();

        for file_path in entries {
            let file_report = Self::analyze_file(&file_path);
            if file_report.has_errors {
                report.error_files += 1;
            } else if !file_report.migrations.is_empty() {
                report.migrated_files += 1;
            }
            report.files.push(file_report);
        }

        Ok(report)
    }

    /// 分析单个文件
    fn analyze_file(file_path: &Path) -> FileMigrationReport {
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                return FileMigrationReport {
                    file_path: file_path.to_path_buf(),
                    migrations: Vec::new(),
                    has_errors: true,
                    error_message: Some(format!("Failed to read file: {}", e)),
                };
            }
        };

        let migrations = Self::analyze_content(&content);

        FileMigrationReport {
            file_path: file_path.to_path_buf(),
            migrations,
            has_errors: false,
            error_message: None,
        }
    }

    /// 分析文件内容
    fn analyze_content(content: &str) -> Vec<Migration> {
        let mut migrations = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            // PHP echo 变量: <?php echo $var; ?>
            if let Some(m) = Self::try_migrate_echo(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Echo,
                });
                continue;
            }

            // PHP 短标签变量: <?= $var ?>
            if let Some(m) = Self::try_migrate_short_echo(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Variable,
                });
                continue;
            }

            // PHP if 条件
            if let Some(m) = Self::try_migrate_if(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Conditional,
                });
                continue;
            }

            // PHP endif
            if let Some(m) = Self::try_migrate_endif(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Conditional,
                });
                continue;
            }

            // PHP foreach 循环
            if let Some(m) = Self::try_migrate_foreach(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Loop,
                });
                continue;
            }

            // PHP endforeach
            if let Some(m) = Self::try_migrate_endforeach(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Loop,
                });
                continue;
            }

            // PHP include
            if let Some(m) = Self::try_migrate_include(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Include,
                });
                continue;
            }

            // PHP 函数调用: <?php func($arg); ?>
            if let Some(m) = Self::try_migrate_function(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Function,
                });
                continue;
            }

            // PHP 注释: <?php // comment ?>
            if let Some(m) = Self::try_migrate_comment(line) {
                migrations.push(Migration {
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    migrated: m,
                    migration_type: MigrationType::Comment,
                });
            }
        }

        migrations
    }

    fn try_migrate_echo(line: &str) -> Option<String> {
        // Match: <?php echo $var; ?> or <?php echo $var; ?>
        let re = regex::Regex::new(r"<\?php\s+echo\s+\$(\w+)\s*;?\s*\?>").ok()?;
        let caps = re.captures(line)?;
        let var_name = caps.get(1)?.as_str();
        Some(format!("{{{{ {} }}}}", var_name))
    }

    fn try_migrate_short_echo(line: &str) -> Option<String> {
        let re = regex::Regex::new(r"<\?=\s*\$(\w+)\s*\?>").ok()?;
        let caps = re.captures(line)?;
        let var_name = caps.get(1)?.as_str();
        Some(format!("{{{{ {} }}}}", var_name))
    }

    fn try_migrate_if(line: &str) -> Option<String> {
        let re = regex::Regex::new(r"<\?php\s+if\s*\(\s*\$(\w+)\s*\)\s*:\s*\?>").ok()?;
        let caps = re.captures(line)?;
        let var_name = caps.get(1)?.as_str();
        Some(format!("{{% if {} %}}", var_name))
    }

    fn try_migrate_endif(line: &str) -> Option<String> {
        if line.contains("<?php endif; ?>") || line.contains("<?php endif;?>") {
            Some("{% endif %}".to_string())
        } else {
            None
        }
    }

    fn try_migrate_foreach(line: &str) -> Option<String> {
        // Match: <?php foreach ($items as $item): ?>
        let re =
            regex::Regex::new(r"<\?php\s+foreach\s*\(\s*\$(\w+)\s+as\s+\$(\w+)\s*\)\s*:\s*\?>")
                .ok()?;
        let caps = re.captures(line)?;
        let collection = caps.get(1)?.as_str();
        let item = caps.get(2)?.as_str();
        Some(format!("{{% for {} in {} %}}", item, collection))
    }

    fn try_migrate_endforeach(line: &str) -> Option<String> {
        if line.contains("<?php endforeach; ?>") || line.contains("<?php endforeach;?>") {
            Some("{% endfor %}".to_string())
        } else {
            None
        }
    }

    fn try_migrate_include(line: &str) -> Option<String> {
        // Match: <?php include 'header.php'; ?>
        let re = regex::Regex::new(r#"<\?php\s+include\s+['"]([^'"]+)['"]\s*;?\s*\?>"#).ok()?;
        let caps = re.captures(line)?;
        let file_path = caps.get(1)?.as_str();
        let template_name = file_path.trim_end_matches(".php");
        Some(format!("{{% include \"{}.html\" %}}", template_name))
    }

    fn try_migrate_function(line: &str) -> Option<String> {
        // Match: <?php func($arg); ?>
        let re = regex::Regex::new(r"<\?php\s+(\w+)\s*\(\s*\$(\w+)\s*\)\s*;?\s*\?>").ok()?;
        let caps = re.captures(line)?;
        let func_name = caps.get(1)?.as_str();
        let arg = caps.get(2)?.as_str();
        Some(format!("{{{{ {}({}) }}}}", func_name, arg))
    }

    fn try_migrate_comment(line: &str) -> Option<String> {
        // Match: <?php // comment ?>
        let re = regex::Regex::new(r"<\?php\s+//\s*(.+?)\s*\?>").ok()?;
        let caps = re.captures(line)?;
        let comment = caps.get(1)?.as_str();
        Some(format!("{{# {} #}}", comment))
    }

    /// 查找所有 PHP 模板文件
    fn find_php_templates(dir: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(Self::find_php_templates(&path));
                } else if let Some(ext) = path.extension() {
                    if ext == "php" {
                        files.push(path);
                    }
                }
            }
        }
        files
    }

    /// 生成迁移摘要报告
    pub fn generate_summary(report: &MigrationReport) -> String {
        let mut output = String::new();
        output.push_str("=== Theme Migration Report ===\n\n");
        output.push_str(&format!("Source: {:?}\n", report.source_path));
        output.push_str(&format!("Total files: {}\n", report.total_files));
        output.push_str(&format!(
            "Files needing migration: {}\n",
            report.migrated_files
        ));
        output.push_str(&format!("Files with errors: {}\n\n", report.error_files));

        let mut type_counts: HashMap<&str, usize> = HashMap::new();
        for file in &report.files {
            for migration in &file.migrations {
                *type_counts
                    .entry(migration.migration_type.as_str())
                    .or_insert(0) += 1;
            }
        }

        if !type_counts.is_empty() {
            output.push_str("Migration types:\n");
            for (type_name, count) in &type_counts {
                output.push_str(&format!("  {}: {}\n", type_name, count));
            }
            output.push('\n');
        }

        for file in &report.files {
            if !file.migrations.is_empty() {
                output.push_str(&format!("--- {:?} ---\n", file.file_path));
                for m in &file.migrations {
                    output.push_str(&format!(
                        "  L{}: [{}] {} -> {}\n",
                        m.line,
                        m.migration_type.as_str(),
                        m.original.chars().take(50).collect::<String>(),
                        m.migrated
                    ));
                }
                output.push('\n');
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_echo() {
        let line = "<?php echo $title; ?>";
        let result = ThemeMigrator::try_migrate_echo(line);
        assert_eq!(result, Some("{{ title }}".to_string()));
    }

    #[test]
    fn test_migrate_short_echo() {
        let line = "<?= $name ?>";
        let result = ThemeMigrator::try_migrate_short_echo(line);
        assert_eq!(result, Some("{{ name }}".to_string()));
    }

    #[test]
    fn test_migrate_if() {
        let line = "<?php if ($show): ?>";
        let result = ThemeMigrator::try_migrate_if(line);
        assert_eq!(result, Some("{% if show %}".to_string()));
    }

    #[test]
    fn test_migrate_endif() {
        let line = "<?php endif; ?>";
        let result = ThemeMigrator::try_migrate_endif(line);
        assert_eq!(result, Some("{% endif %}".to_string()));
    }

    #[test]
    fn test_migrate_foreach() {
        let line = "<?php foreach ($posts as $post): ?>";
        let result = ThemeMigrator::try_migrate_foreach(line);
        assert_eq!(result, Some("{% for post in posts %}".to_string()));
    }

    #[test]
    fn test_migrate_endforeach() {
        let line = "<?php endforeach; ?>";
        let result = ThemeMigrator::try_migrate_endforeach(line);
        assert_eq!(result, Some("{% endfor %}".to_string()));
    }

    #[test]
    fn test_migrate_include() {
        let line = r#"<?php include 'header.php'; ?>"#;
        let result = ThemeMigrator::try_migrate_include(line);
        assert_eq!(result, Some(r#"{% include "header.html" %}"#.to_string()));
    }

    #[test]
    fn test_migrate_function() {
        let line = "<?php the_content($post); ?>";
        let result = ThemeMigrator::try_migrate_function(line);
        assert_eq!(result, Some("{{ the_content(post) }}".to_string()));
    }

    #[test]
    fn test_migrate_comment() {
        let line = "<?php // This is a comment ?>";
        let result = ThemeMigrator::try_migrate_comment(line);
        assert_eq!(result, Some("{# This is a comment #}".to_string()));
    }

    #[test]
    fn test_analyze_content() {
        let content = r#"<?php echo $title; ?>
<?php if ($show): ?>
<p>Hello</p>
<?php endif; ?>"#;
        let migrations = ThemeMigrator::analyze_content(content);
        assert_eq!(migrations.len(), 3);
        assert_eq!(migrations[0].migration_type, MigrationType::Echo);
        assert_eq!(migrations[1].migration_type, MigrationType::Conditional);
        assert_eq!(migrations[2].migration_type, MigrationType::Conditional);
    }

    #[test]
    fn test_generate_summary() {
        let report = MigrationReport {
            source_path: PathBuf::from("/themes/test"),
            files: vec![FileMigrationReport {
                file_path: PathBuf::from("/themes/test/index.php"),
                migrations: vec![Migration {
                    line: 1,
                    original: "<?php echo $title; ?>".to_string(),
                    migrated: "{{ title }}".to_string(),
                    migration_type: MigrationType::Echo,
                }],
                has_errors: false,
                error_message: None,
            }],
            total_files: 1,
            migrated_files: 1,
            error_files: 0,
        };
        let summary = ThemeMigrator::generate_summary(&report);
        assert!(summary.contains("Theme Migration Report"));
        assert!(summary.contains("Total files: 1"));
        assert!(summary.contains("{{ title }}"));
    }
}
