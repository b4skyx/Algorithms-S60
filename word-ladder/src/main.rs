use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // Creating a HashSet for faster lookup to match changed word.
        let mut lookup = HashSet::new();
        for word in word_list {
            lookup.insert(word);
        }
        // Initially count = 1 since minimum 1 transition is needed
        let mut count = 1;
        let len = begin_word.len();
        //initailize a bfs with starting word
        let mut bfs = vec![begin_word];
        while !bfs.is_empty() {
            // rem = count of the number of nodes in present layer
            let rem = bfs.len();
            // iterate through present layer of bfs
            for _i in 0..rem {
                // select the word
                let current = bfs.remove(0);
                // possible changes can be made at 0 to len
                for j in 0..len {
                    // try changing from a to z
                    for k in b'a'..b'z' + 1 {
                        let temp = format!(
                            "{}{}{}",
                            current.get(0..j).unwrap(),
                            k as char,
                            current.get(j + 1..len).unwrap()
                        );
                        // check if by changing one character, we come to any succesfull word_list match
                        if lookup.contains(&temp) {
                            // if match == end_word, return count + 1 more transition for match as ans
                            if temp == end_word {
                                return count + 1;
                            }
                            // else remove the word from table as it has already been traversed to avoid loopback
                            lookup.remove(&temp);
                            // push the traversed word in bfs
                            bfs.push(temp);
                        }
                    }
                }
            }
            // increase count by 1 as after succesfully going through one layer of bfs, next layer has +1 transition
            count += 1;
        }
        // return 0 if none satisfies
        0i32
    }
}
fn main() {
    unimplemented!();
}
