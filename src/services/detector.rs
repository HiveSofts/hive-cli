use crate::models::project::Framework;

pub fn default_port(framework: &Framework) -> u16 {
    match framework {
        Framework::Laravel => 8000,
        Framework::NextJs => 3000,
        Framework::React => 5173,
        Framework::Vue => 5173,
        Framework::Nuxt => 3000,
        Framework::Node => 3000,
        Framework::Python => 8000,
        Framework::Django => 8000,
        Framework::FastAPI => 8000,
        Framework::Rust => 8080,
        Framework::Go => 8080,
        Framework::Unknown => 3000,
    }
}

pub fn default_run_command(framework: &Framework) -> &'static str {
    match framework {
        Framework::Laravel => "php artisan serve",
        Framework::NextJs => "npm run dev",
        Framework::React => "npm run dev",
        Framework::Vue => "npm run dev",
        Framework::Nuxt => "npm run dev",
        Framework::Node => "npm start",
        Framework::Python => "python main.py",
        Framework::Django => "python manage.py runserver",
        Framework::FastAPI => "uvicorn main:app --reload",
        Framework::Rust => "cargo run",
        Framework::Go => "go run .",
        Framework::Unknown => "echo 'No run command configured'",
    }
}

pub fn framework_icon(framework: &Framework) -> &'static str {
    match framework {
        Framework::Laravel => "🐘",
        Framework::NextJs => "▲",
        Framework::React => "⚛",
        Framework::Vue => "◈",
        Framework::Nuxt => "◆",
        Framework::Node => "⬡",
        Framework::Python => "🐍",
        Framework::Django => "🎸",
        Framework::FastAPI => "⚡",
        Framework::Rust => "🦀",
        Framework::Go => "🐹",
        Framework::Unknown => "📦",
    }
}
