impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {

        let mut check: i32 = 3;
        let mut count: i32 = 0;
        let mut res: String = String::new();
        let mut res_2: String = String::new();

        for word in word1
            {
                res.push_str(&word);
            }

        for word in word2
            {
                res_2.push_str(&word);
            }

        return res == res_2
        
    }
}