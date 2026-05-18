use openmodelica_util_datatypes_basic::Mutable;

#[test]
fn test_create() {
    let m = Mutable::create(42);
    let val = Mutable::access(m);
    assert_eq!(val, 42);
}

#[test]
fn test_update() {
    let m = Mutable::create(10);
    Mutable::update(m.clone(), 20);
    let val = Mutable::access(m);
    assert_eq!(val, 20);
}

#[test]
fn test_access() {
    let m = Mutable::create(String::from("hello"));
    let val = Mutable::access(m);
    assert_eq!(val, "hello");
}

#[test]
fn test_eq() {
    let m1 = Mutable::create(5);
    let m2 = Mutable::create(5);
    let m3 = Mutable::create(10);
    assert!(m1 == m2);
    assert!(m1 != m3);

    // After update, should no longer equal
    Mutable::update(m1.clone(), 10);
    assert!(m1 == m3);
}

#[test]
fn test_clone_shares_state() {
    let m = Mutable::create(100);
    let m2 = m.clone();
    Mutable::update(m, 200);
    let val = Mutable::access(m2);
    assert_eq!(val, 200);
}

#[test]
fn test_update_then_access_chain() {
    let m = Mutable::create(0);
    for i in 1..=10 {
        Mutable::update(m.clone(), i);
        assert_eq!(Mutable::access(m.clone()), i);
    }
}
