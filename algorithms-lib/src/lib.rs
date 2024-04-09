mod binary_search;
mod bubble_sort;
mod quick_sort;

#[cfg(test)]
mod tests {
    use self::{binary_search::binary_search_works, bubble_sort::bubble_sort_works, quick_sort::quick_sort_works};

    use super::*;

    #[test]
    fn it_works() {
       quick_sort_works(); 
       binary_search_works();
       bubble_sort_works();
    }
}
