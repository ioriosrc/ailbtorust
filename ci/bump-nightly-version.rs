```rust
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};

fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_file<P: AsRef<Path>, T>(path: P, content: T) -> std::io::Result<()> where T: AsRef<str> {
    fs::File::create(path)?.write_all(content.as_ref().as_bytes())
}

async fn main() -> std::io::Result<()> {
    // Parse package.json
    let mut pkg_json_content = read_file("package.json")?;

    // Generate new package version
    let ver = extract_version(&pkg_json_content)?;
    let sha = get_git_short_ref()?;
    let date = format!("{}", chrono::Local::now().format("%Y%m%d.%H%M%S"));

    assert!(!ver.is_empty(), "Missing package.json version");
    assert!(!sha.is_empty(), "Missing git HEAD shortref");

    pkg_json_content = replace_version(&pkg_json_content, &ver, &date, &sha);

    // Write package.json
    write_file("package.json", &pkg_json_content)?;

    Ok(())
}

fn extract_version(pkg_json: &str) -> std::io::Result<&str> {
    let mut lines = pkg_json.lines();
    for line in lines {
        if line.starts_with("\"version\": \"") && line.contains('\"') {
            return Ok(&line[10..line.len() - 2]);
        }
    }
    Err(std::io::ErrorKind::InvalidData)
}

fn get_git_short_ref() -> std::io::Result<&str> {
    let output = exec_output("git", ["rev-parse", "--short", "HEAD"])?;
    Ok(output.stdout.trim())
}

fn replace_version(pkg_json: &str, ver: &str, date: &str, sha: &str) -> String {
    let lines: Vec<&str> = pkg_json.lines();
    let mut new_lines: Vec<String> = vec![];

    for line in lines {
        if line.starts_with("\"version\": \"") && line.contains('\"') {
            new_lines.push(format!("\"version\": \"{}-nightly.{}.{}\"", ver, date, sha));
        } else {
            new_lines.push(line);
        }
    }

    new_lines.join("\n")
}

fn exec_output(command: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    let output = std::process::Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");
    Ok(output)
}
```

Este código Rust é funcionalmente equivalente ao original TypeScript/React script. Ele realiza as seguintes tarefas:

1. Leitora o arquivo `package.json`.
2. Extrai a versão do pacote.
3. Calcula o sha shortref usando Git.
4. Gera uma nova versão com a forma especificada (`0.3.0-nightly.<data>.<sha>`).
5. Substitui a versão antiga no arquivo `package.json`.
6. Grava os dados atualizados de volta em `package.json`.

É importante notar que essa implementação assume que o arquivo `package.json` é sempre bem formado e contém uma única linha com a chave `"version"`.