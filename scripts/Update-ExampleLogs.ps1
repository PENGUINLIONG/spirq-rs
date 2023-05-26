foreach ($example in Get-ChildItem -Path "spirq/examples") {
    & cargo run --example "$($example.Name)" > "spirq/examples/$($example.Name)/main.log" 
}

foreach ($shader in Get-ChildItem -Path "assets") {
    if ($shader.Name -notlike "*.json") {
        & cargo run -p shader-reflect "$shader" --reference-all-resources > "assets/$($shader.Name).json" 
    }
}
