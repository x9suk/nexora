"""CLI interface for the Nexora AI Engine."""

import asyncio
import sys
from pathlib import Path

import typer
from rich.console import Console
from rich.markdown import Markdown
from rich.panel import Panel

from nexora_ai.engine import AIEngine, process_ai_command
from nexora_ai.models import AIRequest

app = typer.Typer(
    name="nexora-ai",
    help="Nexora AI Engine - AI-powered code assistance",
)
console = Console()


@app.command()
def explain(file: Path):
    """Explain code in simple language."""
    if not file.exists():
        console.print(f"[red]Error:[/red] File not found: {file}")
        sys.exit(1)
    
    code = file.read_text()
    result = asyncio.run(process_ai_command("explain", code=code, file_path=str(file)))
    
    if result.success:
        console.print(Panel(result.result, title="Code Explanation", border_style="green"))
    else:
        console.print(f"[red]Error:[/red] {result.result}")


@app.command()
def generate(prompt: str):
    """Generate code from a natural language prompt."""
    result = asyncio.run(process_ai_command("generate", prompt=prompt))
    
    if result.success:
        console.print(Panel(result.result, title="Generated Code", border_style="green"))
        if result.suggestions:
            console.print("\n[yellow]Suggestions:[/yellow]")
            for suggestion in result.suggestions:
                console.print(f"  • {suggestion}")
    else:
        console.print(f"[red]Error:[/red] {result.result}")


@app.command()
def refactor(file: Path):
    """Refactor code for better structure."""
    if not file.exists():
        console.print(f"[red]Error:[/red] File not found: {file}")
        sys.exit(1)
    
    code = file.read_text()
    result = asyncio.run(process_ai_command("refactor", code=code, file_path=str(file)))
    
    if result.success:
        console.print(Panel(result.result, title="Refactoring Suggestions", border_style="green"))
    else:
        console.print(f"[red]Error:[/red] {result.result}")


@app.command()
def doctor(path: Path = Path(".")):
    """Scan project for issues."""
    if not path.exists():
        console.print(f"[red]Error:[/red] Path not found: {path}")
        sys.exit(1)
    
    console.print(f"[cyan]Scanning project at {path}...[/cyan]\n")
    
    # Scan .nx files
    nx_files = list(path.rglob("*.nx"))
    
    if not nx_files:
        console.print("[yellow]No .nx files found in the project.[/yellow]")
        return
    
    all_issues = []
    
    for file in nx_files:
        code = file.read_text()
        result = asyncio.run(process_ai_command("doctor", code=code, file_path=str(file)))
        
        if result.success and result.result != "No obvious bugs detected.":
            all_issues.append((file, result.result))
    
    if all_issues:
        console.print(f"[yellow]Found issues in {len(all_issues)} file(s):[/yellow]\n")
        for file, issues in all_issues:
            console.print(f"[bold]{file}:[/bold]")
            console.print(f"  {issues}\n")
    else:
        console.print("[green]No issues found![/green]")


@app.command()
def fix(path: Path = Path(".")):
    """Auto-fix common issues."""
    if not path.exists():
        console.print(f"[red]Error:[/red] Path not found: {path}")
        sys.exit(1)
    
    console.print(f"[cyan]Fixing issues in {path}...[/cyan]\n")
    
    nx_files = list(path.rglob("*.nx"))
    
    for file in nx_files:
        code = file.read_text()
        result = asyncio.run(process_ai_command("fix", code=code, file_path=str(file)))
        
        if result.success and result.result != code:
            file.write_text(result.result)
            console.print(f"[green]Fixed:[/green] {file}")
        else:
            console.print(f"[dim]No changes:[/dim] {file}")


@app.command()
def security(path: Path = Path(".")):
    """Scan for security vulnerabilities."""
    if not path.exists():
        console.print(f"[red]Error:[/red] Path not found: {path}")
        sys.exit(1)
    
    console.print(f"[cyan]Scanning for security issues in {path}...[/cyan]\n")
    
    nx_files = list(path.rglob("*.nx"))
    
    all_issues = []
    
    for file in nx_files:
        code = file.read_text()
        result = asyncio.run(process_ai_command("security", code=code, file_path=str(file)))
        
        if result.success and result.result != "No obvious security issues found.":
            all_issues.append((file, result.result))
    
    if all_issues:
        console.print(f"[red]Found security issues in {len(all_issues)} file(s):[/red]\n")
        for file, issues in all_issues:
            console.print(f"[bold]{file}:[/bold]")
            console.print(f"  {issues}\n")
    else:
        console.print("[green]No security issues found![/green]")


@app.command()
def optimize(path: Path = Path(".")):
    """Analyze for performance issues."""
    if not path.exists():
        console.print(f"[red]Error:[/red] Path not found: {path}")
        sys.exit(1)
    
    console.print(f"[cyan]Analyzing performance in {path}...[/cyan]\n")
    
    nx_files = list(path.rglob("*.nx"))
    
    all_issues = []
    
    for file in nx_files:
        code = file.read_text()
        result = asyncio.run(process_ai_command("optimize", code=code, file_path=str(file)))
        
        if result.success and result.result != "No obvious performance issues found.":
            all_issues.append((file, result.result))
    
    if all_issues:
        console.print(f"[yellow]Found performance issues in {len(all_issues)} file(s):[/yellow]\n")
        for file, issues in all_issues:
            console.print(f"[bold]{file}:[/bold]")
            console.print(f"  {issues}\n")
    else:
        console.print("[green]No performance issues found![/green]")


@app.command()
def docs(file: Path):
    """Generate documentation for code."""
    if not file.exists():
        console.print(f"[red]Error:[/red] File not found: {file}")
        sys.exit(1)
    
    code = file.read_text()
    result = asyncio.run(process_ai_command("docs", code=code, file_path=str(file)))
    
    if result.success:
        console.print(Panel(Markdown(result.result), title="Generated Documentation", border_style="green"))
    else:
        console.print(f"[red]Error:[/red] {result.result}")


@app.command()
def test(file: Path):
    """Generate test cases for code."""
    if not file.exists():
        console.print(f"[red]Error:[/red] File not found: {file}")
        sys.exit(1)
    
    code = file.read_text()
    result = asyncio.run(process_ai_command("test", code=code, file_path=str(file)))
    
    if result.success:
        console.print(Panel(result.result, title="Generated Tests", border_style="green"))
    else:
        console.print(f"[red]Error:[/red] {result.result}")


if __name__ == "__main__":
    app()
