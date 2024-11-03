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

// fn factorial(n: u32) -> u32 {
//     let mut counter = n;
//     let mut fact: u32 = 0;
//     if counter > 0 {
//         fact += n * (counter - 1);
//         counter -= 1;
//         println!("Current factorial: {}", fact);
//         println!("Current count: {}", counter);
//         factorial(counter);
//     } else {
//         return 1;
//     }
//     fact
// }

// fn factorial(n: u32) -> u32 {
//     let mut counter = 1;
//     let mut fac: u32 = 0;
//     if counter < n && counter > 0 {
//         fac += n * counter;
//         println!("Current factorial: {}", fac);
//         println!("Current count: {}", counter);
//         counter += 1;
//         factorial(n);
//     } else {
//         return 1;
//     }
//     fac
// }

/*
   n is the initial value to calculate the factorial
   n needs to be decremented for each recursive function call until 1 or 0
   and multiplied by the previous value of n
*/

fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
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
