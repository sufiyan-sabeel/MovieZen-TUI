use moviezen_tui::tui::app::App;

#[cfg(not(target_os = "android"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct TerminalGuard;

fn restore_terminal() {
    let _ = crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::SetCursorStyle::DefaultUserShape,
        crossterm::cursor::Show,
        crossterm::event::DisableMouseCapture,
        crossterm::event::DisableFocusChange,
        crossterm::event::PopKeyboardEnhancementFlags,
        crossterm::terminal::LeaveAlternateScreen
    );
    let _ = crossterm::terminal::disable_raw_mode();
}

fn purge_stale_subtitles() {
    tokio::task::spawn_blocking(|| {
        let max_age = 24 * 60 * 60;
        let mut dirs = vec![
            moviezen_tui::service::resolve_subtitle_dir(),
            std::env::temp_dir().join("moviezen-tui/subs"),
        ];
        if moviezen_tui::updater::artifact::is_termux_environment()
            && let Some(home) = dirs::home_dir()
        {
            let android_storage = home.join("storage/downloads/moviezen_subs");
            if home.join("storage/downloads").exists() {
                dirs.push(android_storage);
            }
            // Also purge legacy upstream download dirs so stale files age out.
            let legacy_storage = home.join("storage/downloads/moviebox_subs");
            if legacy_storage.exists() {
                dirs.push(legacy_storage);
            }
        }

        for dir in dirs {
            if dir.exists()
                && let Ok(entries) = std::fs::read_dir(&dir)
            {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata()
                        && let Ok(modified) = metadata.modified()
                        && let Ok(elapsed) = modified.elapsed()
                        && elapsed.as_secs() > max_age
                    {
                        let _ = std::fs::remove_file(entry.path());
                    }
                }
            }
        }
    });
}

fn purge_stale_update_artifacts() {
    tokio::task::spawn_blocking(|| {
        if let Ok(current_exe) = std::env::current_exe() {
            moviezen_tui::updater::apply::cleanup_stale_update_artifacts(&current_exe);
        }
    });
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore_terminal();
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(pos) = args.iter().position(|a| a == "--proxy-for-vlc") {
        let target_url = args.get(pos + 1).cloned().unwrap_or_default();
        let headers_json = args
            .get(pos + 2)
            .cloned()
            .unwrap_or_else(|| "[]".to_string());
        let sub_url = args.get(pos + 3).cloned().filter(|s| !s.is_empty());
        let headers: Vec<(String, String)> =
            serde_json::from_str(&headers_json).unwrap_or_default();
        moviezen_tui::proxy::run_sidecar(target_url, headers, sub_url).await;
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("moviezen-tui {}", env!("CARGO_PKG_VERSION"));
        println!("MovieZen-TUI - terminal media experience for movies, TV shows, and anime.\n");
        println!("USAGE:");
        println!("    moviezen-tui [OPTIONS]\n");
        println!("OPTIONS:");
        println!("    -h, --help           Print help information");
        println!("    -v, -V, --version    Print version information\n");
        println!("ENVIRONMENT VARIABLES:");
        println!("    MOVIEZEN_LOG              Log level (off, error, warn, info, debug, trace)");
        println!("    MOVIEZEN_THEME            Theme name (e.g. mocha, cinema, ocean, etc.)");
        println!("    MOVIEZEN_PLAYER           Preferred player (auto, mpv, iina, vlc, android)");
        println!("    MOVIEZEN_MPV_PATH         Custom mpv binary path");
        println!("    MOVIEZEN_VLC_PATH         Custom vlc binary path");
        println!("    MOVIEZEN_IINA_PATH        Custom iina-cli binary path");
        println!("    MOVIEZEN_FOURKHDHUB_URL   Custom 4KHDHub base URL");
        println!("    MOVIEZEN_NO_IMAGE         Disable poster image queries (1/true)");
        println!("    MOVIEZEN_IMAGE_PROTOCOL   Force graphics protocol (kitty, sixel, iterm2, none)");
        println!("    MOVIEZEN_CELL_SIZE        Override terminal cell size as WxH (e.g. 10x20)");
        println!("    MOVIEZEN_IMAGE_CACHE      Override poster/image cache directory");
        println!("    MOVIEZEN_IMAGE_CACHE_SIZE Poster/image cache budget in MB (default 64)");
        println!();
        println!("Legacy MOVIEBOX_* variables are still accepted as aliases for these.");
        return Ok(());
    }
    if args
        .iter()
        .any(|arg| arg == "--version" || arg == "-v" || arg == "-V")
    {
        println!("moviezen-tui {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    moviezen_tui::logging::init();

    std::panic::set_hook(Box::new(|info| {
        let backtrace = std::backtrace::Backtrace::capture();
        log::error!("panic: {info}\nbacktrace:\n{backtrace}");
        moviezen_tui::logging::flush();
        restore_terminal();
        eprintln!("{info}\n{backtrace}");
    }));

    let stdout = std::io::stdout();
    let backend =
        ratatui_crossterm::CrosstermBackend::new(std::io::BufWriter::with_capacity(65536, stdout));
    let mut terminal = ratatui::Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;
    let _guard = TerminalGuard;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture,
        crossterm::event::EnableFocusChange
    )?;
    if !moviezen_tui::updater::artifact::is_termux_environment() {
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::event::PushKeyboardEnhancementFlags(
                crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | crossterm::event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        );
    }

    moviezen_tui::cache::clean_old_cache_background();
    purge_stale_subtitles();
    purge_stale_update_artifacts();

    let mut app = App::new();
    if let Err(err) = app.run(&mut terminal).await {
        log::error!("application error: {err}");
    }
    Ok(())
}
