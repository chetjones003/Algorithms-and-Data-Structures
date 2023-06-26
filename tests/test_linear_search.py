from linear_search import linear_search

def test_when_is_found():
    assert linear_search([1,2,3,4], 1) == 0
    assert linear_search([1,2,3,4], 2) == 1
    assert linear_search([1,2,3,4], 3) == 2
    assert linear_search([1,2,3,4], 4) == 3

def test_when_not_found():
    assert linear_search([1,2,3,4], 0) == None
    assert linear_search([1,2,3,4], 5) == None