use select_indices::prelude::*;

#[test]
fn select_indices()
{
    let mut data = [
        0,0,9,0,0,
        0,0,9,0,0,
        0,0,9,0,0,
        0,0,9,0,0,
        0,0,9,0,0,
    ];

    let indices = [7,17,2,12,22];

    data.select_indices_mut(&indices).enumerate().for_each(|(i, x)| *x = i+1);

    assert_eq!(
        data,
        [
            0,0,3,0,0,
            0,0,1,0,0,
            0,0,4,0,0,
            0,0,2,0,0,
            0,0,5,0,0,
        ]
    )
}

#[test]
#[should_panic]
fn out_of_range_panic()
{
    let mut data = [1,2,3];
    let indices = [1,2,3];

    data.select_indices_mut(&indices) // 3 is out of bounds: should panic
        .for_each(|x| {
            println!("{x}");
        })
}

#[test]
#[should_panic]
fn repeated_index_panic()
{
    let mut data = [1,2,3];
    let indices = [1,1];

    data.select_indices_mut(&indices) // Repeated index: should panic
        .for_each(|x| println!("{x}") )
}

#[test]
fn indexed_halfway()
{
    let mut data = [
        11, 22, 33, 44, 55, 66, 77, 88,
        99, 00, 11, 22, 33, 44, 55, 66,
        77, 88, 99, 00, 11, 22, 33, 44
    ];

    let mut iter = data.select_indices_mut(&[4, 23, 12, 21, 0]);
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