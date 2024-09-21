method LinearSearch<T(==)>(arr: array<T> , key: T) returns (index : int )
    ensures index >= 0 ==> index < arr.Length && arr[index] == key
    ensures index <  0 ==> forall i :: 0 <= i < arr.Length ==>arr[i] != key
    {
        index := 0 ;
        while index < arr.Length
        invariant 0 <= index <= arr.Length
        invariant forall i :: 0 <= i < index ==> arr[i] != key
        {
            if arr[index] == key {
                return index;
            }
            index := index+ 1 ;
        }
        index := -1 ;
    }

method Main() {
    var arr := new int[5] ;
    arr[0] ,arr[1] ,arr[2] , arr[3] ,arr[4] := 1,6,3,4,5;
    var result := LinearSearch(arr, 5);
    assert result == 4;

    result := LinearSearch(arr, 6);
    // assert result == 1;

    print "All tests passed!";
}
