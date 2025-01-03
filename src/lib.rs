#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_export]
macro_rules! str {
    ($s:expr) => (String::from($s));
}

#[macro_export]
macro_rules! tree {
    ($s:expr) => {
        {
            let codec = crate::serialize_and_deserialize_binary_tree::Codec::new();
            let data = String::from($s);
            codec.deserialize(data)
        }
    };
}

#[macro_export]
macro_rules! assert_tree {
    ($s:expr, $t:expr) => {
        {
            let codec = crate::serialize_and_deserialize_binary_tree::Codec::new();
            assert_eq!(codec.serialize($s), $t)
        }
    };
}

pub mod interval;
pub mod kth_largest;
pub mod lowercase_letter_counter;
pub mod list_node;
pub mod list_node_additions;
pub mod quick_find;
pub mod quick_union;
pub mod union_by_rank;
pub mod union_find_path_compression;
pub mod union_find; // Optimized
pub mod stack;
pub mod tree_node;
pub mod tree_node_additions;
pub mod vec_additions;
pub mod adjacency_graph;
pub mod heap_array;
pub mod number_additions;
pub mod linked_list;

pub mod two_sum; // 1 ✓
pub mod add_two_numbers; // 2
pub mod longest_substring_without_repeating_characters; // 3 ✓
pub mod median_of_two_sorted_arrays; // 4 ✓
pub mod longest_palindromic_substring; // 5 ✓
pub mod zigzag_conversion; // 6 ✓
pub mod reverse_integer; // 7 ✓
pub mod string_to_integer; // 8 ✓
pub mod palindrome_number; // 9 ✓
pub mod regular_expression_matching; // 10
pub mod container_with_most_water; // 11 ✓
pub mod integer_to_roman; // 12 ✓
pub mod roman_to_integer; // 13 ✓
pub mod longest_common_prefix; // 14
pub mod three_sum; // 15
pub mod three_sum_closest; // 16 ✓
pub mod letter_combinations_of_a_phone_number; // 17 ✓

pub mod remove_nth_node_from_end_of_list; // 19 ✓
pub mod valid_parentheses; // 20
pub mod merge_two_sorted_lists; // 21
pub mod generate_parentheses; // 22 ✓
pub mod merge_k_sorted_lists; // 23 ✓
pub mod swap_nodes_in_pairs; // 24
pub mod reverse_nodes_in_k_groups; // 25 ✓
pub mod remove_duplicates_from_sorted_array; // 26
pub mod remove_element; // 27
pub mod find_the_index_of_the_first_occurrence_in_a_string; // 28

pub mod next_permutation; // 31 ✓

pub mod search_in_rotated_sorted_array; // 33 ✓

pub mod search_insert_position; // 35
pub mod valid_sudoku; // 36 ✓

pub mod count_and_say; // 38 ✓
pub mod combination_sum; // 39 ✓
pub mod combination_sum_ii; // 40 ✓
pub mod first_missing_positive; // 41 ✓
pub mod trapping_rain_water; // 42 ✓
pub mod multiply_strings; // 43 ✓

pub mod jump_game_ii; // 45 ✓
pub mod permutations; // 46 ✓
pub mod permutations_ii; // 47 ✓
pub mod rotate_image; // 48
pub mod group_anagrams; // 49 ✓
pub mod pow_x_n; // 50
pub mod n_queens; // 51 ✓
pub mod n_queens_ii; // 52 ✓
pub mod maximum_subarray; // 53 ✓

pub mod jump_game; // 55 ✓
pub mod merge_intervals; // 56 ✓
pub mod insert_interval; // 57 ✓
pub mod length_of_last_word; // 58

pub mod rotate_list; // 61 ✓
pub mod unique_paths; // 62 ✓
pub mod unique_paths_ii; // 63
pub mod minimum_path_sum; // 64 ✓

