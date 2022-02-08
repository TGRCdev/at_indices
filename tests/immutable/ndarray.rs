use select_indices::prelude::*;
use ndarray::prelude::*;

#[test]
fn select_indices_test()
{
    let data = arr2(&[
        [0,0,2,0,0],
        [0,1,0,3,0],
        [0,0,4,5,0],
    ]);

    let indices = [
        (1, 1), (0, 2),
        (1, 3), (2, 2),
        (2, 3), (2, 3),
        (2, 2), (1, 3),
        (0, 2), (1, 1),
    ];

    assert!(
        data.select_indices(&indices).eq(&[1,2,3,4,5,5,4,3,2,1])
    )
}

#[test]
#[should_panic]
fn out_of_range_panic()
{
    let data = arr2(&[
        [1,0,0],
        [0,2,0],
        [0,0,3],
    ]);
    let indices = [
        [1,1],[2,2],[3,3]
    ];

    data.select_indices(&indices) // 3 is out of bounds: should panic
        .for_each(|x| {
            println!("{x}");
        })
}

#[test]
fn indexed()
{
    let data = arr2(&[
        [11, 22, 33, 44, 55, 66, 77, 88],
        [99, 00, 11, 22, 33, 44, 55, 66],
        [77, 88, 99, 00, 11, 22, 33, 44],
    ]);
    
    data.select_indices(&[(0,4), (2,7), (1,3), (0,0), (2,3)]).indexed().for_each(|(i, x)| {
        println!("data[{i:?}] = {x:02}");
    });
}


#[cfg(feature = "rayon")]
mod rayon {
    use select_indices::prelude::*;
    use ndarray::prelude::*;
    use rayon::prelude::*;

    #[test]
    fn select_indices_test()
    {
        let data = arr2(&[
            [0,0,2,0,0],
            [0,1,0,3,0],
            [0,0,4,5,0],
        ]);

        let indices = [
            (1, 1), (0, 2),
            (1, 3), (2, 2),
            (2, 3), (2, 3),
            (2, 2), (1, 3),
            (0, 2), (1, 1),
        ];

        assert!(
            data.par_select_indices(&indices).eq(&[1,2,3,4,5,5,4,3,2,1])
        )
    }

    #[test]
    #[should_panic]
    fn out_of_range_panic()
    {
        let data = arr2(&[
            [1,0,0],
            [0,2,0],
            [0,0,3],
        ]);
        let indices = [
            [1,1],[2,2],[3,3]
        ];

        data.par_select_indices(&indices) // 3 is out of bounds: should panic
            .for_each(|x| {
                println!("{x}");
            })
    }

    #[test]
    fn indexed()
    {
        let data = arr2(&[
            [11, 22, 33, 44, 55, 66, 77, 88],
            [99, 00, 11, 22, 33, 44, 55, 66],
            [77, 88, 99, 00, 11, 22, 33, 44],
        ]);
        
        data.par_select_indices(&[(0,4), (2,7), (1,3), (0,0), (2,3)]).indexed().for_each(|(i, x)| {
            println!("data[{i:?}] = {x:02}");
        });
    }
}