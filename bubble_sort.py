from typing import List, Optional


def bubble_sort(arr: List[int]) -> Optional[List[int]]:
    swapped: bool = False

    if len(arr) == 0:
        return None

    for i in range(len(arr) - 1):
        for j in range(len(arr) - 1 - i):
            if arr[j] > arr[j + 1]:
                tmp = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = tmp
                swapped = True

    if swapped:
        return arr