pub mod plus_one; // 66
pub mod add_binary; // 67 ✓
pub mod text_justification; // 68 ✓
pub mod sqrt_x; // 69 ✓
pub mod climbing_stairs; // 70
pub mod simplify_path; // 71
pub mod edit_distance; // 72 ✓
pub mod set_matrix_zeroes; // 73 ✓
pub mod search_a_2d_matrix; // 74 ✓
pub mod sort_colors; // 75 ✓
pub mod minimum_window_substring; // 76 ✓
pub mod combinations; // 77 ✓
pub mod subsets; // 78 ✓
pub mod word_search; // 79 ✓
pub mod remove_duplicates_from_sorted_array_ii; // 80

pub mod remove_duplicates_from_sorted_list_ii; // 82 ✓
pub mod remove_duplicates_from_sorted_list; // 83
pub mod largest_rectangle_in_histogram; // 84 ✓

pub mod partition_list; // 86 ✓

pub mod merge_sorted_array; // 88

pub mod subsets_ii; // 90 ✓
pub mod decode_ways; // 91 ✓
pub mod reverse_linked_list_ii; // 92

pub mod binary_tree_inorder_traversal; // 94 ✓

pub mod unique_binary_search_trees; // 96 ✓
pub mod interleaving_string; // 97 ✓
pub mod validate_binary_search_tree; // 98

pub mod same_tree; // 100 ✓
pub mod symmetric_tree; // 101 ✓
pub mod binary_tree_level_order_traversal; // 102 

pub mod binary_tree_zigzag_level_order_traversal; // 103
pub mod maximum_depth_of_binary_tree; // 104 ✓
pub mod construct_binary_tree_from_preorder_and_inorder_traversal; // 105

pub mod binary_tree_level_order_traversal_ii; // 107 ✓
pub mod convert_sorted_array_to_binary_search_tree; // 108 ✓

pub mod balanced_binary_tree; // 110 ✓
pub mod minimum_depth_of_binary_tree; // 111
pub mod path_sum; // 112
pub mod path_sum_ii; // 113 ✓
pub mod flatten_binary_tree_to_linked_list; // 114

pub mod pascals_triangle; // 118 ✓

pub mod best_time_to_buy_and_sell_stock; // 121
pub mod best_time_to_buy_and_sell_stock_ii; // 122

pub mod binary_tree_maximum_path_sum; // 124 ✓
pub mod valid_palindrome; // 125

pub mod word_ladder; // 127 ✓
pub mod longest_consecutive_sequence; // 128 ✓
pub mod sum_root_to_leaf_numbers; // 129 ✓
pub mod surrounded_regions; // 130 ✓
pub mod palindrome_partitioning; // 131 ✓
pub mod palindrome_partitioning_ii; // 132 ✓

// pub mod clone_graph; // 133 - This problem is unavailable in Rust
pub mod gas_station; // 134 ✓

pub mod single_number; // 136

pub mod word_break; // 139 ✓

// pub mod linked_list_cycle; // 141 - This problem is unavailable in Rust
// pub mod linked_list_cycle_ii; // 142 - This problem is unavailable in Rust
pub mod reorder_list; // 143 ✓

pub mod binary_tree_preorder_traversal; // 144 ✓
pub mod binary_tree_postorder_traversal; // 145 ✓
pub mod lru_cache; // 146 ✓
pub mod insertion_sort_list; // 147 ✓
pub mod sort_list; // 148

pub mod evaluate_reverse_polish_notation; // 150 ✓
pub mod reverse_words_in_a_string; // 151 ✓
pub mod maximum_product_subarray; // 152 ✓
pub mod find_minimum_in_rotated_sorted_array; // 153 ✓

pub mod min_stack; // 155

pub mod read_n_characters_given_read4; // 157 ✓

pub mod longest_substring_with_at_most_two_distinct_characters; // 159 ✓
// pub mod intersection_of_two_lists; // 160 - This problem is unavailable in Rust
pub mod one_edit_distance; // 161 ✓

pub mod missing_ranges; // 163
pub mod maximum_gap; // 164 ✓

pub mod two_sum_ii; // 167 ✓
pub mod excel_sheet_column_title; // 168 ✓
pub mod majority_element; // 169

pub mod excel_sheet_column_number; // 171 ✓

pub mod binary_search_tree_iterator; // 173 ✓

