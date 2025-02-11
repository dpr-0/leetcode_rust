pub mod binary_search;
pub mod car_fleet;
pub mod container_with_most_water;
pub mod convert_an_array_into_a_2d_array_with_conditions;
pub mod daily_temperatures;
pub mod encode_and_decode_strings;
pub mod evaluate_reverse_polish_notation;
pub mod find_minimum_in_rotated_sorted_array;
pub mod generate_parentheses;
pub mod group_anagrams;
pub mod koko_eating_bananas;
pub mod largest_rectangle_in_histogram;
pub mod longest_consecutive_sequence;
pub mod maximum_number_of_words_found_in_sentences;
pub mod min_stack;
pub mod minimum_difficulty_of_a_job_schedule;
pub mod product_of_array_except_self;
pub mod redistribute_characters_to_make_all_strings_equal;
pub mod reverse_words_in_a_string;
pub mod search_a_2d_matrix;
pub mod three_sum;
pub mod top_k_frequent_elements;
pub mod trapping_rain_water;
pub mod two_sum;
pub mod two_sum_ii_input_array_is_sorted;
pub mod valid_palindrome;
pub mod valid_parentheses;
pub mod valid_sudoku;
#[cfg(test)]
mod tests {
    pub struct StringVec;
    impl StringVec {
        pub fn from<const N: usize>(s: [&str; N]) -> Vec<String> {
            s.into_iter().map(String::from).collect()
        }
    }
}
