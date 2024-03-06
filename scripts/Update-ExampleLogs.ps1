foreach ($example in Get-ChildItem -Path "spirq/examples") {
    & cargo run --example "$($example.Name)" > "spirq/examples/$($example.Name)/main.log" 
}

foreach ($shader in Get-ChildItem -Path "assets") {
    # ignore json and direcotries.
    if ($shader.Name -notlike "*.json" -and $shader.Name -notlike "*.spvasm" -and $shader.PSIsContainer -eq $false) {
        & cargo run -p shader-reflect "$shader" --reference-all-resources > "assets/$($shader.Name).json" 
    }
}
