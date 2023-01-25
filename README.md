# rs-uptobox

This crate is a wrapper around the uptobox api.

## Details

```rust
// Get the files of //dev
let res = uptobox.get_files(&GetFiles::new("//dev")).await;

// Get the files of //dev with an order_by, an offset and a limit
let res = uptobox
    .get_files(
        &GetFiles::new("//dev")
            .order_by(OrderBy::FileSize)
            .offset(2)
            .limit(50),
    )
    .await;

// Move the files to a folder
let res = uptobox
    .move_files(vec!["file_id_1", "file_id_2"], 000000000)
    .await;
```