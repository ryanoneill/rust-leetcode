#![allow(dead_code)]
#![allow(unused_imports)]

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

pub mod two_sum; // 1 ✓
pub mod add_two_numbers; // 2
pub mod length_of_longest_substring; // 3 ✓
pub mod median_of_two_sorted_arrays; // 4
pub mod longest_palindromic_substring; // 5
pub mod zigzag_conversion; // 6 ✓
pub mod reverse_integer; // 7 ✓
pub mod string_to_integer; // 8 ✓
pub mod palindrome_number; // 9 ✓
pub mod regular_expression_matching; // 10
pub mod container_with_most_water; // 11 ✓

pub mod longest_common_prefix; // 14

pub mod remove_nth_node_from_end_of_list; // 19 ✓
pub mod valid_parentheses; // 20
pub mod merge_two_sorted_lists; // 21

pub mod swap_nodes_in_pairs; // 24

pub mod remove_duplicates_from_sorted_array; // 26
pub mod remove_element; // 27
pub mod find_the_index_of_the_first_occurrence_in_a_string; // 28

pub mod range_sum_query_immutable; // 30

pub mod search_insert_position; // 35
pub mod valid_sudoku; // 36 ✓

pub mod jump_game_ii; // 45 ✓

pub mod rotate_image; // 48

pub mod jump_game; // 55 ✓

pub mod length_of_last_word; // 58

pub mod plus_one; // 66

pub mod climbing_stairs; // 70
pub mod simplify_path; // 71

pub mod sort_colors; // 75

pub mod remove_duplicates_from_sorted_array_ii; // 80

pub mod remove_duplicates_from_sorted_list_ii; // 82
pub mod remove_duplicates_from_sorted_list; // 83

pub mod merge_sorted_array; // 88

pub mod reverse_linked_list_ii; // 92

pub mod validate_binary_search_tree; // 98

pub mod binary_tree_zigzag_level_order_traversal; // 103
pub mod maximum_depth_of_binary_tree; // 104

pub mod minimum_depth_of_binary_tree; // 111
pub mod path_sum; // 112

pub mod best_time_to_buy_and_sell_stock; // 121
pub mod best_time_to_buy_and_sell_stock_ii; // 122

pub mod valid_palindrome; // 125

// pub mod clone_graph; // 133 - This problem is unavailable in Rust

pub mod single_number; // 136

pub mod reverse_words_in_a_string; // 151 ✓

pub mod min_stack; // 155

pub mod missing_ranges; // 163

pub mod majority_element; // 169

pub mod rotate_array; // 189

pub mod house_robber; // 198
pub mod binary_tree_right_side_view; // 199
pub mod number_of_islands; // 200

pub mod happy_number; // 202

pub mod reverse_linked_list; // 206
pub mod course_schedule; // 207 ✓

pub mod course_schedule_ii; // 210 ✓

pub mod contains_duplicate; // 217

pub mod contains_duplicate_ii; // 219

pub mod palindrome_linked_list; // 234 ✓

pub mod lowest_common_ancestor_of_a_binary_tree; // 236

pub mod product_of_array_except_self; // 238
pub mod sliding_window_maximum; // 239
pub mod valid_anagram; // 240

pub mod meeting_rooms; // 252
pub mod meeting_rooms_ii; // 253

pub mod add_digits; // 258

pub mod graph_valid_tree; // 261

pub mod alien_dictionary; // 269 ✓
pub mod closest_binary_search_tree_value; // 270

pub mod h_index; // 274 ✓

pub mod move_zeroes; // 283

pub mod find_median_from_data_stream; // 295

pub mod serialize_and_deserialize_binary_tree; // 297

pub mod longest_increasing_subsequence; // 300

pub mod minimum_height_trees; // 310

pub mod shortest_distance_from_all_buildings; // 317

pub mod bulb_switcher; // 319

pub mod coin_change; // 322
pub mod number_of_connected_components_in_an_undirected_graph; // 323

pub mod reconstruct_itinerary; // 332

pub mod increasing_triplet_subsequence; // 334 ✓

pub mod reverse_string; // 344

pub mod reverse_vowels_of_a_string; // 345 ✓
pub mod moving_average_from_data_stream; // 346
pub mod top_k_frequent_elements; // 347

pub mod intersection_of_two_arrays_ii; // 350

pub mod kth_smallest_element_in_a_sorted_matrix; // 378

pub mod insert_delete_getrandom_o1; // 380 ✓

pub mod ransom_note; // 383

