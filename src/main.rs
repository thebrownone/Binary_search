fn binary_search(a: &[i32], item: i32) -> Option<i32> {
    println!("Searching for {}, in an array of length {}", item, a.len());
    let mut low = 0;
    let mut high = a.len() - 1;
    let mut ticker = 0;
    while low <= high {
        ticker += 1;
        let mid = (low + high) / 2;
        println!("mid: {}", mid);
        let guess = a[mid];
        if guess == item {
            println!("You got it in {} guesses!", ticker);
            println!("Low: {} High: {} guess: {}", low, high, guess);
            return Some(mid as i32);
        }
        println!("Low: {} High: {} guess: {}", low, high, guess);
        low = if guess <= item { mid + 1 } else { low };
        high = if guess > item { mid - 1 } else { high };
    }
    return None;
}

fn main () {
    let array = [1, 3, 5, 7, 9];
    let item = 3;
    let result = binary_search(&array, item);
    println!("Result: {:?}", result);

}


// summarise the use of this binary search algorithm
// this is a recursive algorithm that uses the divide and conquer approach 
// to find the index of an item in an array of integers 
// the array must be sorted in ascending order
// the algorithm works by dividing the array in half and checking if the item is in the first half or the second half
// if the item is in the first half, the algorithm will repeat the process on the first half of the array
// if the item is in the second half, the algorithm will repeat the process on the second half of the array
// the algorithm will repeat this process until the item is found or the array is empty

