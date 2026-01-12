# algorithgms-and-data-structures-note

Knuth-Morris-Pratt (KMP) Algorithm - substring search
https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/solutions/7210141/video-easy-to-understand-kmp-on-c-java-p-3f2x/?source=submission-ac

Repeated Substring Pattern
(String Doubling Trick)
https://leetcode.com/problems/repeated-substring-pattern/solutions/3938580/9942-2-approaches-on-by-vanamsen-dqwu/?envType=study-plan-v2&envId=programming-skills
// codecode
// odecod
// code

// abcabc
// abcabcabcabc
// bcabcabcab -- abcabc
```rust
pub fn repeated_substring_pattern(s: String) -> bool {
    let doubled = s.clone() + &s;
    // remove character from beginning and end
    let substring = &doubled[1..doubled.len()-1];
    substring.contains(&s)
}
```
