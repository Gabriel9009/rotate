fn main() {     
    let x = [[1,2,3],[4,5,6],[7,8,9]];
    let sd = x.len();
    for i in 0..sd{
        for j in (0..sd).rev(){
        print!("{:?}", x[j][i]);
        }
       println!();
    }
    
}
