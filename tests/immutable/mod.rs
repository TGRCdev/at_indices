use select_indices::prelude::*;

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
        data.select_indices(&indices).eq(&[1,2,3,4,5,5,4,3,2,1])
    )
}

#[test]
#[should_panic]
fn out_of_range_panic()
{
    let data = [1,2,3];
    let indices = [1,2,5]; 

    data.select_indices(&indices) // 3 is out of bounds: should panic
        .for_each(|x| {
            println!("{x}");
        })
}

#[test]
fn indexed()
{
    let data = vec![
        11, 22, 33, 44, 55, 66, 77, 88,
        99, 00, 11, 22, 33, 44, 55, 66,
        77, 88, 99, 00, 11, 22, 33, 44
    ];
    
    data.select_indices(&[4, 23, 11, 0, 19]).indexed().for_each(|(i, x)| {
        println!("data[{i:2}] = {x:02}");
    });
}

#[test]
fn indexed_halfway()
{
    let data = [
        11, 22, 33, 44, 55, 66, 77, 88,
        99, 00, 11, 22, 33, 44, 55, 66,
        77, 88, 99, 00, 11, 22, 33, 44
    ];

    let mut iter = data.select_indices(&[4, 23, 12, 21, 0]);
    assert_eq!(iter.next().cloned(), Some(55));
    assert_eq!(iter.next().cloned(), Some(44));
    assert_eq!(iter.next().cloned(), Some(33));

    let mut iter = iter.indexed();
    assert_eq!(iter.next().map(|(i, v)| (i, *v)), Some((21, 22)));
    assert_eq!(iter.next().map(|(i, v)| (i, *v)), Some((0, 11)));
    assert_eq!(iter.next(), None);
}

#[cfg(feature = "rayon")]
mod rayon;

#[cfg(feature = "ndarray")]
mod ndarray;