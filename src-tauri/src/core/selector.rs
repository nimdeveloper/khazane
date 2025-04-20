#![allow(dead_code)]
use super::{
    error::{custom_error, Error, Result},
    helpers::random_string,
};
use duckdb::{params_from_iter, Connection, Statement, ToSql};
use log;
use std::{collections::HashMap, ops::Deref, rc::Rc};

const REL_DETERMINER: &str = "__";

fn resolve_field_string(f: String, rel_map: &HashMap<String, String>, current: String) -> String {
    let mut res = "".to_string();
    let parts: Vec<&str> = f.split(REL_DETERMINER).collect();
    if parts.len() > 1 {
        let column = parts.last().unwrap();
        let relation = parts[..parts.len() - 1].join(REL_DETERMINER);
        // let scoped_relation;
        if rel_map.contains_key(&relation) {
            res += &format!("{}.{}", rel_map.get(&relation).unwrap(), column);
        } else {
            // TODO: Unknown relation!
            if current.len() > 0 {
                let scoped_relation = current.to_string();
                res += &format!("{}.{}", scoped_relation.to_owned(), column);
            } else {
                res += &f;
            }
        }
    } else {
        if f.starts_with("(") {
            //
            //
            // !WARNING: Unstable condition! waiting till the Duckdb implements Arrays
            //
            //
            res += &f;
        } else {
            if current.len() > 0 {
                let scoped_relation = current.to_string();
                res += &format!("{}.{}", scoped_relation.to_owned(), f);
            } else {
                res += &f;
            }
        }
    }
    res
}

pub struct Field {
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
impl Field {
    fn resolve(&self, rel_map: &HashMap<String, String>, current: String) -> String {
        resolve_field_string(*self.inner.to_owned(), rel_map, current)
    }
}

pub struct Value {
    inner: Rc<dyn ToSql>,
}
impl<T: ToSql + Clone + 'static> From<T> for Value {
    fn from(item: T) -> Self {
        return Self {
            inner: Rc::new(item.to_owned()),
        };
    }
}

pub enum Internal {
    Field(Field),
    Value(Value),
}