pub mod reverse_words_in_a_string_ii; // 186 ✓

pub mod best_time_to_buy_and_sell_stock_iv; // 188 ✓
pub mod rotate_array; // 189
pub mod reverse_bits; // 190
pub mod number_of_1_bits; // 191 ✓

// pub mod valid_phone_numbers; // 193 - This problem is unavailable in Rust

// pub mod tenth_line; // 195 - This problem is unavailable in Rust
// pub mod delete_duplicate_entries; // 196 - This problem is unavailable in Rust
// pub mod rising_temperature; // 197 - This problem is unavailable in Rust
pub mod house_robber; // 198
pub mod binary_tree_right_side_view; // 199
pub mod number_of_islands; // 200

pub mod happy_number; // 202
pub mod remove_linked_list_elements; // 203 ✓
pub mod count_primes; // 204 ✓
pub mod isomorphic_strings; // 205 ✓
pub mod reverse_linked_list; // 206 ✓
pub mod course_schedule; // 207 ✓
pub mod implement_trie; // 208 ✓
pub mod minimum_size_subarray_sum; // 209 ✓
pub mod course_schedule_ii; // 210 ✓
pub mod design_add_and_search_words_data_structure; // 211 ✓
pub mod word_search_ii; // 212 ✓
pub mod house_robber_ii; // 213 ✓

pub mod kth_largest_element_in_an_array; // 215 ✓
pub mod combination_sum_iii; // 216 ✓
pub mod contains_duplicate; // 217

pub mod contains_duplicate_ii; // 219

pub mod maximal_square; // 221 ✓
pub mod count_complete_tree_nodes; // 222 ✓

pub mod implement_stack_using_queues; // 225 ✓
pub mod invert_binary_tree; // 226 ✓

pub mod summary_ranges; // 228 ✓
pub mod majority_element_ii; // 229 ✓
pub mod kth_smallest_element_in_a_bst; // 230 ✓
pub mod power_of_two; // 231 ✓

pub mod palindrome_linked_list; // 234 ✓
pub mod lowest_common_ancestor_of_a_binary_search_tree; // 235 ✓
pub mod lowest_common_ancestor_of_a_binary_tree; // 236 ✓
// pub mod delete_node_in_a_linked_list; // 237 - This problem is unavailable in Rust
pub mod product_of_array_except_self; // 238
pub mod sliding_window_maximum; // 239
pub mod search_a_2d_matrix_ii; // 240 ✓

pub mod valid_anagram; // 242
pub mod shortest_word_distance; // 243 ✓
pub mod shortest_word_distance_ii; // 244 ✓
pub mod shortest_word_distance_iii; // 245 ✓

pub mod strobogrammatic_number; // 246

pub mod flatten_2d_vector; // 251 ✓
pub mod meeting_rooms; // 252
pub mod meeting_rooms_ii; // 253

pub mod verify_preorder_sequence_in_bst; // 255 ✓

pub mod binary_tree_paths; // 257
pub mod add_digits; // 258

pub mod graph_valid_tree; // 261

pub mod ugly_number; // 263 ✓

pub mod palindrome_permutation; // 266 ✓

pub mod missing_number; // 268
pub mod alien_dictionary; // 269 ✓
pub mod closest_binary_search_tree_value; // 270
pub mod encode_and_decode_strings; // 271 ✓

pub mod integer_to_english_words; // 273 ✓
pub mod h_index; // 274 ✓

pub mod first_bad_version; // 278 ✓

pub mod wiggle_sort; // 280

pub mod move_zeroes; // 283

pub mod find_the_duplicate_number; // 287 ✓

pub mod game_of_life; // 289 ✓
pub mod word_pattern; // 290 ✓

pub mod nim_game; // 292 ✓
pub mod flip_game; // 293 ✓

pub mod find_median_from_data_stream; // 295

pub mod serialize_and_deserialize_binary_tree; // 297

pub mod longest_increasing_subsequence; // 300

pub mod range_sum_query_immutable; // 303
pub mod range_sum_query_2d_immutable; // 304

