use oxido_lexer::inputfile::InputFile;
use oxido_lexer::token::Token;
use oxido_lexer::lexer::Lexer;

fn main() {
    // Tive que remover os testes, perdão igor :(
    // Não sabia como rodar os testes de um projeto de dentro de outro
    let source_text = "extern \"BeginDrawing\" fn begdraw();"; // Apparently it works
    let lx = Lexer::new(InputFile::new(
        "main.rs",
        source_text.chars(),
    ));

    let tokens: Vec<Token> = lx.collect();
    println!("{:?}", tokens);
}
