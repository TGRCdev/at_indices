use select_indices::prelude::*;
use ndarray::prelude::*;

#[test]
fn select_indices()
{
    let mut data = arr2(&[
        [0,0,9,0,0],
        [0,0,9,0,0],
        [0,0,9,0,0],
        [0,0,9,0,0],
        [0,0,9,0,0],
    ]);

    let indices = [(1,2),(3,2),(0,2),(2,2),(4,2)];

    data.select_indices_mut(&indices).enumerate().for_each(|(i, x)| *x = i+1);

    assert_eq!(
        data,
        arr2(&[
            [0,0,3,0,0],
            [0,0,1,0,0],
            [0,0,4,0,0],
            [0,0,2,0,0],
            [0,0,5,0,0],
        ])
    )
}

#[test]
#[should_panic]
fn out_of_range_panic()
{
    let mut data = arr2(&[
        [1,0,0],
        [0,2,0],
        [0,0,3],
    ]);

    let indices = [(1,1),(2,2),(3,3)];

    data.select_indices_mut(&indices) // (3,3) is out of bounds: should panic
        .for_each(|x| {
            println!("{x}");
        })
}

#[test]
#[should_panic]
fn repeated_index_panic()
{
    let mut data = arr2(&[
        [1,0,0],
        [0,2,0],
        [0,0,3],
    ]);
    let indices = [
        (1,1),(1,1)
    ];

    data.select_indices_mut(&indices) // Repeated index: should panic
        .eq(&[2,2]);
}


#[cfg(feature = "rayon")]
mod rayon {
    use select_indices::prelude::*;
    use ndarray::prelude::*;
    use rayon::prelude::*;

    #[test]
    fn select_indices()
    {
        let mut data = arr2(&[
            [0,0,9,0,0],
            [0,0,9,0,0],
            [0,0,9,0,0],
            [0,0,9,0,0],
            [0,0,9,0,0],
        ]);

        let indices = [(1,2),(3,2),(0,2),(2,2),(4,2)];

        data.par_select_indices_mut(&indices).enumerate().for_each(|(i, x)| *x = i+1);

        assert_eq!(
            data,
            arr2(&[
                [0,0,3,0,0],
                [0,0,1,0,0],
                [0,0,4,0,0],
                [0,0,2,0,0],
                [0,0,5,0,0],
            ])
        )
    }

    #[test]
    #[should_panic]
    fn out_of_range_panic()
    {
        let mut data = arr2(&[
            [1,0,0],
            [0,2,0],
            [0,0,3],
        ]);

        let indices = [(1,1),(2,2),(3,3)];

        data.par_select_indices_mut(&indices) // (3,3) is out of bounds: should panic
            .eq(&[2,3,4]);
    }

    #[test]
    #[should_panic]
    fn repeated_index_panic()
    {
        let mut data = arr2(&[
            [1,0,0],
            [0,2,0],
            [0,0,3],
        ]);
        let indices = [
            (1,1),(1,1)
        ];

        data.par_select_indices_mut(&indices) // Repeated index: should panic
            .eq(&[2,2]);
    }

    #[test]
    fn mutable_indexed()
    {
        let mut data = arr2(&[
            [11, 22, 33, 44, 55, 66, 77, 88],
            [99, 00, 11, 22, 33, 44, 55, 66],
            [77, 88, 99, 00, 11, 22, 33, 44],
        ]);
        
        data.par_select_indices_mut(&[(0,4), (2,7), (1,3), (0,0), (2,3)]).indexed().for_each(|(i, x)| {
            println!("data[{i:?}] = {x:02}");
        });
    }
}