use select_indices::prelude::*;

#[test]
fn immutable_select_indices_test()
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
fn immutable_out_of_range_panic()
{
    let data = [1,2,3];
    let indices = [1,2,5]; 

    data.select_indices(&indices) // 3 is out of bounds: should panic
        .for_each(|x| {
            println!("{}", x);
        })
}

#[test]
fn immutable_indexed()
{
    let data = vec![
        11, 22, 33, 44, 55, 66, 77, 88,
        99, 00, 11, 22, 33, 44, 55, 66,
        77, 88, 99, 00, 11, 22, 33, 44
    ];
    
    data.select_indices(&[4, 23, 11, 0, 19]).indexed().for_each(|(i, x)| {
        println!("data[{:2}] = {:02}", i, x);
    });
}

#[cfg(feature = "rayon")]
mod rayon
{
    use select_indices::prelude::*;
    use rayon::prelude::*;

    #[test]
    fn par_immutable_select_indices_test()
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
    fn par_immutable_out_of_range_panic()
    {
        let data = [1,2,3];
        let indices = [1,2,3]; 

        data.par_select_indices(&indices) // 3 is out of bounds: should panic
            .eq(&[1,2,3]);
    }
}

#[cfg(feature = "ndarray")]
mod ndarray {
    use select_indices::prelude::*;
    use ndarray::prelude::*;

    #[test]
    fn immutable_select_indices_test()
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
    fn immutable_out_of_range_panic()
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
                println!("{}", x);
            })
    }

    #[test]
    fn immutable_indexed()
    {
        let data = arr2(&[
            [11, 22, 33, 44, 55, 66, 77, 88],
            [99, 00, 11, 22, 33, 44, 55, 66],
            [77, 88, 99, 00, 11, 22, 33, 44],
        ]);
        
        data.select_indices(&[(0,4), (2,7), (1,3), (0,0), (2,3)]).indexed().for_each(|(i, x)| {
            println!("data[{:?}] = {:02}", i, x);
        });
    }

    #[cfg(feature = "rayon")]
    mod rayon {
        use select_indices::prelude::*;
        use ndarray::prelude::*;
        use rayon::prelude::*;

        #[test]
        fn par_immutable_select_indices_test()
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
        fn par_immutable_out_of_range_panic()
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
                .eq(&[1,2,3]);
        }

        #[test]
        fn par_immutable_indexed()
        {
            let data = arr2(&[
                [11, 22, 33, 44, 55, 66, 77, 88],
                [99, 00, 11, 22, 33, 44, 55, 66],
                [77, 88, 99, 00, 11, 22, 33, 44],
            ]);
            
            data.par_select_indices(&[(0,4), (2,7), (1,3), (0,0), (2,3)]).indexed().for_each(|(i, x)| {
                println!("data[{:?}] = {:02}", i, x);
            });
        }
    }
}