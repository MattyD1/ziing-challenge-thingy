pub struct IntList {
    pub list: Vec<i32>,
}

impl IntList {
    pub fn new(list: Vec<i32>) -> Self {
        return IntList { list };
    }

    pub fn new_fib(length: i32) -> Self {
        let mut a = 0;
        let mut b = 1;
        let mut fib_list: Vec<i32> = Vec::new();

        for _ in 0..length {
            fib_list.push(a);
            // Rust destructuring assignment syntax
            // a = b; and b = a + b;
            (a, b) = (b, a + b);
        }
        return IntList { list: fib_list };
    }

    pub fn sum_evens(&self) -> i32 {
        let mut sum = 0;
        for i in &self.list {
            if i % 2 == 0 {
                sum += i;
            }
        }
        return sum;
    }

    pub fn sum_divisible_by_three(&self) -> i32 {
        let mut sum = 0;
        for i in &self.list {
            if i % 3 == 0 {
                sum += i;
            }
        }
        return sum;
    }

    pub fn second_largest(&self) -> Option<&i32> {
        // Make sure there are at least 2 elements in the list
        if self.list.len() < 2 {
            return None;
        }

        let mut largest = &self.list[0];
        let mut second_largest = &self.list[1];

        // Make sure largest is actually the largest
        if second_largest > largest {
            (largest, second_largest) = (second_largest, largest);
        }

        // Iterate through the list and find the second largest
        for i in &self.list {
            if i > largest {
                second_largest = largest;
                largest = i;
            } else if i > second_largest && i != largest {
                second_largest = i;
            } else if second_largest == largest && i < largest {
                second_largest = i;
            }
        }

        // If the largest and second largest are the same, then all values are the same and there is no second largest
        if largest == second_largest {
            return None;
        }

        return Some(second_largest);
    }

    pub fn dislay(&self) {
        for i in &self.list {
            print!("{} ", i);
        }
    }
}