pub mod best_time_to_buy_and_sell_stock_with_cooldown; // 309 ✓
pub mod minimum_height_trees; // 310

pub mod binary_tree_vertical_order_traversal; // 314 ✓

pub mod remove_duplicate_letters; // 316 ✓
pub mod shortest_distance_from_all_buildings; // 317 ✓

pub mod bulb_switcher; // 319

pub mod coin_change; // 322
pub mod number_of_connected_components_in_an_undirected_graph; // 323

pub mod power_of_three; // 326

pub mod odd_even_linked_list; // 328 ✓
pub mod longest_increasing_path_in_a_matrix; // 329 ✓

pub mod reconstruct_itinerary; // 332 ✓

pub mod increasing_triplet_subsequence; // 334 ✓

pub mod counting_bits; // 338 ✓
pub mod nested_list_weight_sum; // 339 ✓
pub mod longest_substring_with_at_most_k_distinct_characters; // 340 ✓
pub mod flatten_nested_list_iterator; // 341 ✓
pub mod power_of_four; // 342 ✓

pub mod reverse_string; // 344

pub mod reverse_vowels_of_a_string; // 345 ✓
pub mod moving_average_from_data_stream; // 346
pub mod top_k_frequent_elements; // 347

pub mod intersection_of_two_arrays; // 349 ✓
pub mod intersection_of_two_arrays_ii; // 350

pub mod valid_perfect_square; // 367 ✓

pub mod sum_of_two_integers; // 371 ✓

pub mod find_k_pairs_with_smallest_sums; // 373
pub mod guess_number_higher_or_lower; // 374

pub mod combination_sum_iv; // 377 ✓
pub mod kth_smallest_element_in_a_sorted_matrix; // 378

pub mod insert_delete_getrandom_o1; // 380 ✓

pub mod ransom_note; // 383
pub mod shuffle_an_array; // 384 ✓

pub mod first_unique_character_in_a_string; // 387 ✓

pub mod find_the_difference; // 389

pub mod is_subsequence; // 392

pub mod decode_string; // 394 ✓

pub mod evaluate_division; // 399 ✓

pub mod remove_k_digits; // 402 ✓
                         
pub mod sum_of_left_leaves; // 404 ✓
pub mod convert_a_number_to_hexadecimal; // 405

pub mod valid_word_abbreviation; // 408 ✓
pub mod longest_palindrome; // 409 ✓
pub mod split_array_largest_sum; // 410 ✓

pub mod fizz_buzz; // 412

pub mod add_strings; // 415 ✓
pub mod partition_equal_subset_sum; // 416 ✓

pub mod valid_word_square; // 422 ✓

pub mod longest_repeating_character_replacement; // 424 ✓

// pub mod n_ary_tree_level_order_traversal; // 429 - This problem is unavailable in Rust

pub mod minimum_genetic_mutation; // 433 ✓
pub mod number_of_segments_in_a_string; // 434 ✓
pub mod non_overlapping_intervals; // 435 ✓

pub mod path_sum_iii; // 437 ✓
pub mod find_all_anagrams_in_a_string; // 438 ✓

pub mod arranging_coins; // 441 ✓

pub mod string_compression; // 443 ✓

pub mod find_all_numbers_disappeared_in_an_array; // 448 ✓

pub mod delete_node_in_a_bst; // 450 ✓
pub mod sort_characters_by_frequency; // 451 ✓
pub mod minimum_number_of_arrows_to_burst_balloons; // 452 ✓

pub mod assign_cookies; // 455 ✓

pub mod repeated_substring_pattern; // 459 ✓

pub mod hamming_distance; // 461 ✓

pub mod island_perimeter; // 463

pub mod validate_ip_address; // 468 ✓

pub mod ones_and_zeroes; // 474

pub mod number_complement; // 476 ✓

pub mod sliding_window_median; // 480 ✓

pub mod license_key_formatting; // 482 ✓

pub mod max_consecutive_ones; // 485 ✓

pub mod target_sum; // 494 ✓

pub mod next_greater_element_i; // 496

pub mod diagonal_traverse; // 498 ✓

