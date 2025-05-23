use std:collections:HashMap;
use std:fmt:

#[derive(Debug, Clone)]
struct Livro {
    id:u32,
    titulo: String,
    autor: String,
    isbn: String,
    ano_publicacao: u16,
    disponivel:bool,
    categoria: Categoria,
}

#[derive(Debug, Clone)]
enum Categoria {
    Ficcao,
    NaoFiccao,
    Ciencia,
    Historia,
    Bibiografia,
    Romance, 
    Terror,
    Fantasia,
    Trilogias,
}

#[derive (Debug, Clone)]
struct Usuario {
    id: u32,
    nome: String,
    email: String,
    telefone: String,
    livros_emprestadas: Vec<u32>,
}

#[derive (Debug, Clone)]
struct Emprestimo {
    id: u32,
    usuario_id: u32,
    livro_id: u32,
    data_emprestimo: String,
    data_devolucao_prevista: String,
    devolvido: bool,
}

#[derive(Debug)]
enum ErroSistema {
    LivroNaoEncontrado(u32),
    UsuarioNaoEncontrado(u32),
    LivroJaEmprestado(u32),
    LivroNaoEmprestado(u32),
    UsuarioJaPossuiLivro(u32, u32),
    LimiteEmprestimosExcedido(u32),
}

impl fmt::Display for ErroSistema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErroSistema::LivroNaoEncotrado(id) => write!(f, "Livro com ID {} não encontrado", id),
            ErroSistema::UsuarioNaoEncontrado(id) => write!(f, "Usuario com ID {} não encontrado, id),
            ErroSistema::LivroJaEmprestado(id) => write!(f, "Livro com ID {} já emprestado", id),
            ErroSistema::LivroNaoEmprestado(id) => write!(f, "Livro com ID {} não está emprestado", id),
            write!(f, "Usuario {} já possui o livro {}", user_id, book_id)
        }
        ErroSistema::LimiteEmprestimoExcedido(id) => {
            write!(f, "Usuário {} excedeu o limite de empréstimo (máximo 3)", id)
        }
    }
}

struct SistemaBiblioteca {
    livros> HashMap<u32, Livro>,
    usuarios: HashMap<u32, Usuario>,
    emprestimos: HashMap<u32, Emprestimo>,
    proximo_id_livro: u32,
    proximo_id_usuario: u32,
    proximo_id_emprestimo: u32,
}

impl SistemaBiblioteca {
    fn novo() -> Self {
        SistemaBiblioteca {
            livros: HashMap::new(),
            usuarios: HashMap::new(),
            emprestimos: HashMap::new(),
            proximo_id_livro: 1,
            proximo_id_usuario: 1,
            proximo_id_emprestimo: 1,
        }
    }
}

fn adicionar_livro(&mut self, titulo: String, autor: String, isbn: String,
    ano: u16, categoria: Categoria) -> u32 {
        let id = self.proximo_id_livro;
        let livro = Livro {
            id,
            titulo,
            autor,
            isbn,
            ano_publicacao: ano,
            disponivel: true,
            categoria,
        };
        self.livros.insert (id, livro);
        self.proximo_id_livro += 1;
        id
    }

    fn buscar_livro_por_id(&self, id: u32) -> Option<&Livro> {
        self.livros.get(&id)
    }

    fn buscar_livro_por_autor(&self, autor: ustr) -> Vec<&Livro> {
        self.livros.values()
        .filter(|livro| livro.titulo.to_lowercase().contains(&titulo.to_lowercase()))
        .collect()
    }

    fn listar_livros_disponiveis(&self) -> Vec<&Livro> {
        self.livros.values()
        .filter(|livro| livro.disponivel)
        .collect()
    }

fn listar_livros_por_categoria(&self, categoria: &Categoria) -> Vec<&Livro> {
    self.livros.values()
    .filter(|livro| std::mem::discriminant(&livro.categoria) == std::mem::discriminant(categoria))
    .collect()
}

