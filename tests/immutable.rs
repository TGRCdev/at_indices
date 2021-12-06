use select_indices::prelude::*;

#[test]
fn immutable_select_indices_test()
{
    let data = [
        0,0,2,0,1,
        0,0,0,0,0,
        0,0,4,5,3
    ];

    let indices = [4, 13, 2, 12, 14, 14, 12, 2, 13];

    assert!(
        data.select_indices(&indices).eq(&[1, 5, 2, 4, 3, 3, 4, 2, 5])
    )
}

#[test]
#[should_panic]
fn immutable_out_of_range_panic()
{
    let data = [1,2,3];
    let indices = [1,2,3]; 

    data.select_indices(&indices) // 3 is out of bounds: should panic
        .eq(&[1,2,3]);
}

#[test]
fn immutable_repeated_index()
{
    let data = [1,2,3];
    let indices = [1,1];

    data.select_indices(&indices) // Repeated index: valid when immutable
        .eq(&[2,2]);
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

        let indices = [4, 13, 2, 12, 14, 14, 12, 2, 13];

        assert!(
            data.par_select_indices(&indices).eq(&[1, 5, 2, 4, 3, 3, 4, 2, 5])
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

    #[test]
    fn par_immutable_repeated_index()
    {
        let data = [1,2,3];
        let indices = [1,1];

        data.par_select_indices(&indices) // Repeated index: valid when immutable
            .eq(&[2,2]);
    }
}