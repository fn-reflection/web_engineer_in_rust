use chrono::NaiveDateTime; // タイムゾーン無しの日時
use std::sync::{Arc, Mutex}; // スレッドセーフな共有参照とMutexロックを使用

#[derive(Copy, Clone, Debug)]
// 観測データの構造体(データの組)を定義
struct Measurement {
    // 日時(タイムゾーン無し)
    time: NaiveDateTime,
    // 観測値
    value: f64,
    // データを観測したスレッドID
    thread_id: usize,
}

impl Measurement {
    // Measurement構造体の関連関数(メソッド)を定義
    fn new(value: f64, thread_id: usize) -> Self {
        Measurement {
            // 生成時の現在時刻を記録
            time: chrono::Utc::now().naive_utc(),
            // 観測値を登録, キーと変数名と同じなら省略記法が使える
            value,
            // データを生成したスレッドを記録
            thread_id,
        }
    }
}

fn main() -> anyhow::Result<()> {
    // キューの生成、キューの所有権はメインスレッドが持つ
    let queue = Vec::new();
    // ①：キューのロック付(Mutex)の共有参照(Arc)を定義する
    let arc_queue = Arc::new(Mutex::new(queue));
    // キューの共有参照をコピー(そうしないと複数のスレッドからアクセスできない)
    let arc_queue1 = arc_queue.clone();
    // データ記録スレッド1を作成
    // moveで共有参照を別スレッドに移動
    // 共有参照を移動＝キューの参照権限を別スレッドにも渡したと考えるとわかりやすい
    // Mutexがついているので、編集権限も渡せる
    let record_thread1 = std::thread::spawn(move || {
        for i in 1..=10000 {
            let m = Measurement::new(i as f64, 1);
            // ②: キューのロックを取りキューに観測値を記録
            arc_queue1.lock().unwrap().push(m);
        }
    });

    let arc_queue2 = arc_queue.clone();
    // データ記録スレッド2を作成
    let record_thread2 = std::thread::spawn(move || {
        for i in 1..=10000 {
            let m = Measurement::new(i as f64, 2);
            arc_queue2.lock().unwrap().push(m);
        }
    });

    // データ観測スレッドを作成
    // 1ミリ秒のスリープを挟みつつ、キューの最新値を出力
    let observe_thread = std::thread::spawn(move || loop {
        {
            let queue = arc_queue.lock().unwrap();
            let latest = queue.last();
            println!("{:?}", latest);
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    });
    // 2つのデータ記録スレッドの終了を待つ
    for thread in [record_thread1, record_thread2] {
        let _ = thread.join();
    }
    Ok(())
}
