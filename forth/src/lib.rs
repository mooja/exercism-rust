use std::collections::{HashMap, VecDeque};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Default, Debug)]
pub struct Forth {
    state: Fsm,
    stack: Vec<Value>,
    expansions: Vec<Vec<Op>>,
    aliases: HashMap<String, usize>,
    new_alias: Option<(String, Vec<Op>)>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    InvalidWord,
    UnknownWord,
}

#[derive(Clone, Debug)]
pub enum Op {
    Value(Value),

    Add,
    Sub,
    Mul,
    Div,

    Dup,
    Drop,
    Swap,
    Over,

    ExpandAlias(usize),

    StartNewAlias,
    NewAliasName(String),
    EndNewAlias,
}

#[derive(Debug)]
pub enum Fsm {
    ExpectingOp,
    ExpectingAliasName,
    ExpectingAliasDefinition,
}

impl Default for Fsm {
    fn default() -> Self {
        Fsm::ExpectingOp
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn parse_token(&mut self, s: &str) -> std::result::Result<Op, Error> {
        match self.state {
            Fsm::ExpectingOp => match s {
                s if self.aliases.contains_key(s) => {
                    let key_idx = *self.aliases.get(s).unwrap();
                    Ok(Op::ExpandAlias(key_idx))
                }

                ":" => Ok(Op::StartNewAlias),

                s if s.parse::<i32>().is_ok() => Ok(Op::Value(s.parse::<i32>().unwrap())),

                "+" => Ok(Op::Add),
                "-" => Ok(Op::Sub),
                "*" => Ok(Op::Mul),
                "/" => Ok(Op::Div),

                "dup" => Ok(Op::Dup),
                "drop" => Ok(Op::Drop),
                "swap" => Ok(Op::Swap),
                "over" => Ok(Op::Over),

                _ => Err(Error::UnknownWord),
            },

            Fsm::ExpectingAliasName => {
                let name = String::from(s);
                Ok(Op::NewAliasName(name))
            }

            Fsm::ExpectingAliasDefinition => match s {
                ";" => Ok(Op::EndNewAlias),
                
                s if self.aliases.contains_key(s) => {
                    let key_idx = *self.aliases.get(s).unwrap();
                    Ok(Op::ExpandAlias(key_idx))
                }

                s if s.parse::<i32>().is_ok() => Ok(Op::Value(s.parse::<i32>().unwrap())),

                "+" => Ok(Op::Add),
                "-" => Ok(Op::Sub),
                "*" => Ok(Op::Mul),
                "/" => Ok(Op::Div),

                "dup" => Ok(Op::Dup),
                "drop" => Ok(Op::Drop),
                "swap" => Ok(Op::Swap),
                "over" => Ok(Op::Over),

                _ => Err(Error::UnknownWord),
            },
        }
    }

    fn math_op(&mut self, op: Op) -> Result {
        match op {
            Op::Add => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(a + b);
            }

            Op::Sub => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(b - a);
            }

            Op::Mul => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(a * b);
            }

            Op::Div => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;

                if a == 0 {
                    return Err(Error::DivisionByZero);
                }

                self.stack.push(b / a);
            }

            _ => panic!(),
        }

        Ok(())
    }

    fn stack_op(&mut self, op: Op) -> Result {
        match op {
            Op::Dup => {
                let a = *self.stack.last().ok_or(Error::StackUnderflow)?;
                self.stack.push(a);
            }

            Op::Drop => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
            }

            Op::Swap => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(a);
                self.stack.push(b);
            }

            Op::Over => {
                let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let c = b;
                self.stack.push(b);
                self.stack.push(a);
                self.stack.push(c);
            }

            _ => panic!(),
        }

        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut input_iter = input
            .split_ascii_whitespace()
            .map(|s| s.to_ascii_lowercase());
        let mut ops_to_process: VecDeque<_> = VecDeque::new();

        loop {
            let op = if !ops_to_process.is_empty() {
                ops_to_process.pop_front()
            } else {
                let op = input_iter.next();
                match op {
                    None => None,
                    Some(s) => Some(self.parse_token(s.as_str())?),
                }
            };

            if op.is_none() {
                break;
            }

            let op = op.unwrap();

            match self.state {
                Fsm::ExpectingOp => match op {
                    Op::ExpandAlias(idx) => {
                        ops_to_process = self.expansions[idx]
                            .iter()
                            .cloned()
                            .chain(ops_to_process.into_iter())
                            .collect();
                    }

                    Op::StartNewAlias => self.state = Fsm::ExpectingAliasName,

                    Op::Dup | Op::Over | Op::Swap | Op::Drop => self.stack_op(op)?,

                    Op::Add | Op::Sub | Op::Mul | Op::Div => self.math_op(op)?,

                    Op::Value(v) => self.stack.push(v),

                    _ => return Err(Error::InvalidWord),
                },

                Fsm::ExpectingAliasName => match op {
                    Op::NewAliasName(name) => {
                        if name.chars().next().unwrap().is_numeric() {
                            return Err(Error::InvalidWord);
                        }

                        self.new_alias = Some((name, vec![]));
                        self.state = Fsm::ExpectingAliasDefinition;
                    }

                    _ => panic!(),
                },

                Fsm::ExpectingAliasDefinition => match op {
                    Op::EndNewAlias => {
                        if let Some((name, expansion)) = self.new_alias.take() {
                            self.expansions.push(expansion);
                            self.aliases.insert(name, self.expansions.len() - 1);
                            self.state = Fsm::ExpectingOp;
                        } else {
                            panic!()
                        }
                    }

                    _ => {
                        if let Some((ref _name, ref mut expansion)) = self.new_alias {
                            expansion.push(op);
                        }
                    }
                }
            }
        }

        match self.state {
            Fsm::ExpectingOp => Ok(()),
            _ => {
                self.state = Fsm::ExpectingOp;
                self.new_alias = None;
                Err(Error::InvalidWord)
            }
        }
    }
}
