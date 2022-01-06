use itertools::Itertools;
// Rust標準ではないItertoolsトレイトをuse

fn main() {
    let iter1 = vec![1, 2, 3].into_iter();
    let product1 = iter1
        // Rust標準の型にメソッドを後付けで追加できる
        .cartesian_product(vec!['a', 'b'])
        .collect::<Vec<(_, _)>>();
    println!("{:?}", product1);
    // => [(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b'), (3, 'a'), (3, 'b')]
    let iter2 = vec![1, 2, 3].into_iter();
    // メソッド名が重複時はコンパイルエラーになるので安全
    // 下記のように明示的に呼び出す関数を指定して衝突回避可能
    let product2 = Itertools::cartesian_product(iter2, vec!['a', 'b']).collect::<Vec<(_, _)>>();
    println!("{:?}", product2);
    // => [(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b'), (3, 'a'), (3, 'b')]

}