fn adicionar_usuario(&mut self, nome: String, email: String, telefone: String) -> u32 {
    let id = self.proximo_id_usuario;
    let usuario = Usuario {
        id,
        nome,
        email,
        telefone,
        livros_emprestados: Vec::new(),
    
    };
    self.usuarios.insert(id, usuario);
    self.proximo_id_usuario += 1;
    id
}

fn buscar_usuario(&self, id: u32) -> Option<&Usuario> {
    self.usuarios.get(&id)
}

fn emprestar_livro(&mut self, usuario_id> u32, livro_id: u32,
                data_emprestimo: String, data_devolucao: String) -> Result<u32, ErroSistema> {
    //verificar se o usuario existe
    let usuario = self.usuarios.get(&usuario_id)
    .ok_or(ErroSistema::UsuarioNaoEncontrado(usuario_id))?;

    if usuario.livros_emprestados.len() >= 3 {
        return Err(ErroSistema::LimiteEmprestimoExcedido(usuario_id));
    }
}

    let livro = self.livros.get(&livro_id) {
    .ok_or(ErroSistema::LivroNaoEncontrado(livro_id));
}

    if !livro.disponivel {
    return Err(ErroSistema::LivroJaEmprestado(livro_id));
}

    if usuario.livros_emprestados.contains(&livro_id) {
        return Err(ErroSistema::UsuarioJaPossuiLivro(usuario_id, livro_id));
}

    let emprestimo_id = self.proximo_id_emprestimo; 
    let emprestimo = Emprestimo {
        id: emprestimo_id,
        usuario_id,
        livro_id,
        data_emprestimo,
        data_devolucao_prevista> data_devolucao,
        devolvido: false,
};

    self.emprestimos.insert(emprestimo_id, emprestimo);
    self.livros.get_mut(&livro_id).unwrap().disponivel = false;
    self.usuarios.get_mut(&usuario_id).unwrap().livros_emprestados.push(livro_id);
    self.proximo_id_emprestimo += 1;

    Ok(emprestimo_id)
}

fn devolver_livro(&mut self, usuario_id: u32, livro_id: u32) -> Result<(), ErroSistema> {
    let emprestimo_id = self.emprestimos.iter()
    .find(|(_, e)| e.usuario_id == usuario_id && e.livro_id == livro_id == livro_id && !e.devolvido)
    .map(|(id, _)| *id)
    .ok_or(ErroSistema::LivroNaoEmprestado(livro_id))?;


    self.emprestimos.get_mut(&emprestimo_id).unwrap().devolvido = true;

    self.livros.get_mut(&livro_id).unwrap().disponivel = true;

    let usuario = self.usuarios.get_mut(&usuario_id).unwrap();
    usuario.livros_emprestados.retain(|&x| x != livro_id);

    Ok(())
}

fn relatorio_usuario(&self, usuario_id: u32) -> Result<String, ErroSistema> {
    let usuario = self.usuarios.get(&usuario_id)
    .ok_or(ErroSistema::UsuarioNaoEncontrado(usuario_id))?;

    let mut relatorio = format! ("=== RELATÓRIO DO USUÁRIO ===\n");
    relatorio.push_str(&format!("Nome: {}\n", usuario.nome));
    relatorio.push_str(&format!("Email: {}\n", usuario.email));
    relatorio.push_str(&format!("Telefone: {}\n," usuario.telefone));
    relatorio.push_str(&format!("Telefone: {}\n," usuario.livros_emprestadas.len()));

    if !usuario.livros_emprestadas.is_empty() {
        relatorio.push_str("\nLivros atualmente emprestados:\n");
        for &livro_id in &usuario.livros_emprestados {
            if let Some(livro) = self.livros.get (&livro_id) {
                relatorio.push_str(&format!(" - {} por {}\n", livro.titulo, livro.autor));
            }
        }
    }

    Ok(relatorio)
}

