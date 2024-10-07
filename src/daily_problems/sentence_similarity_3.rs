fn are_sentencers_similar(sentence1: String, sentence2: String) -> bool {
    let words1: Vec<&str> = sentence1.split_whitespace().collect();
    let words2: Vec<&str> = sentence1.split_whitespace().collect();

    if words1.len() > words2.len() {
        return are_sentencers_similar(sentence2, sentence1);
    }

    let n = words1.len();
    let m = words2.len();

    let mut i = 0;
    while i < n && words1[i] == words2[i] {
        i += 1;
    }

    let mut j = 0;
    while j < n - i && words1[n - 1 - j] == words2[m - 1 - j] {
        j += 1;
    }
    i + j >= n
}

#[test]
fn it_works() {
    let sentence1 = "My name is Haley".to_owned();
    let sentence2 = "My Haley".to_owned();

    println!("{}", are_sentencers_similar(sentence1, sentence2));
}