pub enum OrderDirection {
    DESC,
    ASC,
}
impl From<String> for OrderDirection {
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
    joins: HashMap<String, Join>,
    conditions: Vec<Condition>,
    rel_map: HashMap<String, String>,
    order: Option<Field>,
    order_direction: OrderDirection,
    limit: Option<usize>,
    offset: Option<usize>,
}
impl Default for Selector {
    fn default() -> Self {
        return Self {
            select: None,
            joins: HashMap::new(),
            conditions: Vec::new(),
            rel_map: HashMap::new(),
            order: None,
            order_direction: OrderDirection::ASC,
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
    pub fn with(&mut self, join: Join, join_as: &str) {
        let mut join_prefix;
        loop {
            join_prefix = random_string(4);
            if !self.rel_map.contains_key(&join_prefix) {
                self.rel_map
                    .insert(join_prefix.to_owned(), join_as.to_string());
                break;
            }
        }
        self.joins.insert(join_prefix, join);
    }
    pub fn order(&mut self, column: Field, direction: OrderDirection) -> &mut Self {
        self.order = Some(column);
        self.order_direction = direction;
        self
    }
    pub fn paginate(&mut self, page_size: usize, page_number: Option<usize>) -> &mut Self {
        self.limit = Some(page_size);
        self.offset = page_number;
        self
    }
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn filter(
        &mut self,
        comparable: Internal,
        operation: Operations,
        compared: Internal,
    ) -> &mut Condition {
        let new_condition = Condition {
            comparable: Box::new(comparable),
            operation,
            compared: Box::new(compared),
            and: None,
            or: None,
        };
        self.conditions.push(new_condition);
        self.conditions.last_mut().unwrap()
    }
    pub fn all<'a>(
        &mut self,
        connection: &'a Connection,
    ) -> Result<(Rc<Statement<'a>>, DbTranslateBox<'a>)> {
        let (query, params, scope, rel_map) = self.get_sql();
        log::debug!("{}", query.to_owned());
        let mut stmt: Statement<'a> = connection.prepare(&query).map_err(Error::from)?;
        stmt.execute(params_from_iter(params.into_iter()))
            .map_err(Error::from)?;
        let res = Rc::new(stmt);
        return Ok((
            res.clone(),
            DbTranslateBox::new(res.clone(), scope, rel_map),
        ));
    }
}
impl Selector {
    fn get_rel_map(&self, scope_rel: String) -> HashMap<String, String> {
        let mut res = HashMap::new();
        match &self.select {
            Some(_) => {
                for (rel, shorthand) in self.rel_map.iter() {
                    let scope_rel = scope_rel.to_owned() + REL_DETERMINER + &rel;
                    let scope_shorthand = shorthand.to_owned();
                    res.insert(scope_shorthand.to_owned(), scope_rel.to_owned());
                    if let Some(v) = self.joins.get(rel) {
                        let value = v
                            .get_rel_map(scope_rel.to_owned(), scope_shorthand.to_owned())
                            .to_owned();
                        for (k, v) in value.iter() {
                            res.insert(v.to_owned(), k.to_owned());
                        }
                    }
                }
            }
            None => {}
        };
        res
    }
    fn get_sql(&self) -> (String, Vec<Rc<dyn ToSql>>, String, HashMap<String, String>) {
        if let Some(super_select) = &self.select {
            let scope = random_string(1);
            let mut value_registry: Vec<Rc<dyn ToSql>> = Vec::new();
            let mut rel_map = self.get_rel_map(scope.to_owned());
            let selections = self.get_selection(&self.rel_map, scope.to_owned());
            let joins = self.get_joins(scope.to_owned(), &rel_map, &mut value_registry);
            rel_map.insert(scope.to_owned(), scope.to_owned());
            let conditions = self.get_conditions(&rel_map, scope.to_owned(), &mut value_registry);
            let mut res = format!(
                "SELECT\n{}\nFROM {} {}\n",
                selections, super_select.table_name, scope
            )
            .to_string();
            if joins.len() > 0 {
                res += format!("{}\n", joins).as_str()
            }
            if conditions.len() > 0 {
                res += format!("WHERE\n\t{}", conditions).as_str();
            }
            if let Some(order) = &self.order {
                res += format!("ORDER BY {}", order.resolve(&rel_map, scope.to_owned())).as_str();
                match self.order_direction {
                    OrderDirection::ASC => res += " ASC",
                    OrderDirection::DESC => res += " DESC",
                }
                res += "\n"
            }
            if let Some(limit) = self.limit {
                res += format!("LIMIT {}\n", limit).as_str();
            }
            if let Some(offset) = self.offset {
                res += format!("OFFSET {}\n", offset).as_str();
            }
            return (res, value_registry, scope, rel_map);
        }
        ("".to_string(), Vec::new(), String::new(), HashMap::new())
    }
    fn get_conditions(
        &self,
        rel_map: &HashMap<String, String>,
        current: String,
        registry: &mut Vec<Rc<dyn ToSql>>,
    ) -> String {
        let mut res = "".to_string();
        res += self
            .conditions
            .iter()
            .map(|e| "(".to_string() + &e.get_sql(rel_map, current.to_owned(), registry) + ")")
            .collect::<Vec<String>>()
            .join("AND")
            .as_str();
        res
    }
    fn get_selection(&self, rel_map: &HashMap<String, String>, current: String) -> String {
        let mut res = "".to_string();
        if self.select.is_some() {
            match &self.select {
                Some(selections) => {
                    res += selections.get_sql(current.to_owned()).as_str();
                }
                None => {}
            };
        }

        if self.joins.len() > 0 {
            res += "\n";
            res += self
                .joins
                .iter()
                .map(|(scope, e)| {
                    e.get_selections(&rel_map, current.to_owned() + REL_DETERMINER + &*scope)
                })
                .collect::<Vec<String>>()
                .join("\n")
                .as_str();
        }
        res
    }
    fn get_joins(
        &self,
        scope: String,
        rel_map: &HashMap<String, String>,
        registry: &mut Vec<Rc<dyn ToSql>>,
    ) -> String {
        self.joins
            .iter()
            .map(|(rel, e)| {
                e.get_joins(
                    scope.to_owned() + REL_DETERMINER + &*rel,
                    scope.to_owned(),
                    rel_map,
                    registry,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

struct Query {
    columns: Vec<String>,
    table_name: String,
}
impl Default for Query {
    fn default() -> Self {
        return Self {
            columns: Vec::new(),
            table_name: "".to_string(),
        };
    }
}
impl Query {}
impl Query {
    pub fn get_sql(&self, scope: String) -> String {
        self.columns
            .iter()
            .map(|e| {
                if scope.len() > 0 {
                    format!("\t{scope}.{e} AS {scope}_{e},",)
                } else {
                    format!("\t{e} AS {e},")
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Clone, Copy)]
pub enum Operations {
    EqualTo,
    Like,
    In,
}
impl Into<&str> for Operations {
    fn into(self) -> &'static str {
        match self {
            Operations::EqualTo => "=",
            Operations::Like => "LIKE",
            Operations::In => "IN",
        }
    }
}

pub struct Condition {
    comparable: Box<Internal>,
    operation: Operations,
    compared: Box<Internal>,
    and: Option<Box<Condition>>,
    or: Option<Box<Condition>>,
}
impl Condition {
    pub fn new(comparable: Internal, operation: Operations, compared: Internal) -> Self {
        Self {
            comparable: Box::new(comparable),
            operation: operation,
            compared: Box::new(compared),
            and: None,
            or: None,
        }
    }
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
    fn get_sql(
        &self,
        rel_map: &HashMap<String, String>,
        current: String,
        registry: &mut Vec<Rc<dyn ToSql>>,
    ) -> String {
        let mut res = "".to_string();
        match self.comparable.deref() {
            Internal::Field(name) => {
                let tmp = name.resolve(rel_map, current.to_owned());
                res += tmp.as_str();
            }
            Internal::Value(v) => {
                res += "?";
                registry.push(v.inner.to_owned());
            }
        };
        res += " ";
        res += self.operation.into();
        res += " ";
        match self.compared.deref() {
            Internal::Field(name) => {
                let tmp = name.resolve(rel_map, current.to_owned());
                res += tmp.as_str();
            }
            Internal::Value(v) => {
                res += "?";
                registry.push(v.inner.to_owned());
            }
        };
        if self.and.is_some() {
            res += " ";
            let and_cond = self.and.as_ref().unwrap();
            res += "AND (";
            res += and_cond
                .get_sql(rel_map, current.to_owned(), registry)
                .as_str();
            res += ")";
        }
        if self.or.is_some() {
            res += " ";
            let or_cond = self.or.as_ref().unwrap();
            res += "AND (";
            res += or_cond
                .get_sql(rel_map, current.to_owned(), registry)
                .as_str();
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

#[derive(Clone, Copy)]
pub enum JoinMethod {
    LeftJoin,
    RightJoin,
    FullJoin,
}
impl Into<&str> for JoinMethod {
    fn into(self) -> &'static str {
        match self {
            JoinMethod::LeftJoin => "LEFT JOIN",
            JoinMethod::RightJoin => "RIGHT JOIN",
            JoinMethod::FullJoin => "JOIN",
        }
    }
}

pub struct Join {
    select: Option<Query>,
    joins: HashMap<String, Join>,
    table_name: String,
    join_on: Condition,
    rel_map: HashMap<String, String>,
    join_method: JoinMethod,
}
impl Join {
    pub fn to<'a>(
        table_name: &str,
        join_method: JoinMethod,
        join_on: Condition,
        selections: Vec<String>,
    ) -> Self {
        let res: Join = Self {
            table_name: table_name.to_string(),
            join_method,
            join_on,
            rel_map: HashMap::new(),
            joins: HashMap::new(),
            select: Some(Query {
                columns: selections,
                table_name: table_name.to_string(),
                ..Query::default()
            }),
        };
        // if caller.is_some() {
        //     let caller = caller.unwrap();
        //     caller(&res);
        // }
        res
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
    pub fn with(&mut self, join: Join, join_as: String) {
        let mut join_prefix;
        if self.rel_map.contains_key(&join_as) {
            // TODO: maybe add warning?
            return;
        }
        loop {
            join_prefix = random_string(4);
            if self
                .rel_map
                .values()
                .find(|&e| *e == join_prefix.to_owned())
                .is_none()
            {
                self.rel_map.insert(join_as, join_prefix.to_owned());
                break;
            }
        }
        self.joins.insert(join_prefix, join);
    }
}
impl Join {
    fn get_rel_map(&self, scope_rel: String, scope_shorthand: String) -> HashMap<String, String> {
        let mut res = HashMap::new();
        for (rel, shorthand) in self.rel_map.iter() {
            let tmp_scope_rel;
            let tmp_scope_shorthand;
            if scope_rel.len() > 0 {
                tmp_scope_rel = scope_rel.to_owned() + REL_DETERMINER + &rel;
            } else {
                tmp_scope_rel = rel.to_owned();
            }
            if scope_shorthand.len() > 0 {
                tmp_scope_shorthand = scope_shorthand.to_owned() + REL_DETERMINER + &shorthand;
            } else {
                tmp_scope_shorthand = rel.to_owned();
            }
            res.insert(tmp_scope_shorthand.to_owned(), tmp_scope_rel.to_owned());
            if let Some(v) = self.joins.get(rel) {
                let value = v
                    .get_rel_map(tmp_scope_rel.to_owned(), tmp_scope_shorthand.to_owned())
                    .to_owned();
                for (k, v) in value.iter() {
                    res.insert(v.to_owned(), k.to_owned());
                }
            }
        }
        res
    }
    fn get_selections(&self, rel_map: &HashMap<String, String>, current: String) -> String {
        let mut res = "".to_string();
        if self.select.is_some() {
            res += self
                .select
                .as_ref()
                .unwrap()
                .get_sql(current.to_owned())
                .as_str();
        }
        if self.joins.len() > 0 {
            res += self
                .joins
                .iter()
                .map(|(rel, e)| {
                    e.get_selections(&rel_map, current.to_owned() + REL_DETERMINER + &*rel)
                })
                .collect::<Vec<String>>()
                .join("\n")
                .as_str();
        }
        return res;
    }
    fn get_joins(
        &self,
        scope: String,
        parent_scope: String,
        rel_map: &HashMap<String, String>,
        registry: &mut Vec<Rc<dyn ToSql>>,
    ) -> String {
        let mut res = format!(
            "{} {} {} ON {}\n",
            <JoinMethod as Into<&str>>::into(self.join_method),
            self.table_name,
            scope,
            self.join_on
                .get_sql(&rel_map, parent_scope.to_owned(), registry)
        )
        .to_string();
        for (rel, e) in self.joins.iter() {
            res += e
                .get_joins(
                    scope.to_owned() + REL_DETERMINER + &*rel,
                    scope.to_owned(),
                    &rel_map,
                    registry,
                )
                .as_str();
        }
        res
    }
}

pub struct DbTranslateBox<'a> {
    stmt: Rc<Statement<'a>>,
    scope: String,
    rel_map: Rc<HashMap<String, String>>,
}

impl<'a> DbTranslateBox<'a> {
    fn new(stmt: Rc<Statement<'a>>, scope: String, rel_map: HashMap<String, String>) -> Self {
        DbTranslateBox {
            stmt,
            scope,
            rel_map: Rc::new(rel_map),
        }
    }
    pub fn field(&self, name: &str) -> Result<usize> {
        self.stmt
            .column_index(format!("{}_{}", self.scope, name).as_str())
            .map_err(Error::from)
    }
    pub fn with_rel(&self, rel: &str) -> Result<Self> {
        let new_scope = self.scope.to_owned() + REL_DETERMINER + rel;
        if self.rel_map.contains_key(&new_scope) {
            return Ok(Self {
                stmt: self.stmt.clone(),
                scope: new_scope.to_owned(),
                rel_map: self.rel_map.clone(),
            });
        } else {
            return Err(custom_error(format!("Unknown relation {}.", rel)));
        }
    }
}
