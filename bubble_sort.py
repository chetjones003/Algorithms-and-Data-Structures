#!/usr/bin/env python3

from typing import List


def bubble_sort(arr: List[int]):
    swapped: bool = False

    for i in range(len(arr) - 1):
        for j in range(len(arr) - 1 - i):
            if arr[j] > arr[j + 1]:
                tmp = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = tmp
                swapped = True

    if swapped:
        return arr


def test_sorted_array():
    arr = [1, 3, 4, 7, 2]
    assert bubble_sort(arr) == [1, 2, 3, 4, 7]
