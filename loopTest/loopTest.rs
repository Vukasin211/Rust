fn main()
{
    let x = 5;
    for i in 0..10 {
        if x == i {
            assert_eq!(x, i);
        }
    }
}