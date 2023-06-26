from typing import List, Optional

def linear_search(arr: List[int], target: int) -> Optional[int]:
    for index, item in enumerate(arr):
        if item == target:
            return index

    return None
