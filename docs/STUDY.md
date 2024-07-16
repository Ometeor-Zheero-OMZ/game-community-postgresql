# uuid v4 と uuid v7 の違いとその使い分け

```Rust
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use uuid::Uuid;

fn main() -> Result<()> {
    for _ in 0..3 {
        let uuid = Uuid::new_v4();
        println!("{:>12}: {uuid}", "uuid v4");
    }

    println!();

    for _in 0..3 {
        let uuid = Uuid::new_v7();
        println!("{:>12}: {uuid}", "uuid v7");
    }

    Ok(())
}
```

uuid v4 は完全な乱数だが、uuid v7 は先頭の 48 ビットは複数生成する際に同じ乱数の値が生成される

REf:
Jernemy Chone
UUID v7 vs. v4 + Rust Programming Examples
https://www.youtube.com/watch?v=zIebRwU0FOw
