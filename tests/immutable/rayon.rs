use select_indices::prelude::*;
use rayon::prelude::*;

#[test]
fn select_indices_test()
{
    let data = [
        0,0,2,0,1,
        0,0,0,0,0,
        0,0,4,5,3
    ];

    let indices = [4, 2, 14, 12, 13, 13, 12, 14, 2, 4];

    assert!(
        data.par_select_indices(&indices).eq(&[1,2,3,4,5,5,4,3,2,1])
    );
}

#[test]
#[should_panic]
fn out_of_range_panic()
{
    let data = [1,2,3];
    let indices = [1,2,3];

    println!("{}", data.par_select_indices(&indices).eq(&[2,3,4]));
}