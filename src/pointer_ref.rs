pub fn run(){
    let array: [i32; 3] = [1,2,3];
    let array2: [i32;3] = array;
    
    println!("{:?}", (array, array2));

    let vec1: Vec<i32> = vec![1,2,3];
    let vec2: &Vec<i32> = &vec1;

    println!("{:?}", (&vec1, vec2));
}