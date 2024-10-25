pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    let mut sorted_folders = folder;
    println!("before {sorted_folders:?} ");
    sorted_folders.sort();

    println!("after {sorted_folders:?}");

    let mut result = Vec::new();

    if let Some(first) = sorted_folders.first() {
        result.push(first.clone());
    }

    (1..sorted_folders.len()).for_each(|i| {
        let current = &sorted_folders[i];
        let last = result.last().unwrap();

        let is_subfolder =
            current.starts_with(last) && current.chars().nth(last.len()) == Some('/');

        if !is_subfolder {
            result.push(current.clone());
        }
    });
    result
}

#[test]
fn remove_subfolders_works() {
    let input1 = vec![
        "/c/f".to_string(),
        "/a".to_string(),
        "/c/d".to_string(),
        "/a/b".to_string(),
        "/c/d/e".to_string(),
    ];
    let mut expected1 = vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()];
    let mut result1 = remove_subfolders(input1);
    result1.sort();
    expected1.sort();
    assert_eq!(result1, expected1);
}
