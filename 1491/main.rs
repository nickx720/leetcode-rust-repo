impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut salary = salary;
        salary.sort();
        let slice_salary = &salary[1..salary.len() - 1];
        let total: i32 = slice_salary.iter().sum();
        let count = slice_salary.len() as f64;
        total as f64 / count
    }
}