pub mod find_mode_in_binary_search_tree; // 501 ✓

pub mod next_greater_element_ii; // 503 ✓

pub mod relative_ranks; // 506 ✓

pub mod fibonacci_number; // 509

pub mod find_largest_value_in_each_tree_row; // 515
pub mod longest_palindromic_subsequence; // 516 ✓

pub mod coin_change_ii; // 518 ✓

pub mod contiguous_array; // 525 ✓

pub mod minimum_absolute_difference_in_bst; // 530
pub mod lonely_pixel_i; // 531 ✓

pub mod complex_number_multiplication; // 537 ✓

pub mod single_element_in_a_sorted_array; // 540 ✓
pub mod reverse_string_ii; // 541 ✓
pub mod zero_one_matrix; // 542
pub mod diameter_of_binary_tree; // 543

pub mod number_of_provinces; // 547 ✓

pub mod brick_wall; // 554

pub mod reverse_words_in_a_string_iii; // 557

pub mod subarray_sum_equals_k; // 560 ✓

pub mod permutation_in_string; // 567 ✓

pub mod subtree_of_another_tree; // 572 ✓

// pub mod classes_more_than_5_students; // 596 - This problem is unavailable in Rust

pub mod can_place_flowers; // 605 ✓
pub mod construct_string_from_binary_tree; // 606 ✓

pub mod merge_two_binary_trees; // 617 ✓

pub mod task_scheduler; // 621 ✓

pub mod maximum_distance_in_arrays; // 624 ✓

pub mod average_of_levels_in_binary_tree; // 637

pub mod maximum_average_subarray_i; // 643

pub mod palindromic_substrings; // 647 ✓
pub mod replace_words; // 648 ✓
pub mod dota2_senate; // 649 ✓

pub mod find_k_closest_elements; // 658

pub mod valid_palindrome_ii; // 680

pub mod baseball_game; // 682 ✓

pub mod redundant_connection; // 684 ✓

pub mod top_k_frequent_words; // 692

pub mod max_area_of_island; // 695

pub mod search_in_a_binary_search_tree; // 700 ✓
pub mod insert_into_a_binary_search_tree; // 701

pub mod kth_largest_element_in_a_stream; // 703
pub mod binary_search; // 704
pub mod design_hashset; // 705 ✓
pub mod design_hashmap; // 706 ✓
pub mod design_linked_list; // 707 ✓

pub mod best_time_to_buy_and_sell_stock_with_transaction_fee; // 714 ✓

pub mod accounts_merge; // 721 ✓

pub mod find_pivot_index; // 724 ✓

pub mod sentence_similarity; // 734 ✓
pub mod asteroid_collision; // 735 ✓

pub mod daily_temperatures; // 739 ✓
pub mod delete_and_earn; // 740 ✓

pub mod network_delay_time; // 743 ✓

pub mod min_cost_climbing_stairs; // 746

pub mod open_the_lock; // 752 ✓

pub mod bold_words_in_string; // 758 ✓
pub mod employee_free_time; // 759 ✓
pub mod find_anagram_mappings; // 760 ✓

pub mod jewels_and_stones; // 771

pub mod minimum_distance_between_bst_nodes; // 783

pub mod cheapest_flight_within_k_stops; // 787 ✓

pub mod custom_sort_string; // 791 ✓

pub mod rotate_string; // 796 ✓
pub mod all_paths_from_source_to_target; // 797

pub mod linked_list_components; // 817

pub mod similar_string_groups; // 839

pub mod keys_and_rooms; // 841 ✓

pub mod backspace_string_compare; // 844 ✓

pub mod maximum_distance_to_closest_person; // 849 ✓

pub mod peak_index_in_a_mountain_array; // 852 ✓
pub mod car_fleet; // 853 ✓

pub mod all_nodes_distance_k_in_binary_tree; // 863

pub mod smallest_subtree_with_all_the_deepest_nodes; // 865 ✓

pub mod leaf_similar_trees; // 872 ✓

pub mod koko_eating_bananas; // 875 ✓
pub mod middle_of_the_linked_list; // 876

