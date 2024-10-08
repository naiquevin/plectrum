use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

/// Error representing all the ways that `Mapping::load` can fail
#[derive(Debug)]
pub enum Error {
    // If an enum variant is not defined for a value found in the db
    NotDefinedInCode,
    // If a value corresponding to an enum variant is not found in the db
    NotFoundInDb,
    // Custom error when fetching data from the data source
    DataSource(String),

    #[cfg(feature = "sqlx")]
    Sqlx(sqlx::Error),
}

pub trait Enum {
    fn value(&self) -> &str;
    fn from_value(s: &str) -> Self;
    fn values() -> HashSet<&'static str>;
}

pub trait DataSource {
    type Id: std::hash::Hash + Eq + Copy;
    fn load(
        &self,
    ) -> impl std::future::Future<Output = Result<HashMap<Self::Id, String>, Error>> + Send;
}

pub struct Mapping<K, E> {
    inner: HashMap<K, String>,
    _enum_type: PhantomData<E>,
}

impl<K: std::hash::Hash + Eq + Copy, E: Enum> Mapping<K, E> {
    pub async fn load<S: DataSource<Id = K>>(source: &S) -> Result<Self, Error> {
        let data = source.load().await?;
        let enum_values = E::values();
        let mut inner: HashMap<K, String> = HashMap::new();
        for (key, value) in &data {
            inner.insert(*key, value.to_owned());
            if !enum_values.contains(value.as_str()) {
                return Err(Error::NotDefinedInCode);
            }
        }
        let data_values: HashSet<&str> = data.values().map(|v| v.as_str()).collect();
        if enum_values.difference(&data_values).count() > 0 {
            return Err(Error::NotFoundInDb);
        }
        Ok(Self {
            inner,
            _enum_type: PhantomData,
        })
    }

    pub fn by_id(&self, id: K) -> Option<E> {
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

    pub fn get_id(&self, label: &E) -> Option<K> {
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
        type Id = i32;
        async fn load(&self) -> Result<HashMap<i32, String>, Error> {
            let mut m = HashMap::new();
            m.insert(1, "stopped".to_owned());
            m.insert(2, "running".to_owned());
            m.insert(3, "stopping".to_owned());
            Ok(m)
        }
    }

    struct StateModelMissingValues;

    impl DataSource for StateModelMissingValues {
        type Id = i32;
        async fn load(&self) -> Result<HashMap<i32, String>, Error> {
            let mut m = HashMap::new();
            m.insert(1, "stopped".to_owned());
            m.insert(2, "running".to_owned());
            Ok(m)
        }
    }

    struct StateModelExtraValues;

    impl DataSource for StateModelExtraValues {
        type Id = i32;
        async fn load(&self) -> Result<HashMap<i32, String>, Error> {
            let mut m = HashMap::new();
            m.insert(1, "stopped".to_owned());
            m.insert(2, "running".to_owned());
            m.insert(2, "stopping".to_owned());
            m.insert(2, "waiting".to_owned());
            Ok(m)
        }
    }

    #[tokio::test]
    async fn test_mapping_happy_path() {
        let model = StateModel {};
        let mapping = Mapping::load(&model).await.unwrap();
        assert_eq!(State::Stopped, mapping.by_id(1).unwrap());
        assert_eq!(State::Running, mapping.by_value("running").unwrap());
        assert_eq!(3, mapping.get_id(&State::Stopping).unwrap());
    }

    #[tokio::test]
    async fn test_mapping_errors_when_loading() {
        let model = StateModelMissingValues {};
        match Mapping::<_, State>::load(&model).await {
            Err(Error::NotFoundInDb) => assert!(true),
            _ => assert!(false),
        }

        let model = StateModelExtraValues {};
        match Mapping::<_, State>::load(&model).await {
            Err(Error::NotDefinedInCode) => assert!(true),
            _ => assert!(false),
        }
    }
}

// Gated behind the 'derive' cargo feature, re-export the `Plectrum`
// macro so that the users of the lib don't need to add
// `plectrum_derive` as a dependency

#[cfg(feature = "derive")]
pub use plectrum_derive::Plectrum;
