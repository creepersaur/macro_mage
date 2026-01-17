#[allow(unused)]
#[derive(Clone)]
pub enum Value {
	NIL,
	String(String),
	Float(f32),
	Int(i32),
	Bool(bool)
}

impl ToString for Value {
	fn to_string(&self) -> String {
		match self {
			Value::NIL => String::from("nil"),
			Value::String(x) => x.clone(),
			Value::Float(x) => x.to_string(),
			Value::Int(x) => x.to_string(),
			Value::Bool(x) => x.to_string(),
		}
	}
}