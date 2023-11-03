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
