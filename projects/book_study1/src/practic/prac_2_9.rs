#[allow(unused, dead_code)]
fn prac_2_9_2() {
    use std::collections::HashMap;
    let mut hash_map: HashMap<&str, i32> = HashMap::new();
    hash_map.insert("key1", 1);
    hash_map.insert("key2", 2);
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<String, i32> = teams_list.into_iter().collect();
    Some(&12).copied(); // 12 -> i32，copied内容需要实现Copy trait

    let text = "hello world wonderful world";
    let mut map: HashMap<&str, usize> = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count: &mut usize = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    use std::hash::BuildHasherDefault;
    use twox_hash::XxHash64;

    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
    println!("{:?}", hash);

    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // 使用两种方法实现 team_map2
    // 提示:其中一种方法是使用 `collect` 方法
    // let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();
    let mut teams_map2: HashMap<&str, i32> = HashMap::with_capacity(3);
    teams.into_iter().for_each(|(k, v)| {
        teams_map2.insert(k, v);
    });

    assert_eq!(teams_map1, teams_map2);

    teams_map1.entry("").or_insert_with(|| 45);
    println!("Success!");

    let mut map = HashMap::new();
    map.insert("a".to_string(), "asd");
    map.get("a");

    struct A {}
    impl A {
        fn a(a: &str) {}
        //fn a(a: String) {} Error
    }

    #[derive(PartialEq, Eq, Hash)]
    struct Viking {
        name: String,
        country: String,
    }
    impl Viking {
        fn new(name: &str, country: &str) -> Viking {
            Viking {
                name: name.to_string(),
                country: country.to_string(),
            }
        }
    }
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    assert_eq!("a", "a".to_string());
}

#[allow(unused, dead_code)]
fn prac_2_9_1() {
    let v = vec![1, 2, 3];
    if let Some(value) = v.get(2) {
        println!("{value}");
    }

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);

    // 实现 Ord 需要我们实现 Ord、Eq、PartialEq、PartialOrd 这些属性
    // 默认的Ord排序规则为按字段顺序来进行排序，下面的话就是先按name排序，如果相等则使用age进行排序
    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Self {
            Person {
                name: name,
                age: age,
            }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 20),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
    println!("{:?}", &people);
    people.sort_unstable();
    println!("{:?}", people);
    let arr: [u32; 3] = [1, 2, 3];
    arr.to_vec();
    let mut v1: Vec<u32> = Vec::new();
    arr.map(|a| v1.push(a));
    println!("{:?}", arr);
    println!("{:?}", v1);
    v1.extend(v.iter());
    let v2: Vec<u32> = arr.into();
    // let str_vec: Vec<u32> = "String".to_string().into();
    Vec::from("");

    // 修复错误并实现缺失的代码
    let mut v = Vec::from([1, 2, 3, 4, 5]);
    for i in 0..5 {
        println!("{:?}", v[i])
    }

    for i in 0..5 {
        v[i] += 1;
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}
