foreach ($example in Get-ChildItem -Path "spirq/examples") {
    & cargo run --example "$($example.Name)" > "spirq/examples/$($example.Name)/main.log" 
}