pub mod online_stock_span; // 901

pub mod snakes_and_ladders; // 909 ✓

pub mod sort_an_array; // 912 ✓

pub mod word_subsets; // 916 ✓
pub mod reverse_only_letters; // 917
pub mod maximum_sum_circular_subarray; // 918 ✓

pub mod unique_email_addresses; // 929 ✓
pub mod binary_subarrays_with_sum; // 930 ✓
pub mod minimum_falling_path_sum; // 931 ✓

pub mod number_of_recent_calls; // 933 ✓

pub mod range_sum_of_bst; // 938

pub mod valid_mountain_array; // 941 ✓

pub mod validate_stack_sequences; // 946 ✓

pub mod verifying_an_alien_dictionary; // 953 ✓

pub mod numbers_with_same_consecutive_differences; // 967 ✓

pub mod k_closest_points_to_origin; // 973

pub mod squares_of_a_sorted_array; // 977
pub mod longest_turbulent_subarray; // 978 ✓

pub mod time_based_key_value_store; // 981

pub mod minimum_cost_for_tickets; // 983 ✓

pub mod interval_list_intersections; // 986 ✓

pub mod cousins_in_binary_tree; // 993 ✓
pub mod rotting_oranges; // 994

pub mod next_greater_node_in_linked_list; // 1019 ✓

pub mod maximum_difference_between_node_and_ancestor; // 1026

pub mod last_stone_weight; // 1046
pub mod remove_all_adjacent_duplicates_in_string; // 1047

pub mod last_stone_weight_ii; // 1049 ✓

pub mod height_checker; // 1051

pub mod shortest_way_to_form_string; // 1055 ✓
pub mod confusing_number; // 1056 ✓

pub mod number_of_valid_subarrays; // 1063 ✓

pub mod index_pairs_of_a_string; // 1065 ✓

pub mod greatest_common_divisor_of_strings; // 1071 ✓

pub mod shortest_path_in_binary_matrix; // 1091

pub mod car_pooling; // 1094 ✓

pub mod find_k_length_substrings_with_no_repeated_characters; // 1100 ✓
pub mod the_earliest_moment_when_everyone_becomes_friends; // 1101

pub mod remove_vowels_from_a_string; // 1119 ✓

pub mod lowest_common_ancestor_of_deepest_leaves; // 1123 ✓

pub mod shortest_path_with_alternating_colors; // 1129

pub mod largest_unique_number; // 1133 ✓
pub mod armstrong_number; // 1134 ✓

pub mod parallel_courses; // 1136 ✓
pub mod nth_tribonacci_number; // 1137 ✓

pub mod longest_common_subsequence; // 1143

pub mod check_if_a_number_is_majority_element; // 1150 ✓

pub mod maximum_level_sum_of_a_binary_tree; // 1161 ✓

pub mod single_row_keyboard; // 1165 ✓

pub mod minimum_cost_to_connect_sticks; // 1167

pub mod maximum_number_of_balloons; // 1189 ✓

pub mod how_many_apples_can_you_put_into_the_basket; // 1196 ✓

pub mod find_smallest_common_element_in_all_rows; // 1198 ✓

pub mod minimum_absolute_difference; // 1200 ✓

pub mod smallest_string_with_swaps; // 1202

pub mod unique_number_of_occurrences; // 1207 ✓
pub mod get_equal_substring_within_budget; // 1208 ✓

pub mod missing_number_in_arithmetic_progression; // 1228 ✓

pub mod toss_strange_coins; // 1230
pub mod divide_chocolate; // 1231 ✓
pub mod check_if_it_is_a_straight_line; // 1232 ✓

pub mod minimum_time_visiting_all_points; // 1266 ✓

pub mod search_suggestions_system; // 1268 ✓

pub mod group_the_people_given_the_group_size_they_belong_to; // 1282 ✓
pub mod find_the_smallest_divisor_given_a_threshold; // 1283

pub mod convert_binary_number_in_a_linked_list_to_integer; // 1290 ✓

pub mod shortest_path_in_a_grid_with_obstacle_elimination; // 1293

