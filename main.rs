fn is_palindrome(s: &str) -> bool {
  let s = s.to_lowercase();
  let s: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
  s == s.chars().rev().collect::<String>()
}

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
  arr.iter().position(|&x| x == target)
}

fn shortest_word(sentence: &str) -> &str {
  sentence.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

fn is_prime(num: u64) -> bool {
  if num <= 1 {
      return false;
  }
  for i in 2..=num / 2 {
      if num % i == 0 {
          return false;
      }
  }
  true
}

fn median(arr: &[i32]) -> f64 {
  let len = arr.len();
  if len % 2 == 0 {
      let mid = len / 2;
      (arr[mid - 1] + arr[mid]) as f64 / 2.0
  } else {
      arr[len / 2] as f64
  }
}

fn longest_common_prefix(strings: &[&str]) -> String {
  if strings.is_empty() {
      return String::new();
  }
  let mut prefix = strings[0].to_string();
  for word in strings.iter().skip(1) {
      while !word.starts_with(&prefix) {
          prefix.pop();
      }
  }
  prefix
}

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
  if k > 0 && k <= arr.len() {
      let mut sorted_arr = arr.to_vec();
      sorted_arr.sort();
      Some(sorted_arr[k - 1])
  } else {
      None
  }
}

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
  val: i32,
  left: Option<Box<TreeNode>>,
  right: Option<Box<TreeNode>>,
}

// fn max_depth(root: Option<&TreeNode>) -> i32 {
//   match root {
//       Some(node) => 1 + std::cmp::max(max_depth(node.left.as_ref()), max_depth(node.right.as_ref())),
//       None => 0,
//   }
// }

fn reverse_string(s: &str) -> String {
  s.chars().rev().collect()
}

fn is_prime_rust(num: u64) -> bool {
  if num <= 1 {
      return false;
  }
  for i in 2..=num / 2 {
      if num % i == 0 {
          return false;
      }
  }
  true
}

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
  let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
  let (mut i, mut j) = (0, 0);
  while i < arr1.len() && j < arr2.len() {
      if arr1[i] < arr2[j] {
          merged.push(arr1[i]);
          i += 1;
      } else {
          merged.push(arr2[j]);
          j += 1;
      }
  }
  merged.extend_from_slice(&arr1[i..]);
  merged.extend_from_slice(&arr2[j..]);
  merged
}

fn max_subarray_sum(arr: &[i32]) -> i32 {
  let mut max_sum = 0;
  let mut current_sum = 0;
  for &num in arr {
      current_sum = current_sum.max(0) + num;
      max_sum = max_sum.max(current_sum);
  }
  max_sum
}

fn main() {
  // Testing is_palindrome
  println!("Is 'A man, a plan, a canal, Panama!' a palindrome? {}", is_palindrome("A man, a plan, a canal, Panama!"));

  // Testing first_occurrence
  let arr = [1, 2, 3, 4, 5, 6, 7];
  let target = 5;
  println!("First occurrence of {} in {:?} is at index {:?}", target, arr, first_occurrence(&arr, target));

  // Testing shortest_word
  let sentence = "The quick brown fox jumps over the lazy dog";
  println!("Shortest word in '{}' is '{}'", sentence, shortest_word(sentence));

  // Testing is_prime
  let num = 17;
  println!("Is {} prime? {}", num, is_prime(num));

  // Testing median
  let arr = [1, 2, 3, 4, 5];
  println!("Median of {:?} is {}", arr, median(&arr));

  // Testing longest_common_prefix
  let strings = ["flower", "flow", "flight"];
  println!("Longest common prefix of {:?} is '{}'", strings, longest_common_prefix(&strings));

  // Testing kth_smallest
  let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
  let k = 3;
  println!("{}th smallest element in {:?} is {:?}", k, arr, kth_smallest(&arr, k));

  // Testing max_depth
  let root = Some(Box::new(TreeNode {
      val: 1,
      left: Some(Box::new(TreeNode {
          val: 2,
          left: None,
          right: None,
      })),
      right: Some(Box::new(TreeNode {
          val: 3,
          left: Some(Box::new(TreeNode {
              val: 4,
              left: None,
              right: None,
          })),
          right: Some(Box::new(TreeNode {
              val: 5,
              left: None,
              right: None,
          })),
      })),
  }));
  // println!("Maximum depth of the binary tree is {}", max_depth(root));

  // Testing reverse_string
  let s = "hello";
  println!("Reverse of '{}' is '{}'", s, reverse_string(s));

  // Testing is_prime_rust
  let num = 17;
  println!("Is {} prime? {}", num, is_prime_rust(num));

  // Testing merge_sorted_arrays
  let arr1 = [1, 3, 5, 7, 9];
  let arr2 = [2, 4, 6, 8, 10];
  println!("Merged sorted arrays: {:?}", merge_sorted_arrays(&arr1, &arr2));

  // Testing max_subarray_sum
  let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
  println!("Maximum subarray sum of {:?} is {}", arr, max_subarray_sum(&arr));
}
