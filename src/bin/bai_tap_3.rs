/////////////////////////////////////
// Bài 3
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////

trait AppendBar {
    fn append_bar(&mut self) -> Vec<String>;
}
//TODO: Add your code here
impl AppendBar for Vec<String> {
    fn append_bar(&mut self) -> Self {
        &self.push("Bar".to_owned());
        self.to_vec()
    }
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