pub mod replace_elements_with_greater_element_on_right_side; // 1299 ✓

pub mod deepest_leaves_sum; // 1302

pub mod jump_game_iii; // 1306 ✓

pub mod minimum_flips_to_make_a_or_b_equal_to_c; // 1318 ✓

pub mod maximum_69_number; // 1323 ✓

pub mod minimum_difficulty_of_a_job_schedule; // 1335

pub mod the_k_weakest_rows_in_a_matrix; // 1337
pub mod reduce_array_size_to_the_half; // 1338 ✓

pub mod number_of_steps_to_reduce_a_number_to_zero; // 1342
pub mod number_of_subarrays_of_size_k_and_average_greater_than_or_equal_to_threshold; // 1343 ✓

pub mod count_negative_numbers_in_a_sorted_matrix; // 1351 ✓

pub mod longest_zigzag_path_in_a_binary_tree; // 1372 ✓

pub mod time_needed_to_inform_all_employees; // 1376 ✓

pub mod find_lucky_integer_in_an_array; // 1394 ✓

pub mod design_underground_system; // 1396

pub mod longest_happy_string; // 1405 ✓

pub mod maximum_score_after_splitting_a_string; // 1422 ✓

pub mod perform_string_shifts; // 1427 ✓

pub mod kids_with_the_greatest_number_of_candies; // 1431 ✓

pub mod destination_city; // 1436 ✓

pub mod longest_continuous_subarray_with_absolute_diff; // 1438 ✓

pub mod count_good_nodes_in_binary_tree; // 1448 ✓

pub mod maximum_number_of_vowels_in_a_substring_of_given_length; // 1456 ✓

pub mod course_schedule_iv; // 1462 ✓

pub mod reorder_routes_to_make_all_paths_lead_to_the_city_zero; // 1466 ✓

pub mod design_browser_history; // 1472 ✓

pub mod final_prices_with_a_special_discount; // 1475 ✓

pub mod running_sum_of_1d_array; // 1480

pub mod average_salary_excluding_the_minimum_and_maximum_salary; // 1491

pub mod longest_subarray_of_ones_after_deleting_one_element; // 1493 ✓

pub mod path_crossing; // 1496

pub mod can_make_arithmetic_progression_from_sequence; // 1502

pub mod minimum_difference_in_three_moves; // 1509 ✓

pub mod number_of_good_pairs; // 1512 ✓

pub mod path_with_maximum_probability; // 1514 ✓

pub mod shuffle_string; // 1528 ✓

pub mod make_the_string_great; // 1544

pub mod minimum_number_of_vertices_to_reach_all_nodes; // 1557

pub mod dot_product_of_two_sparse_vectors; // 1570 ✓

pub mod min_cost_to_connect_all_points; // 1584

pub mod max_number_of_unique_substrings; // 1593 ✓

pub mod design_parking_system; // 1603 ✓
pub mod alert_using_same_key_card; // 1604 ✓

pub mod path_with_minimum_effort; // 1631 ✓

pub mod number_of_ways_to_form_a_target_string_given_a_dictionary; // 1639 ✓

pub mod furthest_building_you_can_reach; // 1642

pub mod determine_if_two_strings_are_close; // 1657 ✓

pub mod richest_customer_wealth; // 1672
pub mod find_the_most_competitive_subsequence; // 1673 ✓

pub mod max_number_of_k_sum_pairs; // 1679 ✓

pub mod sum_of_absolute_differences_in_a_sorted_array; // 1685 ✓

pub mod maximum_erasure_value; // 1695 ✓

pub mod number_of_students_unable_to_eat_lunch; // 1700 ✓

pub mod maximum_units_on_a_truck; // 1710 ✓

pub mod find_the_highest_altitude; // 1732 ✓

pub mod sum_of_unique_elements; // 1748 ✓

pub mod longest_nice_substring; // 1763 ✓

pub mod merge_strings_alternately; // 1768 ✓

pub mod maximum_score_from_performing_multiplication_operations; // 1770 ✓

pub mod find_the_center_of_star_graph; // 1791 ✓

