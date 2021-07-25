#![allow(dead_code)]
mod check_permutation;
mod is_unique;

/// There are three types of edits that can be performed on strings:
/// insert a character, remove a character, or replace a character
/// Given two strings, write a function to check if they are one edit (or zero edits away)
/// Both [one_edit_away] and [one_edit_replace] takes O(n) time,
/// where n is the length of the shorter string
mod one_away;
mod palindrome_permutation;

/// Given an image represented by an NxN matrix, where each pixel in the image is 4 bytes,
/// write a method to rotate the image by 90 degrees. Can you do this in place?
///
/// Hints
///     - Try thinking about it layer by layer. Can you rotate a specific layer?
///     - Rotating a spcific layer would just mean swapping the values in four arrays.
///        If you were asked to wap the values in two arrays, could you do this?
///        Can you then extend it to four arrays?F
mod rotate_matrix;

/// Implement a method to perform basic string compression using the counts
/// of repeted characters. For example, the string aabcccccaaa would become a2b1c5a3.
/// If the "compressed" string would not become smaller then the original string,
/// your method should retuyrn the original string. You can assume the string has
/// only uppercase and lowercase letters (a-z).
///
/// Hints
///     - Do the easy things first. Compress the thring, then compare the length.
///     - Be careful that you aren't repeatedly concatenating strings toghether. This can be very inefficient.
mod string_compression;
mod string_rotation;
mod urlify;

/// Write an algorithm such that if an element in an MxN matrix is 0, its entire row and column are set to 0F
mod zero_matrix;
