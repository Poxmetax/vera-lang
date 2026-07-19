//! VERA MVP lexer (Phase 1).

use crate::ast::Span;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokKind {
    Ident,
    TypeIdent,
    Int,
    Str,
    Kw,
    Op,
    Punct,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokKind,
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Error)]
#[error("{span:?}: {message}")]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

const KEYWORDS: &[&str] = &[
    "fn", "let", "if", "else", "match", "uses", "requires", "ensures", "true", "false", "unit",
    "Int", "Bool", "Str", "Unit", "Console", "List", "Option", "Result", "struct", "enum",
];

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = source.chars().collect();
    let n = chars.len();
    let mut i = 0usize;
    let mut line = 1u32;
    let mut col = 1u32;
    let mut out = Vec::new();

    let peek = |i: usize, k: usize| -> Option<char> {
        let j = i + k;
        if j < n {
            Some(chars[j])
        } else {
            None
        }
    };

    while i < n {
        let ch = chars[i];
        // UTF-8 BOM (common on Windows editors / PowerShell Out-File)
        if ch == '\u{FEFF}' {
            i += 1;
            continue;
        }
        if ch == ' ' || ch == '\t' || ch == '\r' {
            i += 1;
            col += 1;
            continue;
        }
        if ch == '\n' {
            i += 1;
            line += 1;
            col = 1;
            continue;
        }
        if ch == '/' && peek(i, 1) == Some('/') {
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        let start = Span { line, col };

        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut text = String::new();
            while i < n && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                text.push(chars[i]);
                i += 1;
                col += 1;
            }
            let kind = if KEYWORDS.contains(&text.as_str()) {
                TokKind::Kw
            } else if text.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                TokKind::TypeIdent
            } else {
                TokKind::Ident
            };
            out.push(Token {
                kind,
                text,
                span: start,
            });
            continue;
        }

        if ch.is_ascii_digit() {
            let mut text = String::new();
            while i < n && (chars[i].is_ascii_digit() || chars[i] == '_') {
                if chars[i] != '_' {
                    text.push(chars[i]);
                }
                i += 1;
                col += 1;
            }
            out.push(Token {
                kind: TokKind::Int,
                text,
                span: start,
            });
            continue;
        }

        if ch == '"' {
            i += 1;
            col += 1;
            let mut text = String::new();
            while i < n && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1;
                    col += 1;
                    if i >= n {
                        return Err(LexError {
                            message: "unterminated string".into(),
                            span: start,
                        });
                    }
                    let esc = chars[i];
                    let mapped = match esc {
                        'n' => '\n',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        _ => {
                            return Err(LexError {
                                message: format!("unknown escape \\{esc}"),
                                span: start,
                            });
                        }
                    };
                    text.push(mapped);
                    i += 1;
                    col += 1;
                } else {
                    text.push(chars[i]);
                    i += 1;
                    col += 1;
                }
            }
            if i >= n || chars[i] != '"' {
                return Err(LexError {
                    message: "unterminated string".into(),
                    span: start,
                });
            }
            i += 1;
            col += 1;
            out.push(Token {
                kind: TokKind::Str,
                text,
                span: start,
            });
            continue;
        }

        let two: String = format!("{}{}", ch, peek(i, 1).unwrap_or('\0'));
        if matches!(
            two.as_str(),
            "==" | "!=" | "<=" | ">=" | "||" | "&&" | "->" | "=>" | "++" | "::"
        ) {
            i += 2;
            col += 2;
            out.push(Token {
                kind: TokKind::Op,
                text: two[..2].to_string(),
                span: start,
            });
            continue;
        }

        if "+-*/%<>=!?".contains(ch) {
            i += 1;
            col += 1;
            // `?ident` with no space → hole token (SPEC §3.2).
            if ch == '?' {
                if let Some(next) = peek(i, 0) {
                    if next.is_ascii_alphabetic() || next == '_' {
                        let mut name = String::new();
                        while i < chars.len()
                            && (chars[i].is_ascii_alphanumeric() || chars[i] == '_')
                        {
                            name.push(chars[i]);
                            i += 1;
                            col += 1;
                        }
                        out.push(Token {
                            kind: TokKind::Op,
                            text: format!("?{name}"),
                            span: start,
                        });
                        continue;
                    }
                }
            }
            out.push(Token {
                kind: TokKind::Op,
                text: ch.to_string(),
                span: start,
            });
            continue;
        }

        if "(){},;:|.[]".contains(ch) {
            i += 1;
            col += 1;
            out.push(Token {
                kind: TokKind::Punct,
                text: ch.to_string(),
                span: start,
            });
            continue;
        }

        return Err(LexError {
            message: format!("unexpected character {ch:?}"),
            span: start,
        });
    }

    out.push(Token {
        kind: TokKind::Eof,
        text: String::new(),
        span: Span { line, col },
    });
    Ok(out)
}
