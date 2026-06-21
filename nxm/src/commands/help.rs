use colored::*;

pub fn execute(command: Option<&str>) {
    match command {
        Some("init") => print_init_help(),
        Some("install") => print_install_help(),
        Some("publish") => print_publish_help(),
        Some("search") => print_search_help(),
        Some("run") => print_run_help(),
        Some("test") => print_test_help(),
        Some("update") => print_update_help(),
        Some("list") => print_list_help(),
        Some("remove") => print_remove_help(),
        Some(cmd) => {
            println!(
                "{}",
                format!("Unknown command: {}", cmd).red()
            );
            println!();
            print_general_help();
        }
        None => print_general_help(),
    }
}

fn print_general_help() {
    println!(
        "{}",
        "╔══════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║     Nexora Package Manager (nxm)         ║".cyan()
    );
    println!(
        "{}",
        "║     Version 1.0.0                        ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════╝".cyan()
    );
    println!();
    println!("Usage: nxm <command> [options]");
    println!();
    println!("Commands:");
    println!("  init                  Initialize a new project");
    println!("  install <pkg>         Install a package");
    println!("  publish               Publish to registry");
    println!("  search <query>        Search packages");
    println!("  run <script>          Run a script");
    println!("  test                  Run tests");
    println!("  update                Update packages");
    println!("  list                  List packages");
    println!("  remove <pkg>          Remove a package");
    println!("  version               Show version");
    println!("  help                  Show this help");
    println!();
    println!("Run 'nxm help <command>' for more information on a specific command.");
}

fn print_init_help() {
    println!("{}", "nxm init".cyan().bold());
    println!();
    println!("Initialize a new Nexora project by creating a nexora.json file.");
    println!();
    println!("Usage:");
    println!("  nxm init");
    println!("  nxm init --name my-project");
    println!();
    println!("Options:");
    println!("  -n, --name <name>    Package name (default: directory name)");
}

fn print_install_help() {
    println!("{}", "nxm install".cyan().bold());
    println!();
    println!("Install packages and their dependencies.");
    println!();
    println!("Usage:");
    println!("  nxm install              # Install all dependencies from nexora.json");
    println!("  nxm install <package>    # Install a specific package");
    println!("  nxm install <pkg> --save-dev  # Save as dev dependency");
    println!();
    println!("Options:");
    println!("  -d, --save-dev    Save as devDependency");
}

fn print_publish_help() {
    println!("{}", "nxm publish".cyan().bold());
    println!();
    println!("Publish a package to the Nexora registry.");
    println!();
    println!("Usage:");
    println!("  nxm publish");
    println!("  nxm publish --access restricted");
    println!();
    println!("Options:");
    println!("  -a, --access <level>    Access level: public (default) or restricted");
}

fn print_search_help() {
    println!("{}", "nxm search".cyan().bold());
    println!();
    println!("Search for packages in the Nexora registry.");
    println!();
    println!("Usage:");
    println!("  nxm search <query>");
    println!();
    println!("Examples:");
    println!("  nxm search math");
    println!("  nxm search utility functions");
}

fn print_run_help() {
    println!("{}", "nxm run".cyan().bold());
    println!();
    println!("Run a script defined in nexora.json.");
    println!();
    println!("Usage:");
    println!("  nxm run <script>");
    println!();
    println!("Examples:");
    println!("  nxm run dev");
    println!("  nxm run build");
    println!("  nxm run test");
}

fn print_test_help() {
    println!("{}", "nxm test".cyan().bold());
    println!();
    println!("Run tests for the current project.");
    println!();
    println!("Usage:");
    println!("  nxm test");
    println!("  nxm test --pattern \"*.test.nx\"");
}

fn print_update_help() {
    println!("{}", "nxm update".cyan().bold());
    println!();
    println!("Update packages to their latest versions.");
    println!();
    println!("Usage:");
    println!("  nxm update              # Update all packages");
    println!("  nxm update <package>    # Update a specific package");
}

fn print_list_help() {
    println!("{}", "nxm list".cyan().bold());
    println!();
    println!("List installed packages.");
    println!();
    println!("Usage:");
    println!("  nxm list              # List local packages");
    println!("  nxm list --global     # List global packages");
}

fn print_remove_help() {
    println!("{}", "nxm remove".cyan().bold());
    println!();
    println!("Remove a package from the project.");
    println!();
    println!("Usage:");
    println!("  nxm remove <package>");
    println!();
    println!("Examples:");
    println!("  nxm remove lodash");
    println!("  nxm remove testing");
}