pub mod sign_of_the_product_of_an_array; // 1822 ✓

pub mod maximum_population_year; // 1854
pub mod maximum_distance_between_a_pair_of_values; // 1855 ✓

pub mod maximum_subarray_min_product; // 1856 ✓

pub mod minimum_speed_to_arrive_on_time; // 1870 ✓

pub mod redistribute_characters_to_make_all_strings_equal; // 1897

pub mod build_array_from_permutation; // 1920 ✓

pub mod nearest_exit_from_entrance_in_maze; // 1926

pub mod concatenation_of_array; // 1929 ✓

pub mod number_of_visible_people_in_a_queue; // 1944 ✓

pub mod delete_characters_to_make_fancy_string; // 1957 ✓

pub mod remove_stones_to_minimize_the_total; // 1962

pub mod array_with_elements_not_equal_to_average_of_neighbors; // 1968

pub mod find_if_path_exists_in_graph; // 1971

pub mod minimum_difference_between_highest_and_lowest_of_k_scores; // 1984

pub mod final_value_of_variable_after_performing_operations; // 2011

pub mod sort_linked_list_already_sorted_using_absolute_values; // 2046 ✓

pub mod reverse_nodes_in_even_length_groups; // 2074 ✓

pub mod k_radius_subarray_averages; // 2090

pub mod delete_the_middle_node_of_a_linked_list; // 2095 ✓

pub mod detonate_the_maximum_bombs; // 2101 ✓

pub mod destroying_asteroids; // 2126

pub mod maximum_twin_sum_of_a_linked_list; // 2130 ✓

pub mod solving_questions_with_brainpower; // 2140

pub mod minimum_operations_to_halve_array_sum; // 2208

pub mod find_the_difference_of_two_arrays; // 2215 ✓

pub mod find_players_with_zero_or_one_losses; // 2225 ✓

pub mod successful_pairs_of_spells_and_potions; // 2300 ✓

pub mod smallest_number_in_infinite_set; // 2336

pub mod query_kth_smallest_trimmed_number; // 2343 ✓

pub mod equal_row_and_column_pairs; // 2352 ✓

pub mod minimum_replacements_to_sort_the_array; // 2366 ✓

pub mod reachable_nodes_with_restrictions; // 2368

pub mod longest_subsequence_with_limited_sum; // 2389 ✓
pub mod removing_stars_from_a_string; // 2390 ✓

pub mod reverse_odd_levels_of_binary_tree; // 2415 ✓

pub mod robot_to_print_the_lexicographically_smallest_string; // 2434 ✓

pub mod total_cost_to_hire_k_workers; // 2462 ✓

pub mod circular_sentence; // 2490 ✓

pub mod longest_square_streak_in_an_array; // 2501 ✓

pub mod minimum_common_value; // 2540 ✓

pub mod maximum_subsequence_score; // 2542 ✓

pub mod count_the_number_of_fair_pairs; // 2563 ✓

pub mod kth_largest_sum_in_a_binary_tree; // 2583 ✓

pub mod find_score_of_an_array_after_marking_all_elements; // 2593 ✓

pub mod remove_trailing_zeros_from_a_string; // 2710 ✓
pub mod difference_of_number_of_distinct_values_on_diagonals; // 2711 ✓
pub mod minimum_cost_to_make_all_characters_equal; // 2712

pub mod neither_minimum_nor_maximum; // 2733

pub mod total_distance_traveled; // 2739 ✓

pub mod find_the_maximum_achievable_number; // 2769 ✓

pub mod sort_vowels_in_a_string; // 2785 ✓

pub mod minimum_number_of_changes_to_make_binary_string_beautiful; // 2914 ✓

pub mod longest_subarray_with_at_most_k_frequency; // 2958 ✓

pub mod count_elements_with_maximum_frequency; // 3005 ✓

pub mod find_if_array_can_be_sorted; // 3011 ✓

pub mod score_of_a_string; // 3110 ✓

pub mod minimum_array_end; // 3133 ✓

pub mod string_compression_iii; // 3163 ✓

pub mod bitwise_or_adjacent_elements; // 3173 ✓
