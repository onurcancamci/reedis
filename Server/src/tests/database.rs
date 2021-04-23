mod mock;

use crate::common_traits::*;
use crate::data::*;
use crate::error::MyError;
use mock::*;
use std::sync::{Arc, RwLock};

#[test]
fn get_non_existent_field() {
    let db = MockDatabase::new();
    let get_command = MockCommand::new_get("test");
    let context = Arc::new(RwLock::new(MockExecutionContext {}));

    let result = db.run(context, get_command);
    assert_eq!(result.unwrap_err(), MyError::KeyNotFound);
}

#[test]
fn set_field() {
    let mut db = MockDatabase::new();
    let context = Arc::new(RwLock::new(MockExecutionContext {}));
    let set_command = MockCommand::new_set("test", Data::Int(42));

    let result = db.run_mutable(context, set_command);

    let result = result.unwrap();

    assert_eq!(result.modified_row_count(), 1);
    assert_eq!(result.results().unwrap().count(), 0);
}

#[test]
fn set_get_basic_values() {
    let mut db = MockDatabase::new();
    let context = Arc::new(RwLock::new(MockExecutionContext {}));

    let sets = vec![
        MockCommand::new_set("int", Data::Int(42)),
        MockCommand::new_set("float", Data::Float(0.1)),
        MockCommand::new_set("bool", Data::Bool(true)),
        MockCommand::new_set("str", Data::Str("test".to_string())),
    ];

    let gets = vec![
        MockCommand::new_get("int"),
        MockCommand::new_get("float"),
        MockCommand::new_get("bool"),
        MockCommand::new_get("str"),
    ];

    for s in sets {
        let result = db.run_mutable(Arc::clone(&context), s).unwrap();
        assert_eq!(result.modified_row_count(), 1);
    }

    for (ind, g) in gets.into_iter().enumerate() {
        let result = db.run(Arc::clone(&context), g).unwrap();
        let mut results = result.results().unwrap();
        match results.next().unwrap() {
            Data::Int(x) => {
                assert_eq!(x, &42);
                assert_eq!(ind, 0);
            }
            Data::Float(x) => {
                assert_eq!(x, &0.1);
                assert_eq!(ind, 1);
            }
            Data::Bool(x) => {
                assert_eq!(x, &true);
                assert_eq!(ind, 2);
            }
            Data::Str(x) => {
                assert_eq!(x, &"test".to_string());
                assert_eq!(ind, 3);
            }
            _ => panic!("Impossible variants"),
        }
    }
}

#[test]
fn set_get_table() {
    let mut db = MockDatabase::new();
    let context = Arc::new(RwLock::new(MockExecutionContext {}));

    let mut table_in = MockTable::new();
    table_in.insert_data("int", Data::Int(42)).unwrap();
    let table_compare = table_in.clone();

    let set = MockCommand::new_set("test", Data::Table(table_in));
    let get = MockCommand::new_get("test");
    let get_int = MockCommand::new_get("test/int");

    let result = db.run_mutable(Arc::clone(&context), set).unwrap();
    assert_eq!(result.modified_row_count(), 1);

    let get_result = db.run(Arc::clone(&context), get).unwrap();

    assert_eq!(
        get_result
            .results()
            .unwrap()
            .next()
            .unwrap()
            .table()
            .unwrap(),
        &table_compare
    );

    let get_result = db.run(Arc::clone(&context), get_int).unwrap();

    assert_eq!(
        get_result.results().unwrap().next().unwrap().int().unwrap(),
        &42
    );
}

//TODO: test nested different tables equality to ensure complete value equality
