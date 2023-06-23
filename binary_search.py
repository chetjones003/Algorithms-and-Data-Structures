#!/usr/bin/env python3

from typing import List, Optional


def binary_search(arr: List[int], target: int) -> Optional[int]:
    low: int = 0
    high: int = len(arr) - 1

    while low <= high:
        midpoint: int = low + ((high - low) // 2)
        value: int = arr[midpoint]

        if value == target:
            return midpoint
        elif value > target:
            high = midpoint - 1
        elif value < target:
            low = midpoint + 1

    return None


def test_when_found():
    assert binary_search([1,2,3,4,5,6,7,8,9,10], 1) == 0
    assert binary_search([1,2,3,4,5,6,7,8,9,10], 5) == 4
    assert binary_search([1,2,3,4,5,6,7,8,9,10], 10) == 9


def test_when_not_found():
    assert binary_search([1,2,3,4], 0) == None
    assert binary_search([1,2,3,4], 5) == None
