from binary_search import binary_search


def test_when_found():
    assert binary_search([1,2,3,4,5,6,7,8,9,10], 1) == 0
    assert binary_search([1,2,3,4,5,6,7,8,9,10], 5) == 4
    assert binary_search([1,2,3,4,5,6,7,8,9,10], 10) == 9


def test_when_not_found():
    assert binary_search([1,2,3,4], 0) == None
    assert binary_search([1,2,3,4], 5) == None