use pgrx::prelude::*;

pgrx::pg_module_magic!();

pgrx::extension_sql!(
    "src/pg_aim.sql",
    name = "pg_aim",
    bootstrap
);

#[pg_extern]
fn aim_version() -> String {
    "0.1.0".to_string()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_aim_version() {
        assert_eq!("0.1.0", crate::aim_version());
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
