// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!

/*
✅ Base case at n == 0 returning 1 is correct
✅ Understanding that n > 0 is the recursive case is spot on
✅ Your formula factorial(n) = factorial(n-1) * n is perfect
✅ You correctly identified that n decrements towards 0
✅ Your understanding of using the recursive result * n is correct
Your mental model about how the recursion will work is solid. Each recursive call will:

Get smaller (n-1)
Eventually hit the base case (0)
Then "unwind" back up, multiplying by each n as it goes */

// fn factorial(n: u32) -> u32 {
//     let result: u32;
//     if n == 0 {
//         return 1;
//     } else {
//         result = factorial(n - 1) * n;
//     }

//     result
// }

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        factorial(n - 1) * n
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
