"""Nexora AI Engine - LLM integration for AI-native development."""

import os
from enum import Enum
from typing import Optional
from dataclasses import dataclass

import httpx


class LLMProvider(str, Enum):
    OPENAI = "openai"
    ANTHROPIC = "anthropic"
    OLLAMA = "ollama"
    LOCAL = "local"


@dataclass
class LLMConfig:
    provider: LLMProvider
    model: str
    api_key: Optional[str] = None
    base_url: Optional[str] = None
    temperature: float = 0.7
    max_tokens: int = 4096


class LLMClient:
    """Client for interacting with LLMs."""

    def __init__(self, config: LLMConfig):
        self.config = config
        self.http_client = httpx.AsyncClient(timeout=60.0)

    async def complete(
        self,
        prompt: str,
        system_prompt: Optional[str] = None,
        temperature: Optional[float] = None,
        max_tokens: Optional[int] = None,
    ) -> str:
        """Send a completion request to the LLM."""
        temp = temperature or self.config.temperature
        tokens = max_tokens or self.config.max_tokens

        if self.config.provider == LLMProvider.OPENAI:
            return await self._openai_complete(prompt, system_prompt, temp, tokens)
        elif self.config.provider == LLMProvider.ANTHROPIC:
            return await self._anthropic_complete(prompt, system_prompt, temp, tokens)
        elif self.config.provider == LLMProvider.OLLAMA:
            return await self._ollama_complete(prompt, system_prompt, temp, tokens)
        else:
            return await self._local_complete(prompt, system_prompt, temp, tokens)

    async def _openai_complete(
        self, prompt: str, system_prompt: Optional[str], temperature: float, max_tokens: int
    ) -> str:
        api_key = self.config.api_key or os.getenv("OPENAI_API_KEY")
        base_url = self.config.base_url or "https://api.openai.com/v1"

        messages = []
        if system_prompt:
            messages.append({"role": "system", "content": system_prompt})
        messages.append({"role": "user", "content": prompt})

        response = await self.http_client.post(
            f"{base_url}/chat/completions",
            headers={"Authorization": f"Bearer {api_key}"},
            json={
                "model": self.config.model,
                "messages": messages,
                "temperature": temperature,
                "max_tokens": max_tokens,
            },
        )
        response.raise_for_status()
        data = response.json()
        return data["choices"][0]["message"]["content"]

    async def _anthropic_complete(
        self, prompt: str, system_prompt: Optional[str], temperature: float, max_tokens: int
    ) -> str:
        api_key = self.config.api_key or os.getenv("ANTHROPIC_API_KEY")

        headers = {
            "x-api-key": api_key,
            "anthropic-version": "2023-06-01",
            "content-type": "application/json",
        }

        body = {
            "model": self.config.model,
            "max_tokens": max_tokens,
            "temperature": temperature,
            "messages": [{"role": "user", "content": prompt}],
        }

        if system_prompt:
            body["system"] = system_prompt

        response = await self.http_client.post(
            "https://api.anthropic.com/v1/messages",
            headers=headers,
            json=body,
        )
        response.raise_for_status()
        data = response.json()
        return data["content"][0]["text"]

    async def _ollama_complete(
        self, prompt: str, system_prompt: Optional[str], temperature: float, max_tokens: int
    ) -> str:
        base_url = self.config.base_url or "http://localhost:11434"

        messages = []
        if system_prompt:
            messages.append({"role": "system", "content": system_prompt})
        messages.append({"role": "user", "content": prompt})

        response = await self.http_client.post(
            f"{base_url}/api/chat",
            json={
                "model": self.config.model,
                "messages": messages,
                "stream": False,
            },
        )
        response.raise_for_status()
        data = response.json()
        return data["message"]["content"]

    async def _local_complete(
        self, prompt: str, system_prompt: Optional[str], temperature: float, max_tokens: int
    ) -> str:
        # Local model implementation
        return "Local model not implemented yet"

    async def close(self):
        await self.http_client.aclose()