fn estatisticas (&self) -> String {
    let total_livros = self.livros.len();
    let listar_livros_disponiveis = self.livros.values().filter (|1| 1.disponivel).count();
    let livros_emprestados = total_livros - listar_livros_disponiveis;
    let total_usuarios = self.usuarios.len();
    let emprestimos_ativos = self.emprestimos.values().filter(|e| !e.devolvido).count();

    format!(
        total de livros: {}\n\
        Livros disponiveis = {}\n\
        Livros emprestados = {}\n\
        Total de usuarios = {}\n\
        Emprestimos Ativos = {}\n\
        total_livros, listar_livros_disponiveis, livros_emprestados,
        total_usuarios, emprestimos_ativos
    )
}

fn main () {
    let mut biblioteca = SistemaBiblioteca::novo(;

    let livro1 = biblioteca.adicionar_livro()
        "O senhor dos aneis".to_string(),
        "J.R.R Tolkien".to_string(),
        "978-0544003415".to_string(),
        1954,
        Categoria::Fantasia
    );

   let livro2 = biblioteca.adicionar_livro(
        "Jurassic Park".to_string(),
        "Jonas Tomowwor".to_string(),
        "978-2410001420".to_string(),
        1973,
        Categoria::Ficcao
    );
    let livro3 = biblioteca.adicionar_livro(
        "Homem Aranha".to_string(),
        "Spirbl Pinto".to_string(),
        "978-8481001901".to_string(),
        1973,
        Categoria::Ficcao
    );

    let usuario1 = biblioteca.adicionar_usuario(
        "Ana Silva".to_string(),
        "ana@email.com".to_string(),
        "(11) 99999-9999".to_string()
    );

    let usuario2 = biblioteca.adicionar_usuario(
        "João Santos".to_string(),
        "joao@email.com".to_string(),
        "(11) 88888-8888".to_string()
    );

    println!("=== SISTEMA DE BIBLIOTECA INICIALIZADO ===\n");

  
    match biblioteca.emprestar_livro(usuario1, livro1, "2024-01-15".to_string(), "2024-02-15".to_string()) {
        Ok(emprestimo_id) => println!("✅ Empréstimo {} realizado com sucesso!", emprestimo_id),
        Err(e) => println!("❌ Erro: {}", e),
    }

    match biblioteca.emprestar_livro(usuario2, livro2, "2024-01-16".to_string(), "2024-02-16".to_string()) {
        Ok(emprestimo_id) => println!("✅ Empréstimo {} realizado com sucesso!", emprestimo_id),
        Err(e) => println!("❌ Erro: {}", e),
    }

    match biblioteca.emprestar_livro(usuario2, livro1, "2024-01-17".to_string(), "2024-02-17".to_string()) {
        Ok(emprestimo_id) => println!("✅ Empréstimo {} realizado com sucesso!", emprestimo_id),
        Err(e) => println!("❌ Erro esperado: {}", e),
    }

    println!("\n{}", biblioteca.estatisticas());

    println!("=== BUSCA POR AUTOR ===");
    let livros_tolkien = biblioteca.buscar_livros_por_autor("tolkien");
    for livro in livros_tolkien {
        println!("📚 {} - {} ({})", livro.titulo, livro.autor, livro.ano_publicacao);
    }

    println!("\n=== RELATÓRIO DE USUÁRIO ===");
    match biblioteca.relatorio_usuario(usuario1) {
        Ok(relatorio) => println!("{}", relatorio),
        Err(e) => println!("Erro: {}", e),
    }

    println!("=== EMPRÉSTIMOS ATIVOS ===");
    let emprestimos_ativos = biblioteca.relatorio_livros_emprestados();
    for (emprestimo, livro, usuario) in emprestimos_ativos {
        println!("📖 '{}' emprestado para {} (devolução: {})", 
                livro.titulo, usuario.nome, emprestimo.data_devolucao_prevista);
    }

    println!("\n=== DEVOLVENDO LIVRO ===");
    match biblioteca.devolver_livro(usuario1, livro1) {
        Ok(()) => println!("✅ Livro devolvido com sucesso!"),
        Err(e) => println!("❌ Erro: {}", e),
    }

    println!("\n{}", biblioteca.estatisticas());
}