use std::{collections::HashMap, ops::Deref};

use duckdb::{Connection, ToSql};

use super::{
    error::{Error, Result},
    helpers::random_string,
};

struct Field {
    inner: Box<String>,
}
impl From<String> for Field {
    fn from(item: String) -> Self {
        Self {
            inner: Box::new(item.to_owned()),
        }
    }
}
impl From<&str> for Field {
    fn from(item: &str) -> Self {
        Self {
            inner: Box::new(item.to_string()),
        }
    }
}

struct Value {
    inner: Box<dyn ToSql>,
}
impl<T: ToSql + Clone + 'static> From<T> for Value {
    fn from(item: T) -> Self {
        return Self {
            inner: Box::new(item.to_owned()),
        };
    }
}

pub enum Internal {
    Field(Field),
    Value(Value),
}

pub enum OderDirection {
    DESC,
    ASC,
}
impl From<String> for OderDirection {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "asc" => Self::ASC,
            "desc" => Self::DESC,
            _ => Self::ASC,
        }
    }
}

pub struct Selector {
    select: Option<Query>,
    joins: Vec<Join>,
    conditions: Option<Box<Condition>>,
    rel_map: HashMap<String, String>,
    order: Option<Field>,
    order_direction: OderDirection,
    limit: Option<usize>,
    offset: Option<usize>,
}
impl Default for Selector {
    fn default() -> Self {
        return Self {
            select: None,
            joins: Vec::new(),
            conditions: None,
            rel_map: HashMap::new(),
            order: None,
            order_direction: OderDirection::ASC,
            limit: None,
            offset: None,
        };
    }
}
impl Selector {
    pub fn select(&mut self, cols: Vec<String>, table_name: &str) -> &mut Self {
        let query = Query {
            columns: cols,
            table_name: table_name.to_string(),
            ..Query::default()
        };
        self.select = Some(query);
        self
    }
    pub fn join(&mut self, mut with: Join, join_as: String) {
        let mut join_prefix;
        loop {
            join_prefix = random_string(4);
            if !self.rel_map.contains_key(&join_prefix) {
                self.rel_map.insert(join_prefix.to_owned(), join_as);
                break;
            }
        }
        with.set_shorthand(join_prefix.to_owned());
        self.joins.push(with);
    }
    pub fn order(&mut self, column: Field, direction: OderDirection) -> &mut Self {
        self.order = Some(column);
        self.order_direction = direction;
        self
    }
    pub fn paginate(&mut self, page_size: usize, page_number: Option<usize>) -> &mut Self {
        self.limit = Some(page_size);
        self.offset = page_number;
        self
    }
    pub fn filter(
        &mut self,
        comparable: Internal,
        operation: Operations,
        compared: Internal,
    ) -> &mut Condition {
        let new_condition = Box::new(Condition {
            comparable: Box::new(comparable),
            operation,
            compared: Box::new(compared),
            and: None,
            or: None,
        });
        self.conditions = Some(new_condition);
        self.conditions.as_deref_mut().unwrap()
    }
    pub fn all<'a>(&mut self, connection: &'a Connection) -> Result<()> {
        let query = self.get_sql();
        let mut stmt = connection.prepare(&query).map_err(Error::from)?;
        // params_from_iter
        let mut rows = stmt.query(
            params
                .into_iter()
                .map(|p| p.as_ref())
                .collect::<Vec<_>>()
                .as_slice(),
        )?;
        Ok(())
    }
}
impl Selector {
    fn get_sql(&self) -> String {
        let selections = self.get_selection();
        "".to_string()
    }
    fn get_conditions(&self) -> Option<String> {
        match self.conditions {
            Some(cond) => Some(cond.get_sql("".to_string(), |e| {
                if self.select.is_some() {
                    let selection = self.select.unwrap();
                    if selection.columns.contains(x)
                }
            })),
            None => None,
        }
    }
    fn get_selection(&self) -> String {
        let mut res = "".to_string();
        if self.select.is_some() {
            match &self.select {
                Some(selections) => {
                    res += selections.get_sql().as_str();
                },
                None => {},
            };
        }
        
        if self.joins.len() > 0 {
            res += self
                    .joins
                    .iter()
                    .map(|e| e.get_selections())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_str();
        }
        res
    }
}

