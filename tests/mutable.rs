use at_indices::prelude::*;

#[test]
fn mutable_at_indices_test()
{
    let mut data = [
        0,0,9,0,0,
        0,0,9,0,0,
        0,0,9,0,0,
        0,0,9,0,0,
        0,0,9,0,0,
    ];

    let indices = [7,17,2,12,22];

    data.at_indices_mut(&indices).enumerate().for_each(|(i, x)| *x = i+1);

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
fn mutable_out_of_range_panic()
{
    let mut data = [1,2,3];
    let indices = [1,2,3];

    data.at_indices_mut(&indices) // 3 is out of bounds: should panic
        .eq(&[1,2,3]);
}

#[test]
#[should_panic]
fn mutable_repeated_index_panic()
{
    let mut data = [1,2,3];
    let indices = [1,1];

    data.at_indices_mut(&indices) // Repeated index: should panic
        .eq(&[2,2]);
}

#[cfg(feature = "rayon-iters")]
mod rayon
{
    use at_indices::prelude::*;
    use rayon::prelude::*;

    #[test]
    fn par_mutable_at_indices_test()
    {
        let mut data = [
            0,0,9,0,0,
            0,0,9,0,0,
            0,0,9,0,0,
            0,0,9,0,0,
            0,0,9,0,0,
        ];

        let indices = [7,17,2,12,22];

        data.par_at_indices_mut(&indices).enumerate().for_each(|(i, x)| {
            *x = i+1;
        });

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
    fn par_mutable_out_of_range_panic()
    {
        let mut data = [1,2,3];
        let indices = [1,2,3];

        data.par_at_indices_mut(&indices) // 3 is out of bounds: should panic
            .eq(&[1,2,3]);
    }

    #[test]
    #[should_panic]
    fn par_mutable_repeated_index_panic()
    {
        let mut data = [1,2,3];
        let indices = [1,1];

        data.par_at_indices_mut(&indices) // Repeated index: should panic
            .eq(&[2,2]);
    }
}