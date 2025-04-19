use std::fmt::{self, Display};
use std::marker::PhantomData;

// Enum for comparison operators
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    NotLike,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Equal => write!(f, "="),
            Operator::NotEqual => write!(f, "!="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqual => write!(f, "<="),
            Operator::Like => write!(f, "LIKE"),
            Operator::NotLike => write!(f, "NOT LIKE"),
            Operator::In => write!(f, "IN"),
            Operator::NotIn => write!(f, "NOT IN"),
            Operator::IsNull => write!(f, "IS NULL"),
            Operator::IsNotNull => write!(f, "IS NOT NULL"),
        }
    }
}

impl Operator {
    pub fn from_str(op: &str) -> Option<Self> {
        match op {
            "=" => Some(Operator::Equal),
            "!=" => Some(Operator::NotEqual),
            ">" => Some(Operator::GreaterThan),
            ">=" => Some(Operator::GreaterThanOrEqual),
            "<" => Some(Operator::LessThan),
            "<=" => Some(Operator::LessThanOrEqual),
            "LIKE" => Some(Operator::Like),
            "NOT LIKE" => Some(Operator::NotLike),
            "IN" => Some(Operator::In),
            "NOT IN" => Some(Operator::NotIn),
            "IS NULL" => Some(Operator::IsNull),
            "IS NOT NULL" => Some(Operator::IsNotNull),
            _ => None,
        }
    }
}

// Define a trait for Queryable column fields
pub trait Queryable {
    fn column_name() -> &'static str;
    fn table_name() -> &'static str;
    fn full_name() -> String {
        format!("{}.{}", Self::table_name(), Self::column_name())
    }
}

// Define condition for WHERE clauses
#[derive(Debug, Clone)]
pub struct Condition {
    pub column: String,
    pub operator: Operator,
    pub value: Option<Box<dyn std::any::Any>>,
}

impl Condition {
    pub fn new<T: Queryable, V: 'static>(column: T, operator: Operator, value: V) -> Self {
        Condition {
            column: T::full_name(),
            operator,
            value: Some(Box::new(value)),
        }
    }

    pub fn null<T: Queryable>(column: T, is_null: bool) -> Self {
        Condition {
            column: T::full_name(),
            operator: if is_null {
                Operator::IsNull
            } else {
                Operator::IsNotNull
            },
            value: None,
        }
    }
}

// Define logical operators for combining conditions
#[derive(Debug, Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

impl Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicalOperator::And => write!(f, "AND"),
            LogicalOperator::Or => write!(f, "OR"),
        }
    }
}

// Define composite condition for complex WHERE clauses
#[derive(Debug, Clone)]
pub enum WhereClause {
    Simple(Condition),
    Composite {
        left: Box<WhereClause>,
        operator: LogicalOperator,
        right: Box<WhereClause>,
    },
}

impl WhereClause {
    pub fn and(self, other: WhereClause) -> WhereClause {
        WhereClause::Composite {
            left: Box::new(self),
            operator: LogicalOperator::And,
            right: Box::new(other),
        }
    }

    pub fn or(self, other: WhereClause) -> WhereClause {
        WhereClause::Composite {
            left: Box::new(self),
            operator: LogicalOperator::Or,
            right: Box::new(other),
        }
    }
}

// Define QueryBuilder for building SQL queries
#[derive(Debug)]
pub struct QueryBuilder<T> {
    table: String,
    where_clause: Option<WhereClause>,
    limit: Option<usize>,
    offset: Option<usize>,
    order_by: Vec<(String, bool)>, // (column, is_ascending)
    _phantom: PhantomData<T>,
}

impl<T: 'static> QueryBuilder<T> {
    pub fn new(table: &str) -> Self {
        QueryBuilder {
            table: table.to_string(),
            where_clause: None,
            limit: None,
            offset: None,
            order_by: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn filter<Q: Queryable, V: 'static>(mut self, column: Q, op: &str, value: V) -> Self {
        let operator = Operator::from_str(op).unwrap_or(Operator::Equal);
        let condition = Condition::new(column, operator, value);
        let where_clause = WhereClause::Simple(condition);

        self.where_clause = match self.where_clause {
            Some(existing) => Some(existing.and(where_clause)),
            None => Some(where_clause),
        };

        self
    }

    pub fn and<Q: Queryable, V: 'static>(self, column: Q, op: &str, value: V) -> Self {
        self.filter(column, op, value)
    }

    pub fn or<Q: Queryable, V: 'static>(mut self, column: Q, op: &str, value: V) -> Self {
        let operator = Operator::from_str(op).unwrap_or(Operator::Equal);
        let condition = Condition::new(column, operator, value);
        let where_clause = WhereClause::Simple(condition);

        self.where_clause = match self.where_clause {
            Some(existing) => Some(existing.or(where_clause)),
            None => Some(where_clause),
        };

        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn order_by(mut self, column: &str, ascending: bool) -> Self {
        self.order_by.push((column.to_string(), ascending));
        self
    }

    // Method to build the SQL query
    pub fn build_query(&self) -> (String, Vec<Box<dyn std::any::Any>>) {
        let mut query = format!("SELECT * FROM {}", self.table);
        let mut params: Vec<Box<dyn std::any::Any>> = Vec::new();

        // Add WHERE clause if exists
        if let Some(where_clause) = &self.where_clause {
            query.push_str(" WHERE ");
            self.build_where_clause(where_clause, &mut query, &mut params);
        }

        // Add ORDER BY if exists
        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            for (i, (column, ascending)) in self.order_by.iter().enumerate() {
                if i > 0 {
                    query.push_str(", ");
                }
                query.push_str(&format!(
                    "{} {}",
                    column,
                    if *ascending { "ASC" } else { "DESC" }
                ));
            }
        }

        // Add LIMIT and OFFSET if exists
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        (query, params)
    }

    fn build_where_clause(
        &self,
        clause: &WhereClause,
        query: &mut String,
        params: &mut Vec<Box<dyn std::any::Any>>,
    ) {
        match clause {
            WhereClause::Simple(condition) => {
                query.push_str(&condition.column);
                query.push_str(&format!(" {} ", condition.operator));

                if let Some(value) = &condition.value {
                    query.push_str("?");
                    params.push(value.clone());
                }
            }
            WhereClause::Composite {
                left,
                operator,
                right,
            } => {
                query.push_str("(");
                self.build_where_clause(left, query, params);
                query.push_str(&format!(" {} ", operator));
                self.build_where_clause(right, query, params);
                query.push_str(")");
            }
        }
    }
}

