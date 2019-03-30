fn main() {

  let a:[[f32; 3]; 3]=[[1.0,2.0,6.0],[8.0,5.0,2.0],[8.0,6.0,3.0]];
  let b:[[f32; 3]; 3]=matmul(a,a);

  print!("\n");

  for i in 0..3{
    for j in 0..3{
      print!("{}\t",b[i][j]);
    }
    print!("\n");
  }


}

fn matmul(a:[[f32; 3]; 3],b:[[f32; 3]; 3])->[[f32; 3]; 3]{
  let mut new_a:[[f32; 3]; 3]=[[0.0,0.0,0.0],[0.0,0.0,0.0],[0.0,0.0,0.0]];
  for i in 0..3{
    for j in 0..3{
      for k in 0..3{
        new_a[i][j]+=a[i][k]*b[k][j];
      }
    }
  }
  return new_a;
}
