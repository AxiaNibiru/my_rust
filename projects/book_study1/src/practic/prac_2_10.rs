#[allow(unused, dead_code)]
pub fn prac_2_10() {
    fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }
    let string1 = String::from("abc");
    let string2 = "xyz";
    let result = longest(&string1, string2);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    struct ImportantExcerpt<'a> {
        party: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(",").next().expect("Could not find '.'");
    let i = ImportantExcerpt {
        party: first_sentence,
    };

    impl<'a> ImportantExcerpt<'a> {
        fn ann<'b>(&'a self, ann: &'b str) -> &'a str {
            println!("{}", ann);
            self.party
        }
    }

    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn bnn(&'a self, bnn: &'b str) -> &'b str {
            println!("{}", bnn);
            self.party
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn cnn<'b>(&'a self, bnn: &'b str) -> &'b str
        where
            'a: 'b,
        {
            println!("{}", bnn);
            self.party
        }
    }

    fn longest_with<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: std::fmt::Display,
    {
        println!("{}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

