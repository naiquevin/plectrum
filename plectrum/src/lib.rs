use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

#[derive(Debug)]
pub enum Error {
    NotDefinedInCode,
    NotFoundInDb,
    // @TODO: Can this be optionally supported behind a cargo feature?
    // Sql(sqlx::Error),
}

pub trait Enum {
    fn value(&self) -> &str;
    fn from_value(s: &str) -> Self;
    fn values() -> HashSet<&'static str>;
}

pub trait DataSource {
    fn load(&self)
        -> impl std::future::Future<Output = Result<HashMap<i32, String>, Error>> + Send;
}

pub struct Mapping<E> {
    inner: HashMap<i32, String>,
    _enum_type: PhantomData<E>,
}

impl<E: Enum> Mapping<E> {
    pub async fn load<S: DataSource>(source: &S) -> Result<Self, Error> {
        let data = source.load().await?;
        let label_values = E::values();
        let mut inner: HashMap<i32, String> = HashMap::new();
        for (key, value) in &data {
            inner.insert(*key, value.to_owned());
        }
        let data_values: HashSet<&str> = data.values().map(|v| v.as_str()).collect();
        if label_values.difference(&data_values).count() > 0 {
            return Err(Error::NotFoundInDb);
        }
        Ok(Self {
            inner,
            _enum_type: PhantomData,
        })
    }

    pub fn by_id(&self, id: i32) -> Option<E> {
        self.inner.get(&id).map(|s| E::from_value(s.as_str()))
    }

    pub fn by_value(&self, value: &str) -> Option<E> {
        let mut res = None;
        for (k, v) in self.inner.iter() {
            if v == value {
                res = self.by_id(*k)
            }
        }
        res
    }

    pub fn get_id(&self, label: &E) -> Option<i32> {
        let mut res = None;
        for (k, v) in self.inner.iter() {
            if label.value() == v {
                res = Some(*k)
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum State {
        Stopped,
        Running,
        Stopping,
    }

    impl Enum for State {
        fn values() -> HashSet<&'static str> {
            HashSet::from_iter(vec!["stopped", "running", "stopping"])
        }

        fn from_value(s: &str) -> Self {
            match s {
                "stopped" => Self::Stopped,
                "running" => Self::Running,
                "stopping" => Self::Stopping,
                _ => panic!("Unknown state: {s}"),
            }
        }

        fn value(&self) -> &str {
            match self {
                Self::Stopped => "stopped",
                Self::Running => "running",
                Self::Stopping => "stopping",
            }
        }
    }

    struct StateModel;

    impl DataSource for StateModel {
        async fn load(&self) -> Result<HashMap<i32, String>, Error> {
            let mut m = HashMap::new();
            m.insert(1, "stopped".to_owned());
            m.insert(2, "running".to_owned());
            m.insert(3, "stopping".to_owned());
            Ok(m)
        }
    }

    #[tokio::test]
    async fn test_kvmap() {
        let model = StateModel {};
        let kvmap = Mapping::load(&model).await.unwrap();
        assert_eq!(State::Stopped, kvmap.by_id(1).unwrap());
        assert_eq!(State::Running, kvmap.by_value("running").unwrap());
        assert_eq!(3, kvmap.get_id(&State::Stopping).unwrap());
    }
}
