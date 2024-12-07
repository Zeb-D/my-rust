/* 插入排序 - 通过插入到适当位置进行排序
   - 从第二个元素开始，与前面的元素比较以找到正确位置并插入。
*/
// fn sort(arr : &mut Vec<usize>, n : usize){
//     for i in 1..n{
//         let key = arr[i];
//         let mut j: i32 = (i - 1) as i32;
//         while j >= 0 && arr[j as usize] < key {
//                 arr[(j + 1) as usize] = arr[j as usize];
//                 j -= 1;
//             }
//             arr[(j + 1) as usize] = key;
//     }
// }
// pub fn example() {
//     let mut arr = vec![5, 2, 9, 1, 5, 6];
//     let n = arr.len();
//     sort(&mut arr, n);
//     println!("{:?}", arr);
// }
