# Backtracking --- Complete Lesson

### Using LeetCode: Letter Case Permutation (Rust)

------------------------------------------------------------------------

## 1. What Is Backtracking?

Backtracking is a problem-solving technique where we:

1.  Make a choice
2.  Recurse (go deeper)
3.  Undo the choice
4.  Try another choice

It is typically used when we must generate **all possible combinations,
permutations, or configurations**.

------------------------------------------------------------------------

## 2. The Example Problem

### Letter Case Permutation

Given a string `s`, return all possible strings where each letter can be
either lowercase or uppercase.

Example:

Input: "a1b2"

Output: \["a1b2","a1B2","A1b2","A1B2"\]

Observation: - Each letter → 2 choices (lower / upper) - Each digit → 1
choice

This forms a decision tree.

------------------------------------------------------------------------

## 3. The Code

``` rust
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut chars = s.chars().collect();
        
        Self::helper(0, &mut res, &mut chars);

        res
    }

    pub fn helper(idx: usize, res: &mut Vec<String>, chars: &mut Vec<char>) {
        if idx == chars.len() {
            res.push(chars.iter().collect());
            return;
        }

        if chars[idx].is_alphabetic() {
            chars[idx] = chars[idx].to_ascii_lowercase();
            Self::helper(idx + 1, res, chars);

            chars[idx] = chars[idx].to_ascii_uppercase();
            Self::helper(idx + 1, res, chars);
        } else {
            Self::helper(idx + 1, res, chars);
        }
    }
}
```
```rust
pub fn generate_codes(idx: i32, cur: String, res: &mut Vec<String>) {
    if idx == 0 {
        res.push(cur);
        return;
    }
    Self::generate_codes(idx - 1, format!("{cur}0"), res);
    Self::generate_codes(idx - 1, format!("{cur}1"), res);
}
```

------------------------------------------------------------------------

## 4. Base Case

When `idx == chars.len()`: - Convert current state to string - Save it -
Return

------------------------------------------------------------------------

## 5. Backtracking Logic

If character is alphabetic: - Try lowercase - Recurse - Try uppercase -
Recurse

If digit: - Recurse once

This explores every path in the decision tree using DFS.

------------------------------------------------------------------------

## 6. Complexity

Let: - n = string length - L = number of letters

Time Complexity: O(2\^L)

Space Complexity: O(n) recursion depth\
O(2\^L \* n) output size

------------------------------------------------------------------------

## 7. Backtracking Template

``` rust
fn backtrack(state) {
    if solution_found {
        save_solution;
        return;
    }

    for choice in possible_choices {
        apply(choice);
        backtrack(updated_state);
        undo(choice);
    }
}
```

------------------------------------------------------------------------

## 8. Key Takeaways

-   Backtracking = DFS on a decision tree
-   Define state, choices, and base case clearly
-   Modify → recurse → restore
-   Used in subsets, permutations, N-Queens, combination sum, etc.

------------------------------------------------------------------------

# Final Mental Model

Backtracking systematically explores every configuration by making a
choice, going deeper, and trying alternatives when necessary.
