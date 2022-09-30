foreach ($example in Get-ChildItem -Path "$PWD/examples") {
    & cargo run --example "$($example.Name)" > "examples/$($example.Name)/main.log" 
}
