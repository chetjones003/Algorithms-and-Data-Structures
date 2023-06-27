from bubble_sort import bubble_sort

def test_sorted_array():
    assert bubble_sort([1, 3, 4, 7, 2]) == [1, 2, 3, 4, 7]
    assert bubble_sort([]) == None