#!/usr/bin/env python3

from typing import List


def linear_search(arr: List[int], target: int):
    for i in range(len(arr)):
        if arr[i] == target:
            print(i)
            return i

    print("did not find")
    return -1


def test_when_is_found():
    assert linear_search([1,2,3,4], 1) == 0
    assert linear_search([1,2,3,4], 2) == 1
    assert linear_search([1,2,3,4], 3) == 2
    assert linear_search([1,2,3,4], 4) == 3

def test_when_not_found():
    assert linear_search([1,2,3,4], 0) == -1
    assert linear_search([1,2,3,4], 5) == -1