class AIEngine:
    """Main AI engine for Nexora."""

    def __init__(self, config: Optional[LLMConfig] = None):
        if config is None:
            # Try to detect provider from environment
            if os.getenv("OPENAI_API_KEY"):
                config = LLMConfig(
                    provider=LLMProvider.OPENAI,
                    model="gpt-4",
                )
            elif os.getenv("ANTHROPIC_API_KEY"):
                config = LLMConfig(
                    provider=LLMProvider.ANTHROPIC,
                    model="claude-3-opus-20240229",
                )
            else:
                config = LLMConfig(
                    provider=LLMProvider.LOCAL,
                    model="local",
                )

        self.config = config
        self.client = LLMClient(config)

    async def generate_code(self, prompt: str, context: Optional[str] = None) -> str:
        """Generate code from a natural language prompt."""
        system_prompt = """You are an expert Nexora programmer. Generate clean, idiomatic Nexora code.
        
Nexora syntax example:
```
func greet(name: String) -> String {
    return "Hello, " + name + "!"
}

let age = 20
if age >= 18 {
    print "Adult"
}

for i in [1, 2, 3] {
    print i
}
```

Rules:
- Use clear, descriptive variable names
- Add type annotations where helpful
- Follow Nexora conventions
- Generate complete, runnable code
"""

        if context:
            system_prompt += f"\n\nContext:\n{context}"

        return await self.client.complete(prompt, system_prompt)

    async def explain_code(self, code: str) -> str:
        """Explain code in simple language."""
        system_prompt = """You are a helpful programming tutor. Explain Nexora code in simple, clear language.
Break down complex concepts and explain what each part does."""

        prompt = f"Explain this Nexora code:\n\n```\n{code}\n```"

        return await self.client.complete(prompt, system_prompt)

    async def refactor_code(self, code: str) -> str:
        """Refactor code for better structure."""
        system_prompt = """You are an expert code refactorer. Improve code structure, readability, and maintainability.
Provide the refactored code with explanations of changes."""

        prompt = f"Refactor this Nexora code for better quality:\n\n```\n{code}\n```"

        return await self.client.complete(prompt, system_prompt)

    async def fix_code(self, code: str, error: str) -> str:
        """Fix code based on an error message."""
        system_prompt = """You are an expert debugger. Fix the code based on the error.
Provide the fixed code with explanations."""

        prompt = f"""Fix this Nexora code based on the error:

Code:
```
{code}
```

Error:
{error}
"""

        return await self.client.complete(prompt, system_prompt)

    async def generate_tests(self, code: str) -> str:
        """Generate test cases for code."""
        system_prompt = """You are an expert test writer. Generate comprehensive tests for Nexora code.
Use the Nexora testing framework:

```
import "testing"

test "test name"() {
    // Arrange
    // Act
    // Assert
    assert condition
}
```
"""

        prompt = f"Generate tests for this code:\n\n```\n{code}\n```"

        return await self.client.complete(prompt, system_prompt)

    async def generate_docs(self, code: str) -> str:
        """Generate documentation for code."""
        system_prompt = """You are a technical documentation writer. Generate clear, comprehensive documentation for Nexora code.
Include function descriptions, parameter explanations, return values, and examples."""

        prompt = f"Generate documentation for this code:\n\n```\n{code}\n```"

        return await self.client.complete(prompt, system_prompt)

    async def security_scan(self, code: str) -> str:
        """Scan code for security vulnerabilities."""
        system_prompt = """You are a security expert. Analyze Nexora code for security vulnerabilities.
Identify issues like:
- SQL injection
- XSS vulnerabilities
- Insecure dependencies
- Hardcoded secrets
- Unsafe operations

Provide specific line numbers and remediation suggestions."""

        prompt = f"Scan this Nexora code for security issues:\n\n```\n{code}\n```"

        return await self.client.complete(prompt, system_prompt)

    async def optimize_code(self, code: str) -> str:
        """Analyze code for performance optimizations."""
        system_prompt = """You are a performance optimization expert. Analyze Nexora code for performance issues.
Suggest specific optimizations with explanations of why they help."""

        prompt = f"Analyze this Nexora code for performance optimizations:\n\n```\n{code}\n```"

        return await self.client.complete(prompt, system_prompt)

    async def autocomplete(self, code: str, cursor_position: int) -> str:
        """Suggest code completion."""
        system_prompt = """You are a code completion assistant. Suggest the next line or block of code based on context.
Provide only the completion, not explanations."""

        prompt = f"""Complete this Nexora code:

```
{code[:cursor_position]}<cursor>{code[cursor_position:]}
```

Suggest the completion:"""

        return await self.client.complete(prompt, system_prompt, max_tokens=256)

    async def generate_from_description(self, description: str) -> str:
        """Generate a complete program from a description."""
        system_prompt = """You are an expert Nexora developer. Generate a complete, production-ready program from a description.
Include all necessary imports, functions, and error handling.
Make the code clean, well-structured, and following Nexora best practices."""

        prompt = f"Generate a complete Nexora program: {description}"

        return await self.client.complete(prompt, system_prompt, max_tokens=4096)

    async def close(self):
        await self.client.close()