pub mod first_unique_character_in_a_string;

pub mod is_subsequence; // 392

pub mod fizz_buzz; // 412

// pub mod n_ary_tree_level_order_traversal; // 429 - This problem is unavailable in Rust

pub mod string_compression; // 443 ✓

pub mod next_greater_element_i; // 496

pub mod fibonacci_number; // 509

pub mod find_largest_value_in_each_tree_row; // 515

pub mod minimum_absolute_difference_in_bst; // 530

pub mod zero_one_matrix; // 542
pub mod diameter_of_binary_tree; // 543

pub mod number_of_provinces; // 547

pub mod reverse_words_in_a_string_iii; // 557

pub mod can_place_flowers; // 605 ✓

pub mod average_of_levels_in_binary_tree; // 637

pub mod find_k_closest_elements; // 658

pub mod top_k_frequent_words; // 692

pub mod max_area_of_island; // 695

pub mod insert_into_a_binary_search_tree; // 701

pub mod kth_largest_element_in_a_stream; // 703
pub mod binary_search; // 704

pub mod daily_temperatures; // 739

pub mod network_delay_time; // 743

pub mod min_cost_climbing_stairs; // 746

pub mod jewels_and_stones; // 771

pub mod minimum_distance_between_bst_nodes; // 783

pub mod all_paths_from_source_to_target; // 797

pub mod similar_string_groups; // 839

pub mod keys_and_rooms; // 841

pub mod all_nodes_distance_k_in_binary_tree; // 863

pub mod middle_of_the_linked_list; // 876

pub mod online_stock_span; // 901

pub mod reverse_only_letters; // 917

pub mod number_of_recent_calls; // 933

pub mod range_sum_of_bst; // 938

pub mod k_closest_points_to_origin; // 973

pub mod rotting_oranges; // 994

pub mod maximum_difference_between_node_and_ancestor; // 1026

pub mod last_stone_weight; // 1046
pub mod remove_all_adjacent_duplicates_in_string; // 1047

pub mod height_checker; // 1051

pub mod greatest_common_divisor_of_strings; // 1071 ✓

pub mod shortest_path_in_binary_matrix; // 1091

pub mod the_earliest_moment_when_everyone_becomes_friends; // 1101

pub mod remove_vowels_from_a_string; // 1119 ✓

pub mod shortest_path_with_alternating_colors; // 1129

pub mod parallel_courses; // 1136 ✓

pub mod longest_common_subsequence; // 1143

pub mod minimum_cost_to_connect_sticks; // 1167

pub mod smallest_string_with_swaps; // 1202

pub mod shortest_path_in_a_grid_with_obstacle_elimination; // 1293

pub mod deepest_leaves_sum; // 1302

pub mod the_k_weakest_rows_in_a_matrix; // 1337

pub mod number_of_steps_to_reduce_a_number_to_zero; // 1342

pub mod kids_with_the_greatest_number_of_candies; // 1431 ✓

pub mod count_good_nodes_in_binary_tree; // 1448

pub mod maximum_number_of_vowels_in_a_substring_of_given_length; // 1456 ✓

pub mod running_sum_of_1d_array; // 1480

pub mod average_salary_excluding_the_minimum_and_maximum_salary; // 1491

pub mod longest_subarray_of_ones_after_deleting_one_element; // 1493 ✓

pub mod make_the_string_great; // 1544

pub mod minimum_number_of_vertices_to_reach_all_nodes; // 1557

pub mod min_cost_to_connect_all_points; // 1584

pub mod furthest_building_you_can_reach; // 1642

pub mod richest_customer_wealth; // 1672

pub mod max_number_of_k_sum_pairs; // 1679 ✓

pub mod find_the_highest_altitude; // 1732 ✓

pub mod merge_strings_alternately; // 1768 ✓

pub mod maximum_population_year; // 1854

pub mod nearest_exit_from_entrance_in_maze; // 1926

pub mod remove_stones_to_minimize_the_total; // 1962

pub mod array_with_elements_not_equal_to_average_of_neighbors; // 1968

pub mod find_if_path_exists_in_graph; // 1971

pub mod delete_the_middle_node_of_a_linked_list; // 2095

pub mod destroying_asteroids; // 2126

pub mod solving_questions_with_brainpower; // 2140

pub mod minimum_operations_to_halve_array_sum; // 2208

pub mod smallest_number_in_infinite_set; // 2336

pub mod reachable_nodes_with_restrictions; // 2368

pub mod removing_stars_from_a_string; // 2390
