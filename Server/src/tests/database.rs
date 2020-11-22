mod mock;

use crate::common_traits::*;
use crate::data::*;
use crate::error::MyError;
use mock::*;

#[test]
fn get_non_existent_field() {
    let db = MockDatabase::new();
    let get_command = MockCommand::new_get("test");

    let result = db.run(get_command);
    assert_eq!(result.unwrap_err(), MyError::KeyNotFound);
}

#[test]
fn set_field() {
    let mut db = MockDatabase::new();
    let set_command = MockCommand::new_set("test", Data::Int(42));

    let result = db.run_mutable(set_command);

    let result = result.unwrap();

    assert_eq!(result.0.modified_row_count(), 1);
    assert_eq!(result.0.result().kind(), ResultTypeVariant::None);
}

#[test]
fn set_get_basic_values() {
    let mut db = MockDatabase::new();

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
        let result = db.run_mutable(s).unwrap();
        assert_eq!(result.0.modified_row_count(), 1);
    }

    for (ind, g) in gets.into_iter().enumerate() {
        let result = db.run(g).unwrap();
        match result.0.result().data().unwrap() {
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
