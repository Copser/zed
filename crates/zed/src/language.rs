pub use language::*;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::{str, sync::Arc};

#[derive(RustEmbed)]
#[folder = "languages"]
struct LanguageDir;

mod rust {
    use anyhow::Result;
    use async_trait::async_trait;
    use collections::{HashMap, HashSet};
    use language::{Diagnostic, DiagnosticEntry, DiagnosticSeverity};
    use parking_lot::Mutex;
    use serde::Deserialize;
    use serde_json::Deserializer;
    use smol::process::Command;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    #[derive(Default)]
    pub struct DiagnosticProvider {
        reported_paths: Mutex<HashSet<Arc<Path>>>,
    }

    #[derive(Debug, Deserialize)]
    struct Check {
        message: CompilerMessage,
    }

    #[derive(Debug, Deserialize)]
    struct CompilerMessage {
        code: ErrorCode,
        spans: Vec<Span>,
        message: String,
        level: ErrorLevel,
    }

    #[derive(Debug, Deserialize)]
    enum ErrorLevel {
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "note")]
        Note,
    }

    #[derive(Debug, Deserialize)]
    struct ErrorCode {
        code: String,
    }

    #[derive(Debug, Deserialize)]
    struct Span {
        is_primary: bool,
        file_name: PathBuf,
        byte_start: usize,
        byte_end: usize,
    }

    #[async_trait]
    impl language::DiagnosticProvider for DiagnosticProvider {
        async fn diagnose(
            &self,
            path: Arc<Path>,
        ) -> Result<HashMap<Arc<Path>, Vec<DiagnosticEntry<usize>>>> {
            let output = Command::new("cargo")
                .arg("check")
                .args(["--message-format", "json"])
                .current_dir(&path)
                .output()
                .await?;

            let mut group_id = 0;
            let mut diagnostics_by_path = HashMap::default();
            let mut new_reported_paths = HashSet::default();
            for value in
                Deserializer::from_slice(&output.stdout).into_iter::<&serde_json::value::RawValue>()
            {
                if let Ok(check) = serde_json::from_str::<Check>(value?.get()) {
                    let severity = match check.message.level {
                        ErrorLevel::Warning => DiagnosticSeverity::WARNING,
                        ErrorLevel::Error => DiagnosticSeverity::ERROR,
                        ErrorLevel::Note => DiagnosticSeverity::INFORMATION,
                    };
                    for span in check.message.spans {
                        let span_path: Arc<Path> = span.file_name.into();
                        new_reported_paths.insert(span_path.clone());
                        diagnostics_by_path
                            .entry(span_path)
                            .or_insert(Vec::new())
                            .push(DiagnosticEntry {
                                range: span.byte_start..span.byte_end,
                                diagnostic: Diagnostic {
                                    code: Some(check.message.code.code.clone()),
                                    severity,
                                    message: check.message.message.clone(),
                                    group_id,
                                    is_valid: true,
                                    is_primary: span.is_primary,
                                    is_disk_based: true,
                                },
                            });
                    }
                    group_id += 1;
                }
            }

            let reported_paths = &mut *self.reported_paths.lock();
            for old_reported_path in reported_paths.iter() {
                if !diagnostics_by_path.contains_key(old_reported_path) {
                    diagnostics_by_path.insert(old_reported_path.clone(), Default::default());
                }
            }
            *reported_paths = new_reported_paths;

            Ok(diagnostics_by_path)
        }
    }
}

pub fn build_language_registry() -> LanguageRegistry {
    let mut languages = LanguageRegistry::default();
    languages.add(Arc::new(rust()));
    languages.add(Arc::new(markdown()));
    languages
}

fn rust() -> Language {
    let grammar = tree_sitter_rust::language();
    let config = toml::from_slice(&LanguageDir::get("rust/config.toml").unwrap().data).unwrap();
    Language::new(config, Some(grammar))
        .with_highlights_query(load_query("rust/highlights.scm").as_ref())
        .unwrap()
        .with_brackets_query(load_query("rust/brackets.scm").as_ref())
        .unwrap()
        .with_indents_query(load_query("rust/indents.scm").as_ref())
        .unwrap()
        .with_diagnostic_provider(rust::DiagnosticProvider::default())
}

fn markdown() -> Language {
    let grammar = tree_sitter_markdown::language();
    let config = toml::from_slice(&LanguageDir::get("markdown/config.toml").unwrap().data).unwrap();
    Language::new(config, Some(grammar))
        .with_highlights_query(load_query("markdown/highlights.scm").as_ref())
        .unwrap()
}

fn load_query(path: &str) -> Cow<'static, str> {
    match LanguageDir::get(path).unwrap().data {
        Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
        Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
    }
}
