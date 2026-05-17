/*
 * metodos a serem implementados:
 * len
 * ref
 * unique
 * contains
 * if
 * fmt
 * path
 * env
 * uuid
 * json
 * default
 * platform
 * join
 *
 * comentarios:
 * ' comentarios devem ser feitos com `'`
 *
 *
 */

use uuid::Uuid;

use crate::{core::{self, *}, statics};

#[derive(Clone, Default)]
pub struct Parser {
    pub data: Vec<char>,
    pub position: (usize, usize),
    pub namespace: String,
    pub unused: String,
    pub cache: EvaCache
}

impl Parser {
    pub fn new(data: &str) -> Self {
        Self {
            namespace: "root".to_string(),
            data: data.chars().collect(),
            ..Default::default()
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.data.is_empty() {
            return None;
        }

        let data = self.data.clone();
        let character = data.first().unwrap();
        self.data.remove(0);

        Some(*character)
    }

    fn next_word(&mut self) -> Option<String> {
        let mut word = String::new();
        let mut finished_word = false;
        let mut already_pushed = false;
        while let Some(c) = self.next() {
            if ( c.is_whitespace() || c == ':' || c == '(' || c == ')' ) && already_pushed {
                self.data.insert(0, c);
                finished_word = true;
                break;
            }

            if c.is_whitespace() {
                continue;
            }

            word.push(c);
            already_pushed = true;
        }

        if! finished_word {
            return None;
        }

        return Some(word);
    }

    fn next_value(&mut self) -> Option<EvaValue> {
        let Some(first) = self.next_word() else {
            return None;
        };

        match first.as_str() {
            "true" => return Some(EvaValue::Bool(true)),
            "false" => return Some(EvaValue::Bool(false)),
            "nil" => return Some(EvaValue::Nil),
            _ => {}
        }

        if first.as_bytes()[0] == b'"' && first.as_bytes()[first.len()-1] == b'"' {
            return Some(EvaValue::String(first[1..first.len()-1].to_string()));
        }

        let Some(value) = self.complex(&first) else {
            return None;
        };

        Some(value)
    }

    fn is_function(&self, first: &String) -> bool {
        statics::FUNCTIONS
            .iter()
            .any(|(name, _)| *name == first.as_str())
    }

    fn complex(&mut self, first: &String) -> Option<EvaValue> {
        if first.as_bytes()[0] == b'"' {
            let mut str = first[1..first.len()].to_string();
            while let Some(c) = self.next() {
                if c == '"' { break; }
                str.push(c);
            }
            return Some(EvaValue::String(str));
        }

        if self.is_function(first) {
            let Some(p1) = self.next() else {
                return None;
            };

            let mut values: Vec<_> = Vec::new();

            loop {
                let Some(value) = self.next_value() else {
                    break;
                };

                values.push(value);

                let Some(op) = self.next() else {
                    return None;
                };

                if op == ',' {
                    continue;
                }

                self.data.insert(0, op);
                break;
            }

            let Some(p2) = self.next() else {
                return None;
            };

            return match (p1, p2) {
                ('(', ')') => {
                    crate::statics::FUNCTIONS
                        .iter()
                        .find(|(name, _)| *name == first.as_str())
                        .map(|(_, func)| func(values))
                }
                (_, _) => None,
            }
        }

        None
    }

    pub fn parse(&mut self) -> Option<core::Error> {

        while let Some(c) = self.next() {
            match c {
                '\t' => self.position.1 += 4,
                ' ' | '\r' => continue,
                '\n' => {
                    self.position.0 += 1;
                    self.position.1 = 0;
                }

                '@' => {
                    let word = self.next_word();
                    self.namespace = word.unwrap_or_default();
                }

                '\'' => {
                    while let Some(c) = self.next()
                    { if c == '\n' { break; } }
                }

                ':' => {
                    if self.unused.is_empty() {
                        todo!("tratar erro de `:` avulso");
                    }

                    let lhs = self.unused.clone();
                    self.unused.clear();

                    let rhs = self.next_value();
                    self.cache.push((
                        Uuid::new_v4(),
                        EvaCached::Field(
                            self.namespace.clone(),
                            lhs,
                            rhs.unwrap_or_default()
                        )
                    ));
                }

                _ => {
                    self.data.insert(0, c);
                    let word = self.next_word();
                    self.unused = word.unwrap_or_default();
                }
            }
        }

        None
    }
}
