fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The const value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    // shadowing 1
    // letを使い同じ変数名を変換する.
    // mutは同じ変数の値を変える.let(shadowing)は別の変数を定義することになる.
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // 12
    }
    println!("The value of y is: {y}"); // 6
    // shadowing 2
    // 別の型で宣言し直すことも可能.
    let spaces = "   "; // 文字列型
    let spaces = spaces.len(); // 数値型
    println!("The value of spaces is: {spaces}"); // 3
    // 以下はコンパイルエラー
    // let mut spaces = "   ";
    // spaces = spaces.len(); // 文字列型の変数に数値を入れようとするため
}
