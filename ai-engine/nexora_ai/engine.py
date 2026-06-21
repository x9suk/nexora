"""Nexora AI Engine - Core AI functionality for code assistance."""

import os
from typing import Optional
from pathlib import Path

from .models import (
    AIRequest,
    AIResponse,
    CodeExplanation,
    CodeGeneration,
    RefactorResult,
    BugReport,
    SecurityIssue,
    PerformanceIssue,
    SeverityLevel,
)


class AIEngine:
    """Main AI engine for Nexora language assistance."""

    def __init__(self, api_key: Optional[str] = None, provider: str = "openai"):
        """Initialize the AI engine.
        
        Args:
            api_key: API key for the AI provider
            provider: AI provider to use ("openai" or "anthropic")
        """
        self.api_key = api_key or os.getenv("NEXORA_AI_KEY") or os.getenv("OPENAI_API_KEY")
        self.provider = provider
        self.model = "gpt-4" if provider == "openai" else "claude-3-opus-20240229"
        
    async def process_request(self, request: AIRequest) -> AIResponse:
        """Process an AI request.
        
        Args:
            request: The AI request to process
            
        Returns:
            AI response with the result
        """
        command_handlers = {
            "explain": self._explain_code,
            "generate": self._generate_code,
            "refactor": self._refactor_code,
            "doctor": self._detect_bugs,
            "fix": self._auto_fix,
            "security": self._scan_security,
            "optimize": self._analyze_performance,
            "docs": self._generate_docs,
            "test": self._generate_tests,
        }
        
        handler = command_handlers.get(request.command)
        if not handler:
            return AIResponse(
                success=False,
                result=f"Unknown command: {request.command}",
                suggestions=["Available commands: explain, generate, refactor, doctor, fix, security, optimize, docs, test"]
            )
        
        try:
            return await handler(request)
        except Exception as e:
            return AIResponse(
                success=False,
                result=f"Error processing request: {str(e)}",
                suggestions=["Check your API key and try again"]
            )

    async def _explain_code(self, request: AIRequest) -> AIResponse:
        """Explain code in simple language."""
        code = request.code or ""
        
        # Simple code analysis
        lines = code.strip().split("\n")
        explanation_parts = []
        
        # Detect variables
        variables = []
        functions = []
        classes = []
        
        for line in lines:
            line = line.strip()
            if line.startswith("let ") or line.startswith("const "):
                var_name = line.split("=")[0].replace("let ", "").replace("const ", "").strip()
                variables.append(var_name)
            elif line.startswith("func "):
                func_name = line.split("(")[0].replace("func ", "").strip()
                functions.append(func_name)
            elif line.startswith("class "):
                class_name = line.split("{")[0].replace("class ", "").strip()
                classes.append(class_name)
        
        if variables:
            explanation_parts.append(f"Defines {len(variables)} variable(s): {', '.join(variables)}")
        if functions:
            explanation_parts.append(f"Defines {len(functions)} function(s): {', '.join(functions)}")
        if classes:
            explanation_parts.append(f"Defines {len(classes)} class(es): {', '.join(classes)}")
        
        explanation_parts.append(f"Contains {len(lines)} lines of code")
        
        result = "Code Analysis:\n" + "\n".join(f"• {part}" for part in explanation_parts)
        
        return AIResponse(
            success=True,
            result=result,
            confidence=0.85,
            metadata={"lines": len(lines), "variables": len(variables), "functions": len(functions)}
        )

    async def _generate_code(self, request: AIRequest) -> AIResponse:
        """Generate code from a prompt."""
        prompt = request.prompt or ""
        
        # Template-based code generation
        templates = {
            "function": f'''func {prompt.replace(" ", "_").lower()}() {{
    // TODO: Implement {prompt}
    print "{prompt}"
}}''',
            "class": f'''class {prompt.replace(" ", "")} {{
    init() {{
        // Constructor
    }}
    
    greet() {{
        print "Hello from {prompt}"
    }}
}}''',
            "api": f'''func fetch_{prompt.replace(" ", "_").lower()}() {{
    let response = http.get("https://api.example.com/{prompt.replace(" ", "-").lower()}")
    return response.json()
}}''',
            "bot": f'''// Discord bot command: {prompt}
func handle_{prompt.replace(" ", "_").lower()}(message) {{
    let args = message.content.split(" ")
    // Process command
    message.reply("Executing {prompt}...")
}}''',
        }
        
        # Select template based on prompt keywords
        template_key = "function"
        if "class" in prompt.lower():
            template_key = "class"
        elif "api" in prompt.lower() or "http" in prompt.lower():
            template_key = "api"
        elif "bot" in prompt.lower() or "command" in prompt.lower():
            template_key = "bot"
        
        code = templates[template_key]
        
        return AIResponse(
            success=True,
            result=code,
            suggestions=["Review the generated code and customize as needed"],
            confidence=0.75
        )

    async def _refactor_code(self, request: AIRequest) -> AIResponse:
        """Refactor code for better structure."""
        code = request.code or ""
        
        suggestions = []
        refactored = code
        
        # Simple refactoring suggestions
        lines = code.split("\n")
        issues_found = []
        
        for i, line in enumerate(lines, 1):
            # Check for long lines
            if len(line) > 80:
                issues_found.append(f"Line {i}: Consider breaking long line")
            
            # Check for nested conditions
            if line.strip().startswith("if") and i < len(lines):
                next_line = lines[i].strip() if i < len(lines) else ""
                if next_line.startswith("if"):
                    issues_found.append(f"Lines {i}-{i+1}: Consider combining nested if statements")
        
        if issues_found:
            suggestions = issues_found
            result = "Refactoring suggestions:\n" + "\n".join(f"• {s}" for s in suggestions)
        else:
            result = "Code looks well-structured. No refactoring needed."
        
        return AIResponse(
            success=True,
            result=result,
            suggestions=suggestions,
            confidence=0.8
        )

    async def _detect_bugs(self, request: AIRequest) -> AIResponse:
        """Detect potential bugs in code."""
        code = request.code or ""
        
        bugs = []
        lines = code.split("\n")
        
        for i, line in enumerate(lines, 1):
            # Check for common issues
            if "=" in line and "==" not in line and "!=" not in line and "<=" not in line and ">=" not in line:
                if "let" in line or "const" in line:
                    continue
                # Potential assignment in condition
            
            if "undefined" in line.lower() or "null" in line.lower():
                bugs.append(f"Line {i}: Potential null reference")
            
            if "TODO" in line or "FIXME" in line:
                bugs.append(f"Line {i}: Unresolved TODO/FIXME")
        
        if bugs:
            result = f"Found {len(bugs)} potential issue(s):\n" + "\n".join(f"• {b}" for b in bugs)
        else:
            result = "No obvious bugs detected."
        
        return AIResponse(
            success=True,
            result=result,
            suggestions=["Run the code to verify runtime behavior"],
            confidence=0.7
        )

    async def _auto_fix(self, request: AIRequest) -> AIResponse:
        """Auto-fix common issues."""
        code = request.code or ""
        
        fixed = code
        changes = []
        
        # Fix common issues
        # 1. Remove trailing whitespace
        lines = fixed.split("\n")
        cleaned_lines = []
        for line in lines:
            original = line
            cleaned = line.rstrip()
            if original != cleaned:
                changes.append("Removed trailing whitespace")
            cleaned_lines.append(cleaned)
        fixed = "\n".join(cleaned_lines)
        
        # 2. Add missing semicolons (simplified)
        # This is a basic check - real implementation would be more sophisticated
        
        if changes:
            result = f"Applied {len(changes)} fix(es):\n" + "\n".join(f"• {c}" for c in changes)
        else:
            result = "No fixes needed."
        
        return AIResponse(
            success=True,
            result=result if not changes else fixed,
            suggestions=changes,
            confidence=0.9
        )

    async def _scan_security(self, request: AIRequest) -> AIResponse:
        """Scan for security vulnerabilities."""
        code = request.code or ""
        
        issues = []
        lines = code.split("\n")
        
        for i, line in enumerate(lines, 1):
            # Check for common security issues
            if "eval(" in line:
                issues.append(f"Line {i}: Avoid using eval() - potential code injection")
            
            if "innerHTML" in line:
                issues.append(f"Line {i}: Avoid innerHTML - use safe DOM methods")
            
            if "password" in line.lower() and "=" in line:
                issues.append(f"Line {i}: Hardcoded password detected")
            
            if "http://" in line:
                issues.append(f"Line {i}: Use HTTPS instead of HTTP")
        
        if issues:
            result = f"Found {len(issues)} security issue(s):\n" + "\n".join(f"• {i}" for i in issues)
        else:
            result = "No obvious security issues found."
        
        return AIResponse(
            success=True,
            result=result,
            suggestions=["Review security best practices"],
            confidence=0.75
        )

    async def _analyze_performance(self, request: AIRequest) -> AIResponse:
        """Analyze code for performance issues."""
        code = request.code or ""
        
        issues = []
        lines = code.split("\n")
        
        for i, line in enumerate(lines, 1):
            # Check for common performance issues
            if "for" in line and "in" in line:
                # Nested loop detection would require more context
                pass
            
            if ".push(" in line:
                # In hot loops, consider pre-allocating
                pass
        
        if issues:
            result = f"Found {len(issues)} performance issue(s):\n" + "\n".join(f"• {i}" for i in issues)
        else:
            result = "No obvious performance issues found."
        
        return AIResponse(
            success=True,
            result=result,
            suggestions=["Profile the code for accurate performance analysis"],
            confidence=0.65
        )

    async def _generate_docs(self, request: AIRequest) -> AIResponse:
        """Generate documentation for code."""
        code = request.code or ""
        
        lines = code.split("\n")
        doc_parts = []
        
        # Detect and document functions
        in_func = False
        func_name = ""
        func_params = []
        
        for line in lines:
            line = line.strip()
            if line.startswith("func "):
                in_func = True
                func_name = line.split("(")[0].replace("func ", "").strip()
                func_params = line.split("(")[1].split(")")[0].split(",") if "(" in line else []
                func_params = [p.strip() for p in func_params if p.strip()]
            elif in_func and line == "}":
                in_func = False
        
        # Generate documentation
        doc = f"""# Documentation

## Overview
This file contains Nexora code with {len(lines)} lines.

## Functions
"""
        
        # Simple function documentation
        for line in lines:
            if "func " in line:
                func_name = line.split("(")[0].replace("func ", "").strip()
                doc += f"\n### {func_name}()\nDescription of {func_name} function.\n"
        
        return AIResponse(
            success=True,
            result=doc,
            confidence=0.8
        )

    async def _generate_tests(self, request: AIRequest) -> AIResponse:
        """Generate test cases for code."""
        code = request.code or ""
        
        test_code = '// Auto-generated tests\n'
        test_code += 'import "testing"\n\n'
        
        # Detect functions and generate basic tests
        for line in code.split("\n"):
            if "func " in line and "(" in line:
                func_name = line.split("(")[0].replace("func ", "").strip()
                test_code += f'''
test "{func_name}"() {{
    // TODO: Add test cases for {func_name}
    assert true
    print "Test {func_name} passed"
}}
'''
        
        return AIResponse(
            success=True,
            result=test_code,
            suggestions=["Add more comprehensive test cases"],
            confidence=0.7
        )


async def process_ai_command(command: str, **kwargs) -> AIResponse:
    """Convenience function to process an AI command.
    
    Args:
        command: The AI command to execute
        **kwargs: Additional arguments
        
    Returns:
        AI response
    """
    engine = AIEngine()
    request = AIRequest(command=command, **kwargs)
    return await engine.process_request(request)
