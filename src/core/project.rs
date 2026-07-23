use crate::models::project::{Framework, Project, ProjectMeta};
use std::path::{Path, PathBuf};

pub fn detect(path: &Path) -> Project {
    let framework = detect_framework(path);
    let meta = extract_meta(path, &framework);
    let name = meta.name.clone();
    Project {
        name,
        framework,
        path: path.to_path_buf(),
        meta,
    }
}

fn detect_framework(path: &Path) -> Framework {
    if path.join("artisan").exists() {
        return Framework::Laravel;
    }
    if path.join("composer.json").exists() {
        if read(path, "composer.json").contains("laravel/framework") {
            return Framework::Laravel;
        }
    }
    if path.join("package.json").exists() {
        let pkg = read(path, "package.json");
        if pkg.contains("\"next\"") {
            return Framework::NextJs;
        }
        if pkg.contains("\"nuxt\"") {
            return Framework::Nuxt;
        }
        if pkg.contains("\"vue\"") {
            return Framework::Vue;
        }
        if pkg.contains("\"react\"") {
            return Framework::React;
        }
        return Framework::Node;
    }
    if path.join("manage.py").exists() {
        return Framework::Django;
    }
    if path.join("requirements.txt").exists() {
        let req = read(path, "requirements.txt");
        if req.contains("fastapi") {
            return Framework::FastAPI;
        }
        if req.contains("django") {
            return Framework::Django;
        }
        return Framework::Python;
    }
    if path.join("pyproject.toml").exists() {
        let pyp = read(path, "pyproject.toml");
        if pyp.contains("fastapi") {
            return Framework::FastAPI;
        }
        if pyp.contains("django") {
            return Framework::Django;
        }
        return Framework::Python;
    }
    if path.join("Cargo.toml").exists() {
        return Framework::Rust;
    }
    if path.join("go.mod").exists() {
        return Framework::Go;
    }
    Framework::Unknown
}

fn extract_meta(path: &Path, framework: &Framework) -> ProjectMeta {
    let has_docker = path.join("Dockerfile").exists()
        || path.join("docker-compose.yml").exists()
        || path.join("docker-compose.yaml").exists();
    let has_env = path.join(".env").exists() || path.join(".env.example").exists();
    let extra_services = detect_extra_services(path);

    let (name, version, description) = match framework {
        Framework::Laravel => extract_from_composer(path),
        Framework::NextJs
        | Framework::React
        | Framework::Vue
        | Framework::Nuxt
        | Framework::Node => extract_from_package_json(path),
        Framework::Python | Framework::Django | Framework::FastAPI => extract_from_pyproject(path),
        Framework::Rust => extract_from_cargo(path),
        Framework::Go => extract_from_go_mod(path),
        Framework::Unknown => (dir_name(path), None, None),
    };

    ProjectMeta {
        name,
        version,
        description,
        has_docker,
        has_env,
        extra_services,
    }
}

fn detect_extra_services(path: &Path) -> Vec<String> {
    let mut services = vec![];
    let compose = ["docker-compose.yml", "docker-compose.yaml"]
        .iter()
        .find_map(|f| {
            let p = path.join(f);
            if p.exists() {
                Some(read(path, f))
            } else {
                None
            }
        })
        .unwrap_or_default()
        .to_lowercase();

    if compose.contains("mysql") || compose.contains("mariadb") {
        services.push("MySQL".into());
    }
    if compose.contains("postgres") {
        services.push("PostgreSQL".into());
    }
    if compose.contains("redis") {
        services.push("Redis".into());
    }
    if compose.contains("mongo") {
        services.push("MongoDB".into());
    }
    if compose.contains("nginx") {
        services.push("Nginx".into());
    }
    if compose.contains("rabbitmq") {
        services.push("RabbitMQ".into());
    }
    if compose.contains("elasticsearch") {
        services.push("Elasticsearch".into());
    }
    services
}

fn extract_from_package_json(path: &Path) -> (String, Option<String>, Option<String>) {
    let content = read(path, "package.json");
    let name = read_env_key(path, "APP_NAME")
        .or_else(|| json_str(&content, "name"))
        .unwrap_or_else(|| dir_name(path));
    let version = json_str(&content, "version");
    let description = json_str(&content, "description");
    (name, version, description)
}

fn extract_from_composer(path: &Path) -> (String, Option<String>, Option<String>) {
    let content = read(path, "composer.json");
    let name = read_env_key(path, "APP_NAME")
        .or_else(|| {
            json_str(&content, "name").map(|n| n.split('/').last().unwrap_or(&n).to_string())
        })
        .unwrap_or_else(|| dir_name(path));
    let description = json_str(&content, "description");
    (name, None, description)
}

fn extract_from_pyproject(path: &Path) -> (String, Option<String>, Option<String>) {
    if path.join("pyproject.toml").exists() {
        let content = read(path, "pyproject.toml");
        return (
            toml_str(&content, "name").unwrap_or_else(|| dir_name(path)),
            toml_str(&content, "version"),
            toml_str(&content, "description"),
        );
    }
    (dir_name(path), None, None)
}

fn extract_from_cargo(path: &Path) -> (String, Option<String>, Option<String>) {
    let content = read(path, "Cargo.toml");
    (
        toml_str(&content, "name").unwrap_or_else(|| dir_name(path)),
        toml_str(&content, "version"),
        toml_str(&content, "description"),
    )
}

fn extract_from_go_mod(path: &Path) -> (String, Option<String>, Option<String>) {
    let content = read(path, "go.mod");
    let name = content
        .lines()
        .find(|l| l.starts_with("module "))
        .map(|l| l["module ".len()..].trim().to_string())
        .map(|m| m.split('/').last().unwrap_or(&m).to_string())
        .unwrap_or_else(|| dir_name(path));
    (name, None, None)
}

fn read_env_key(path: &Path, key: &str) -> Option<String> {
    for file in &[".env", ".env.local", ".env.example"] {
        if let Ok(content) = std::fs::read_to_string(path.join(file)) {
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with('#') || line.is_empty() {
                    continue;
                }
                if let Some(rest) = line.strip_prefix(&format!("{}=", key)) {
                    let val = rest.trim().trim_matches('"').trim_matches('\'').to_string();
                    if !val.is_empty() {
                        return Some(val);
                    }
                }
            }
        }
    }
    None
}

fn json_str(content: &str, field: &str) -> Option<String> {
    let needle = format!("\"{}\"", field);
    let start = content.find(&needle)? + needle.len();
    let after_colon = content[start..].find(':')? + 1;
    let slice = &content[start + after_colon..].trim_start();
    if !slice.starts_with('"') {
        return None;
    }
    let inner = &slice[1..];
    let end = inner.find('"')?;
    let val = &inner[..end];
    if val.is_empty() {
        None
    } else {
        Some(val.to_string())
    }
}

fn toml_str(content: &str, field: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        let key_eq = format!("{} =", field);
        let key_eq2 = format!("{}=", field);
        if line.starts_with(&key_eq) || line.starts_with(&key_eq2) {
            let val = line
                .splitn(2, '=')
                .nth(1)?
                .trim()
                .trim_matches('"')
                .to_string();
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    None
}

fn read(path: &Path, file: &str) -> String {
    std::fs::read_to_string(path.join(file)).unwrap_or_default()
}

fn dir_name(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed")
        .to_string()
}

pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
