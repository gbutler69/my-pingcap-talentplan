#[test]
fn doc_test_package() {
    use crate::KvStore;

    let mut store = KvStore::<String, String>::open(std::path::Path::new("testdb")).unwrap();

    let _ = store.set(String::from("key1"), String::from("value1"));
    let value1 = store.get(String::from("key1")).unwrap();
    assert_eq!(value1, Some("value1".into()));

    let value2 = store.get(String::from("key2")).unwrap();
    assert!(value2.is_none());

    let _ = store.remove(String::from("key1"));
    let value1 = store.get(String::from("key1")).unwrap();
    assert_eq!(value1, None);
}
