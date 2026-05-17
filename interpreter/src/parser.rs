/*
 * metodos a serem implementados:
 * len
 * ref
 * unique
 * contains
 * format
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
            if ( c.is_whitespace() || c == ':' || c == '(' || c == ')' || c == ',' ) && already_pushed {
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

    fn comment(&mut self) {
        let Some(mut c) = self.next() else {
            return;
        };

        while c.is_whitespace() {
            let Some(c2) = self.next() else {
                return;
            };
            c = c2;
        }

        if c != '\'' {
            self.data.insert(0, c);
            return;
        }

        while let Some(c) = self.next() {
            if c == '\n' {
                let Some(mut c2) = self.next() else {
                    break;
                };

                while c2.is_whitespace() {
                    c2 = self.next().unwrap();
                }

                if c2 == '\'' {
                    continue;
                }

                self.data.insert(0, c2);
                break;
            }
        }
    }

    fn next_value(&mut self) -> Option<EvaValue> {
        let Some(mut c) = self.next() else {
            return None;
        };

        while c.is_whitespace() {
            c = self.next()?;
        }

        match c {
            '{' => {
                let mut map: Vec<_> = Vec::new();

                loop {
                    self.comment();
                    let Some(identifier) = self.next_word() else {
                        break;
                    };

                    let Some(colon) = self.next() else {
                        return None;
                    };

                    if colon != ':' {
                        return None;
                    }

                    let Some(value) = self.next_value() else {
                        break;
                    };

                    self.comment();

                    map.push((identifier, value));
                    let Some(op) = self.next() else {
                        return None;
                    };

                    if op != '}' {
                        self.data.insert(0, op);
                        continue;
                    }

                    break;
                }

                return Some(EvaValue::Map(map));
            }

            '[' => {
                let mut list: Vec<_> = Vec::new();

                loop {
                    self.comment();

                    let Some(value) = self.next_value() else {
                        break;
                    };

                    list.push(value);

                    self.comment();

                    let Some(op) = self.next() else {
                        return None;
                    };

                    if op == ',' {
                        continue;
                    }

                    self.data.insert(0, op);
                    break;
                }

                let Some(mut op) = self.next() else {
                    return None;
                };

                while op.is_whitespace() {
                    op = self.next()?;
                }

                if op != ']' {
                    return None;
                }

                return Some(EvaValue::List(list));
            }

            _ => {
                self.data.insert(0, c);
            }
        };

        let Some(first) = self.next_word() else {
            return None;
        };

        match first.as_str() {
            "true" => return Some(EvaValue::Bool(true)),
            "false" => return Some(EvaValue::Bool(false)),
            "nil" => return Some(EvaValue::Nil),
            _ => {}
        }

        if let Ok(num) = first.parse::<f64>() {
            return Some(EvaValue::Number(num));
        }

        let with_dot_zero = format!("{}.0", first);
        if let Ok(num) = with_dot_zero.parse::<f64>() {
            return Some(EvaValue::Number(num));
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

    fn is_ref(&self, first: &String) -> bool {
        for data in self.cache.iter() {
            match data {
                core::EvaCached::Field(ns, field_name, _) => {
                    if ns == &self.namespace && field_name == first {
                        return true;
                    }
                }
                _ => {}
            }
        }

        return false;
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

        if self.is_ref(first) {
            return Some(EvaValue::OtherField(first.clone()))
        }

        if self.is_function(first) {
            let Some(p1) = self.next() else {
                return None;
            };

            let mut values: Vec<_> = Vec::new();

            loop {
                self.comment();
                let Some(value) = self.next_value() else {
                    break;
                };

                values.push(value);

                let Some(op) = self.next() else {
                    return None;
                };

                if op == ',' {
                    self.comment();
                    continue;
                }

                self.data.insert(0, op);
                break;
            }

            let Some(mut p2) = self.next() else {
                return None;
            };

            while p2.is_whitespace() {
                p2 = self.next()?;
            }

            self.comment();

            return match (p1, p2) {
                ('(', ')') => {
                    crate::statics::FUNCTIONS
                        .iter()
                        .find(|(name, _)| *name == first.as_str())
                        .map(|(_, func)| func(self, values))
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
                    self.cache.push(
                        EvaCached::Field(
                            self.namespace.clone(),
                            lhs,
                            rhs.unwrap_or_default()
                        )
                    );
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
