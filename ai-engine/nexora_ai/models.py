"""Data models for the Nexora AI Engine."""

from dataclasses import dataclass, field
from typing import Optional
from enum import Enum


class SeverityLevel(str, Enum):
    """Severity level for issues."""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class IssueType(str, Enum):
    """Type of code issue."""
    BUG = "bug"
    SECURITY = "security"
    PERFORMANCE = "performance"
    STYLE = "style"
    ERROR = "error"


@dataclass
class AIRequest:
    """Request sent to the AI engine."""
    command: str
    code: Optional[str] = None
    file_path: Optional[str] = None
    prompt: Optional[str] = None
    context: Optional[str] = None
    language: str = "nexora"


@dataclass
class AIResponse:
    """Response from the AI engine."""
    success: bool
    result: str
    suggestions: list[str] = field(default_factory=list)
    confidence: float = 0.0
    metadata: dict = field(default_factory=dict)


@dataclass
class CodeExplanation:
    """Explanation of code."""
    summary: str
    detailed: str
    components: list[str] = field(default_factory=list)
    dependencies: list[str] = field(default_factory=list)
    suggestions: list[str] = field(default_factory=list)


@dataclass
class CodeGeneration:
    """Generated code result."""
    code: str
    description: str
    imports: list[str] = field(default_factory=list)
    tests: Optional[str] = None
    documentation: Optional[str] = None


@dataclass
class RefactorResult:
    """Result of code refactoring."""
    original: str
    refactored: str
    changes: list[str] = field(default_factory=list)
    improvements: list[str] = field(default_factory=list)


@dataclass
class BugReport:
    """Detected bug report."""
    line: int
    column: int
    message: str
    severity: SeverityLevel
    suggestion: Optional[str] = None
    code_snippet: Optional[str] = None


@dataclass
class SecurityIssue:
    """Security vulnerability detected."""
    line: int
    column: int
    message: str
    severity: SeverityLevel
    vulnerability_type: str
    suggestion: Optional[str] = None
    cwe_id: Optional[str] = None


@dataclass
class PerformanceIssue:
    """Performance issue detected."""
    line: int
    column: int
    message: str
    severity: SeverityLevel
    impact: str
    suggestion: Optional[str] = None
    optimization: Optional[str] = None


@dataclass
class LintIssue:
    """Linting issue."""
    line: int
    column: int
    message: str
    severity: SeverityLevel
    rule: str
    auto_fixable: bool = False


@dataclass
class TestResult:
    """Test execution result."""
    name: str
    passed: bool
    message: Optional[str] = None
    duration: Optional[float] = None
    stack_trace: Optional[str] = None


@dataclass
class ProjectAnalysis:
    """Analysis of an entire project."""
    files_analyzed: int
    total_lines: int
    issues: list[BugReport] = field(default_factory=list)
    security_issues: list[SecurityIssue] = field(default_factory=list)
    performance_issues: list[PerformanceIssue] = field(default_factory=list)
    suggestions: list[str] = field(default_factory=list)
