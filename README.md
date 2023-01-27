# Fung Sort Benchmark

This program is a simple Rust program that aims to benchmark the performance of the fung sort sorting algorithm. It generates a random vector of 100 64-bit integers and sorts it using 4 different sorting algorithms: stable sort, unstable sort, fung sort, and select sort. The program measures the time taken for each sorting algorithm to sort the vector, and stores it in an array for each algorithm. The program then runs this process for 10,000 iterations, and calculates the mean time taken for each sorting algorithm to sort the vector. 

# Stable Sort (Rust STD sort())
The current algorithm is an adaptive, iterative merge sort inspired by timsort. It is designed to be very fast in cases where the slice is nearly sorted, or consists of two or more sorted sequences concatenated one after another.

Also, it allocates temporary storage half the size of self, but for short slices a non-allocating insertion sort is used instead.

# Unstable Sort (Rust STD sort_unstable())
The current algorithm is based on pattern-defeating quicksort by Orson Peters, which combines the fast average case of randomized quicksort with the fast worst case of heapsort, while achieving linear time on slices with certain patterns. It uses some randomization to avoid degenerate cases, but with a fixed seed to always provide deterministic behavior.

It is typically faster than stable sorting, except in a few special cases, e.g., when the slice consists of several concatenated sorted sequences.

# Fung Sort
There is nothing good about this algorithm. It is slow – the algorithm obviously runs in Θ(n2) time, whether worst-case, average-case or best-case. It unnecessarily compares all pairs of positions, twice. There seems to be no intuition behind it, and its correctness is not entirely obvious. You certainly do not want to use it as a first example to introduce students to sorting algorithms. It is not stable, does not work well for external sorting, cannot sort inputs arriving online, and does not benefit from partially sorted inputs. Its only appeal may be its simplicity, in terms of lines of code and the “symmetry” of the two loops.

# Select Sort
Selection sort is a simple sorting algorithm that repeatedly selects the smallest (or largest, depending on the sorting order) element from the unsorted portion of the list and moves it to the sorted portion of the list. The algorithm repeatedly finds the minimum element from the unsorted list and swaps it with the leftmost unsorted element, moving the boundary of the sorted list one element to the right.

# Results
| Algorithm     | Speed (Average Nano Seconds) |
| ------------- | ------------- |
| Stable sort  | <p align="center">31109</p>  |
| Unstable sort  | <p align="center">44917</p>  |
| Fung sort  | <p align="center">1125436</p>  |
| Select sort | <p align="center">628020</p> |
