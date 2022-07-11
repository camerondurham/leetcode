cmake_minimum_required(VERSION 3.22)
project(leetcode)

set(CMAKE_CXX_STANDARD 14)

include_directories(2022/valid_palindrome)

add_executable(leetcode
        2022/valid_palindrome/Solution.cpp
        2022/valid_palindrome/Solution.h
        problems/0014_longest_common_prefix.cpp
        problems/0017_letter_combinations_of_phone_number.cpp
        problems/0043_multiply_strings.cpp
        problems/0057_insert_interval.cpp
        problems/0108_convert_sorted_list_to_binary_tree.cpp
        problems/0237_delete_node_from_linked_list.cpp
        problems/0387_first_unique_character_in_string.cpp
        problems/0532_k_diff_pairs.cpp
        problems/0701_insert_into_a_binary_search_tree.cpp
        problems/0933_number_of_recent_calls.cpp
        problems/0959_regions_cut_by_slashes.cpp
        problems/1288_removed_covered_interval.cpp
        problems/1486_xor_op_in_array.cpp
        problems/1507_reformat_date.cpp
        problems/car_pooling.cpp
        problems/GetAllElements.cpp)
