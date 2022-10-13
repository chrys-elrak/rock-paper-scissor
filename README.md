# ROCK, PAPER, SCISSOR

This is a simple game on terminal made with Rust.

## INSTALL

To install this on your computer, You will need Rust installed in your computer.  
Then you can build with `cargo build release`.
Then go to `./target/release` and add this to you PATH.
Now you can use `rock-paper-scissor` commande on you terminal. Have fun an enjoy ;)

## CONTRIBUTE
Fork the repository and make your change. Satisfy ? Create a PR, I'll review you code an merge it as good as well.

## GLOBALIZATION

To add new language, there is a few step to do:
 - Go to `src/lang/` and create the json file translation.
 - Try to copy the structure of the existing translation file.
 - Open `src/models/lang.rs` and add inside the enum `AvalaibleLang` the key code for the new language, it must unique and for the
 convention, it should be in uppercase.
 - Then, go to the implementation for the `Lang` struct, in the `new` function you must provide the json file.
 - Finaly, insert to the `main` the ability to choose this language.

 **Note**:
 - For adding new text in translation file, the key must be in english format and uppercase.
 - After that, you must insert into `Lang` struct the key with `String` type for the value.
 
## CONTRIBUTORS
- [Chrys Rakotonimanana](https://github.com/chrys-elrak)

*Copyright 2022 - Chrys Rakotonimanana*
