# µMML Compiler

## About

µMML Compiler or MMML Compiler is a compiler to compiler `.mmml` files into C array bytes to play with an [MMML driver](https://github.com/protodomemusic/mmml/tree/master).

## What is µMML

µMML or Micro Music Macro Language is a derivative of [MML](https://en.wikipedia.org/wiki/Music_Macro_Language)*(Music Macro Language)* for 1-bit music implmentation.

The original µMML compiler is made by [protodomemusic](https://github.com/protodomemusic) in C. If you want to see, get [here.](https://github.com/protodomemusic/mmml/tree/master/compiler)

## How to use

If you want to use, here the use case:

`mmml-compiler [OPTIONS] <INPUT_PATH>`

### Options

|Short name|Long name|Argument|Description|
|----------|---------|--------|-----------|
|-o|--output-path|Path/File name|Output file|
|-m|--music-name|String|Music name in the output file|
|-v|--verbose|None|Output more info *(Debug purpuses only)*|
|-h|--help|None|Print help|
|-V|--version|None|Print version|

## Writing music in µMML

I recommend to see [protodomemusic's guide](https://github.com/protodomemusic/mmml?tab=readme-ov-file#writing-music-in-%CE%BCmml) to see how to make music using µMML.

## How to compile

A simple `cargo build --release` is enough. And if you want to install into your system just do `cargo install` and it will do the job.
