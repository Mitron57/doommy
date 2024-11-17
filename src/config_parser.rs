use pest::Parser;
use pest_derive::Parser;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Value {
    Number(i32),
    List(Vec<Value>),
    Dictionary(BTreeMap<String, Value>),
}

#[derive(Parser)]
#[grammar = "grammar.pest"] // путь к файлу grammar.pest
pub struct ConfigParser {
    constants: HashMap<String, Value>,
}
type Yaml = BTreeMap<String, Value>;
type Constants = HashMap<String, Value>;
impl ConfigParser {
    pub fn parse(input: &str) -> Result<(Yaml, Constants), Box<dyn Error>> {
        let parsed =
            <ConfigParser as pest::Parser<_>>::parse(Rule::start, input)?;
        let constants = HashMap::new();
        let mut yaml = BTreeMap::new();
        let mut parser = ConfigParser { constants };
        // Обрабатываем разобранные данные
        for statement in parsed {
            let statement = statement.into_inner().next().unwrap();
            match statement.as_rule() {
                Rule::def_statement => parser.parse_def_statement(statement)?,
                Rule::expression => {
                    parser.parse_expression(statement)?;
                }
                Rule::list => {
                    let values = statement
                        .into_inner()
                        .map(|val| parser.parse_value(val).unwrap())
                        .collect::<Vec<Value>>();
                    yaml.insert("List".to_string(), Value::List(values));
                }
                Rule::dictionary => {
                    let pairs = statement
                        .into_inner()
                        .map(|kv| {
                            let mut inner = kv.into_inner();
                            let key = inner.next().unwrap().as_str().to_string();
                            let value = parser.parse_value(inner.next().unwrap()).unwrap();
                            (key, value)
                        })
                        .collect::<BTreeMap<_, _>>();
                    yaml.extend(pairs);
                }
                _ => {}
            }
        }
        Ok((yaml, parser.constants))
    }
    fn parse_value(&self, pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
        match pair.as_rule() {
            Rule::NUMBERS => {
                let number = pair.as_str().parse::<i32>()?;
                Ok(Value::Number(number))
            }
            Rule::list => {
                let values = pair
                    .into_inner()
                    .map(|value| self.parse_value(value).unwrap())
                    .collect();
                Ok(Value::List(values))
            }
            Rule::dictionary => {
                let key_value_pairs = pair
                    .into_inner()
                    .filter_map(|kv| {
                        let mut inner = kv.into_inner();
                        let key = inner.next()?.as_str().to_string();
                        let value = self.parse_value(inner.next()?).unwrap();
                        Some((key, value))
                    })
                    .collect();
                Ok(Value::Dictionary(key_value_pairs))
            }
            Rule::NAME => {
                if let Some(value) = self.constants.get(&pair.as_str().to_string()) {
                    Ok(value.clone())
                } else {
                    panic!("Unknown variable: {}", pair.as_str());
                }
            }
            Rule::expression => self.parse_expression(pair),
            Rule::value => self.parse_value(pair.into_inner().next().unwrap()),
            _ => panic!("Unexpected rule"),
        }
    }

    fn parse_def_statement(
        &mut self,
        pair: pest::iterators::Pair<Rule>,
    ) -> Result<(), Box<dyn Error>> {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let value = self.parse_value(inner.next().unwrap())?;
        self.constants.insert(name, value);
        Ok(())
    }

    fn parse_expression(&self, pair: pest::iterators::Pair<Rule>) -> Result<Value, Box<dyn Error>> {
        match pair.as_rule() {
            Rule::bin => {
                // Обработка бинарных операций
                let mut inner = pair.into_inner();
                let mut left = self.parse_expression(inner.next().unwrap())?;

                while let Some(op_pair) = inner.next() {
                    let op = op_pair.as_rule();
                    let right = self.parse_expression(inner.next().unwrap())?;

                    left = match (op, left, right) {
                        (Rule::add, Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                        (Rule::sub, Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                        _ => panic!("Unsupported operation or non-numeric operands"),
                    };
                }
                Ok(left)
            }
            Rule::func => {
                // Обработка функций (например, pow)
                let mut inner = pair.into_inner();
                let base = self.parse_expression(inner.next().unwrap())?;
                let exponent = self.parse_expression(inner.next().unwrap())?;

                if let (Value::Number(base_val), Value::Number(exp_val)) = (base, exponent) {
                    Ok(Value::Number(base_val.pow(exp_val as u32)))
                } else {
                    panic!("Non-numeric values cannot be used in pow()");
                }
            }
            Rule::factor | Rule::expression | Rule::expr => {
                // Обработка чисел, имен, выражений в скобках или функций
                let inner_pair = pair.into_inner().next().unwrap();
                self.parse_expression(inner_pair)
            }
            Rule::NUMBERS => {
                let number = pair.as_str().parse::<i32>()?;
                Ok(Value::Number(number))
            }
            Rule::NAME => {
                let name = pair.as_str().to_string();
                Ok(self
                    .constants
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Unknown variable: {}", name)))
            }
            _ => Err(format!("Unexpected rule: {:?}", pair.as_rule()).into()),
        }
    }
}