struct Query {
    columns: Vec<String>,
    table_name: String,
    shorthand: String,
}
impl Default for Query {
    fn default() -> Self {
        return Self {
            columns: Vec::new(),
            table_name: "".to_string(),
            shorthand: "".to_string(),
        };
    }
}
impl Query {}
impl Query {
    pub fn get_sql(&self) -> String {
        self.columns
            .iter()
            .map(|e| {
                format!(
                    "\t{shorthand}.{e} AS {shorthand}_{e}",
                    shorthand = self.shorthand,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Clone, Copy)]
pub enum Operations {
    EqualTo,
    Like,
}
impl Into<&str> for Operations {
    fn into(self) -> &'static str {
        match self {
            Operations::EqualTo => "=",
            Operations::Like => "LIKE",
        }
    }
}
struct Condition {
    comparable: Box<Internal>,
    operation: Operations,
    compared: Box<Internal>,
    and: Option<Box<Condition>>,
    or: Option<Box<Condition>>,
}
impl Condition {
    // Add an AND condition and return mutable reference to self for chaining
    pub fn filter(
        &mut self,
        comparable: Internal,
        operation: Operations,
        compared: Internal,
    ) -> &mut Self {
        self.comparable = Box::new(comparable);
        self.operation = operation;
        self.compared = Box::new(compared);
        self
    }

    pub fn and<F>(&mut self, builder: F) -> &mut Self
    where
        F: FnOnce(&mut Condition),
    {
        let mut condition = Condition::default();
        builder(&mut condition);
        self.and = Some(Box::new(condition));
        self
    }

    pub fn or<F>(&mut self, builder: F) -> &mut Self
    where
        F: FnOnce(&mut Condition),
    {
        let mut condition = Condition::default();
        builder(&mut condition);
        self.or = Some(Box::new(condition));
        self
    }
}
impl Condition {
    fn get_sql<T>(&self, scope: String, field_resolver: T) -> String
    where
        T: Fn(String, &Field) -> String,
    {
        let mut res = "".to_string();
        match self.comparable.deref() {
            Internal::Field(name) => {
                let tmp = field_resolver(scope.to_owned(), name);
                res += tmp.as_str();
            }
            Internal::Value(_) => {
                res += "?";
            }
        };
        res += " ";
        res += self.operation.into();
        res += " ";
        match self.compared.deref() {
            Internal::Field(name) => {
                let tmp = field_resolver(scope.to_owned(), name);
                res += tmp.as_str();
            }
            Internal::Value(_) => {
                res += "?";
            }
        };
        if self.and.is_some() {
            res += " ";
            let and_cond = self.and.as_ref().unwrap();
            res += "AND (";
            res += and_cond.get_sql(scope.to_owned(), &field_resolver).as_str();
            res += ")";
        }
        if self.or.is_some() {
            res += " ";
            let or_cond = self.or.as_ref().unwrap();
            res += "AND (";
            res += or_cond.get_sql(scope.to_owned(), &field_resolver).as_str();
            res += ")";
        }
        res
    }
}
impl Default for Condition {
    fn default() -> Self {
        Condition {
            comparable: Box::new(Internal::Value(String::from("1").into())),
            operation: Operations::EqualTo,
            compared: Box::new(Internal::Value(String::from("1").into())),
            and: None,
            or: None,
        }
    }
}


enum JoinMethod {
    LeftJoin,
    RightJoin,
    FullJoin,
}
struct Join {
    select: Option<Query>,
    table_name: String,
    shorthand: String,
    join_on: Condition,
    join_method: JoinMethod,
}
impl Join {
    pub fn to(table_name: &str, join_method: JoinMethod, join_on: Condition) -> Self {
        return Self {
            table_name: table_name.to_string(),
            shorthand: "".to_string(),
            join_method,
            join_on,
            select: None
        };
    }
    pub fn select(&mut self, cols: Vec<String>) -> &mut Self {
        let query = Query {
            columns: cols,
            table_name: self.table_name.to_owned(),
            ..Query::default()
        };
        self.select = Some(query);
        self
    }
}
impl Join {
    fn set_shorthand(&mut self, shorthand: String) {
        self.shorthand = shorthand;
    }
    fn get_selections(&self) -> String {
        if self.select.is_some() {
            return self.select.as_ref().unwrap().get_sql();
        }
        return "".to_string();
    }
}
