fn main() {
    let s = "Er ist noch nicht erklärt. Aber es gibt Krieg. Verlaßdich drauf.";

    for (i, j) in s.match_indices(|c| c > 'z') {
        println!("{j} at index {i}");
    }

    for (i, j) in s.match_indices(". ") {
        println!("{j} at index {i}");
    }
}
