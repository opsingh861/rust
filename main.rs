// Iterators

fn main() {
    // let v = vec![1, 2, 3, 4, 5, 6];

    // for num in v {  // it is same as v.iter()
    //     print!("{}", num);
    // }

    // print!("{:?}",v); // this will not work as values has been moved v.iter()
    // let v_iter = v.iter();

    // for i in v_iter {
    //     print!("{}", i);
    // }

    // print!("{}", v); // again it will not work as it has been moved to v_iter
    // let mut v = vec![1, 2, 3, 4, 5, 6];

    // let mut v_iter_mut = v.iter_mut();

    // while let Some(val) = v_iter_mut.next() {
    //     print!("{} ", val);
    // }

    // let iter_mut = v.iter_mut();

    // for val in iter_mut {
    //     // mutating the value
    //     *val = *val + 1;
    // }

    // print!("{:?}", v);

    // let iter = v.into_iter();

    // for val in iter { // happens when we run for loop normaly
    //     print!("{} ", val);
    // }
    // // print!("{:?}", v); // ownership moved to iter after using into_iter

    let v = vec![1, 2, 3, 4, 5, 6];
    let result = filter_map(v);

    print!("{:?}", result);

    // iterator in map will be same
}

fn filter_map(v: Vec<i32>) -> Vec<i32> {
    let result = v
        .iter()
        .filter(|value| *value % 2 == 0)
        .map(|val| val + 1)
        .collect(); // collect will give new Vec other we would have recieved new iterator

    return result;
}
