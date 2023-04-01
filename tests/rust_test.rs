// 专门用来确认语法特性


#[test]
fn rust_test() {
  let v1 = vec![1, 2, 3, 3, 4];

  let mut v2 = Vec::new();


  v2.push(1);
  v2.push(2);
  v2.push(3);
  v2.push(3);
  v2.push(4);

  assert_eq!(v1, v2);
  assert!(v1 == v2);
}




