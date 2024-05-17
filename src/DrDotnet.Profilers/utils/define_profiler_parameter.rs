use crate::{ParameterType, ProfilerParameter};

pub trait DefineParameter {
    fn define(&self, name: &str, key: &str, description: &str) -> ProfilerParameter;
}

impl DefineParameter for u64 {
    fn define(&self, name: &str, key: &str, description: &str) -> ProfilerParameter {
        ProfilerParameter {
            name: name.to_owned(),
            key: key.to_owned(),
            description: description.to_owned(),
            type_: ParameterType::INT.into(),
            value: self.to_string().to_owned(),
            ..std::default::Default::default()
        }
    }
}

impl DefineParameter for bool {
    fn define(&self, name: &str, key: &str, description: &str) -> ProfilerParameter {
        ProfilerParameter {
            name: name.to_owned(),
            key: key.to_owned(),
            description: description.to_owned(),
            type_: ParameterType::BOOLEAN.into(),
            value: self.to_string().to_owned(),
            ..std::default::Default::default()
        }
    }
}

impl DefineParameter for f64 {
    fn define(&self, name: &str, key: &str, description: &str) -> ProfilerParameter {
        ProfilerParameter {
            name: name.to_owned(),
            key: key.to_owned(),
            description: description.to_owned(),
            type_: ParameterType::FLOAT.into(),
            value: self.to_string().to_owned(),
            ..std::default::Default::default()
        }
    }
}

impl DefineParameter for &str {
    fn define(&self, name: &str, key: &str, description: &str) -> ProfilerParameter {
        ProfilerParameter {
            name: name.to_owned(),
            key: key.to_owned(),
            description: description.to_owned(),
            type_: ParameterType::BOOLEAN.into(),
            value: self.to_string().to_owned(),
            ..std::default::Default::default()
        }
    }
}

impl ProfilerParameter {
    pub fn define<T: DefineParameter>(name: &str, key: &str, default_value: T, description: &str) -> ProfilerParameter {
        default_value.define(name, key, description)
    }
}
