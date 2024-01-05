struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn encode(strs: Vec<&str>) -> String {
        let mut encoded = String::new();
        for s in strs {
            let encoded_s = format!("{}#{}", s.chars().count(), s);
            encoded.push_str(&encoded_s);
        }
        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        let mut decoded = Vec::new();
        let mut i = 0;
        while i < s.len() {
            let mut j = i;
            while s[j] != '#' {
                j += 1;
            }
            // println!("{} {}, {:?}", i, j, s[i..j].to_string());
            let len: usize = String::from_iter(&s[i..j]).parse().unwrap();
            j += 1;
            decoded.push(String::from_iter(&s[j..j + len]));
            i = j + len;
        }
        decoded
    }
}

#[test]
fn test_encode_decode_string() {
    let cases = vec![
        vec!["neet", "code", "love", "you"],
        vec!["we", "say", ":", "yes"],
        vec![""],
        vec![],
        vec!["we", "say", ":", "yes", "!@#$%^&*()"],
        vec!["#", "##"],
        vec!["1,23", "45,6", "7,8,9"],
        vec!["hello","world","hello"],
        vec!["a","b","c","d"],
        vec!["@#$","%^&*","!@#$%^&*"],
        vec!["apple","orange","banana","grape","kiwi","melon"],
        vec!["The quick brown fox","jumps over the","lazy dog","1234567890","abcdefghijklmnopqrstuvwxyz"],
        vec!["","   ","!@#$%^&*()_+","LongStringWithNoSpaces","Another, String With, Commas"],
        vec!["String with new\nline","Another\nLine","And\nOne\nMore"],
        vec!["123","456","789","10","11","12","13","14","15","16","17","18","19","20"],
        vec!["Repeated","Repeated","Repeated","Repeated","Repeated","Repeated"],
        vec!["SingleCharacter","A","B","C","D","E","F","G","H","I","J"],
        vec!["EmojiTest 😊","🌟✨🌟✨🌟","🤖👽🤖👽"],
        vec!["Strings with spaces are tricky","Another one with spaces","This also has spaces"],
        vec!["VeryLongStringWithoutAnySpacesOrSpecialCharactersRepeatedManyTimesVeryLongStringWithoutAnySpacesOrSpecialCharactersRepeatedManyTimes"],
    ];
    for case in cases {
        assert_eq!(Solution::decode(Solution::encode(case.clone())), case);
    }
}
