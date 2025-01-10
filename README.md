

## Usage

```rust
use fusio_log::Path;

#[tokio::main]
async fn main() {
    let temp_dir = TempDir::new().unwrap();
    let path = Path::from_filesystem_path(temp_dir.path())
        .unwrap()
        .child("log");

    let mut logger = Options::new(path.clone()).build::<u8>().await.unwrap();
    logger.write(&1).await.unwrap();
    logger.write_batch([2, 3, 4].into_iter()).await.unwrap();
    logger
        .write_batch([2, 3, 4, 5, 1, 255].into_iter())
        .await
        .unwrap();

    logger.flush().await.unwrap();
    logger.close().await.unwrap();

    let expected = [vec![1], vec![2, 3, 4], vec![2, 3, 4, 5, 1, 255]];
    let stream = Options::new(path)
        .recover::<u8>()
        .await
        .unwrap()
        .into_stream();
    pin!(stream);
    let mut i = 0;
    while let Some(res) = stream.next().await {
        assert!(res.is_ok());
        assert_eq!(&expected[i], &res.unwrap());
        i += 1;
    }
}
```
