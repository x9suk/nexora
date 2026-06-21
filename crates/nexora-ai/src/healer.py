"""Self-Healing Runtime for Nexora.

Detects runtime errors, suggests fixes, and applies safe automatic fixes.
"""

from enum import Enum
from typing import Optional, List, Dict, Any
from dataclasses import dataclass


class ErrorSeverity(str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class FixType(str, Enum):
    NULL_CHECK = "null_check"
    TYPE_CONVERSION = "type_conversion"
    BOUNDS_CHECK = "bounds_check"
    ERROR_HANDLING = "error_handling"
    SUGGESTION = "suggestion"


@dataclass
class RuntimeError:
    message: str
    line: int
    column: int
    severity: ErrorSeverity
    context: Optional[str] = None


@dataclass
class FixSuggestion:
    fix_type: FixType
    description: str
    original_code: str
    fixed_code: str
    confidence: float
    auto_applicable: bool


@dataclass
class HealResult:
    error: RuntimeError
    suggestions: List[FixSuggestion]
    auto_fixed: bool
    applied_fix: Optional[FixSuggestion] = None


class SelfHealingRuntime:
    """Runtime with automatic error detection and healing."""

    def __init__(self, auto_fix: bool = True):
        self.auto_fix = auto_fix
        self.error_history: List[RuntimeError] = []
        self.fix_history: List[FixSuggestion] = []

    def analyze_error(
        self,
        error: RuntimeError,
        code: str,
        context: Optional[Dict[str, Any]] = None,
    ) -> HealResult:
        """Analyze a runtime error and suggest fixes."""
        self.error_history.append(error)

        suggestions = self._generate_suggestions(error, code, context)

        auto_fixed = False
        applied_fix = None

        if self.auto_fix and suggestions:
            # Find the highest confidence auto-applicable fix
            for suggestion in suggestions:
                if suggestion.auto_applicable and suggestion.confidence >= 0.8:
                    auto_fixed = True
                    applied_fix = suggestion
                    self.fix_history.append(suggestion)
                    break

        return HealResult(
            error=error,
            suggestions=suggestions,
            auto_fixed=auto_fixed,
            applied_fix=applied_fix,
        )

    def _generate_suggestions(
        self,
        error: RuntimeError,
        code: str,
        context: Optional[Dict[str, Any]],
    ) -> List[FixSuggestion]:
        """Generate fix suggestions based on error type."""
        suggestions = []

        error_msg = error.message.lower()

        # Null reference errors
        if "null" in error_msg or "none" in error_msg:
            suggestions.extend(self._handle_null_error(error, code))

        # Type errors
        elif "type" in error_msg or "mismatch" in error_msg:
            suggestions.extend(self._handle_type_error(error, code))

        # Index errors
        elif "index" in error_msg or "out of bounds" in error_msg:
            suggestions.extend(self._handle_index_error(error, code))

        # Name errors
        elif "undefined" in error_msg or "not defined" in error_msg:
            suggestions.extend(self._handle_name_error(error, code))

        # Attribute errors
        elif "attribute" in error_msg or "property" in error_msg:
            suggestions.extend(self._handle_attribute_error(error, code))

        # Division errors
        elif "division" in error_msg or "zero" in error_msg:
            suggestions.extend(self._handle_division_error(error, code))

        return suggestions

    def _handle_null_error(
        self, error: RuntimeError, code: str
    ) -> List[FixSuggestion]:
        """Handle null reference errors."""
        suggestions = []

        # Suggest null check
        suggestions.append(
            FixSuggestion(
                fix_type=FixType.NULL_CHECK,
                description="Add null check before accessing property",
                original_code=code,
                fixed_code=self._add_null_check(code, error.line),
                confidence=0.9,
                auto_applicable=True,
            )
        )

        # Suggest optional chaining
        suggestions.append(
            FixSuggestion(
                fix_type=FixType.NULL_CHECK,
                description="Use optional chaining (?.)",
                original_code=code,
                fixed_code=self._add_optional_chaining(code, error.line),
                confidence=0.85,
                auto_applicable=True,
            )
        )

        return suggestions

    def _handle_type_error(
        self, error: RuntimeError, code: str
    ) -> List[FixSuggestion]:
        """Handle type errors."""
        suggestions = []

        # Suggest type conversion
        if "string" in error.message.lower():
            suggestions.append(
                FixSuggestion(
                    fix_type=FixType.TYPE_CONVERSION,
                    description="Convert to string using str()",
                    original_code=code,
                    fixed_code=self._add_type_conversion(code, error.line, "str"),
                    confidence=0.85,
                    auto_applicable=True,
                )
            )
        elif "integer" in error.message.lower() or "number" in error.message.lower():
            suggestions.append(
                FixSuggestion(
                    fix_type=FixType.TYPE_CONVERSION,
                    description="Convert to integer using parseInt()",
                    original_code=code,
                    fixed_code=self._add_type_conversion(code, error.line, "parseInt"),
                    confidence=0.85,
                    auto_applicable=True,
                )
            )

        return suggestions

    def _handle_index_error(
        self, error: RuntimeError, code: str
    ) -> List[FixSuggestion]:
        """Handle index out of bounds errors."""
        suggestions = []

        # Suggest bounds check
        suggestions.append(
            FixSuggestion(
                fix_type=FixType.BOUNDS_CHECK,
                description="Add bounds check before indexing",
                original_code=code,
                fixed_code=self._add_bounds_check(code, error.line),
                confidence=0.9,
                auto_applicable=True,
            )
        )

        # Suggest try-catch
        suggestions.append(
            FixSuggestion(
                fix_type=FixType.ERROR_HANDLING,
                description="Wrap in try-catch block",
                original_code=code,
                fixed_code=self._wrap_in_try_catch(code, error.line),
                confidence=0.8,
                auto_applicable=False,
            )
        )

        return suggestions

    def _handle_name_error(
        self, error: RuntimeError, code: str
    ) -> List[FixSuggestion]:
        """Handle undefined variable errors."""
        suggestions = []

        # Extract variable name from error
        var_name = self._extract_variable_name(error.message)

        if var_name:
            # Suggest variable declaration
            suggestions.append(
                FixSuggestion(
                    fix_type=FixType.SUGGESTION,
                    description=f"Declare variable '{var_name}' before use",
                    original_code=code,
                    fixed_code=self._add_variable_declaration(
                        code, error.line, var_name
                    ),
                    confidence=0.7,
                    auto_applicable=False,
                )
            )

            # Suggest typo correction
            similar = self._find_similar_variable(var_name, code)
            if similar:
                suggestions.append(
                    FixSuggestion(
                        fix_type=FixType.SUGGESTION,
                        description=f"Did you mean '{similar}'?",
                        original_code=code,
                        fixed_code=code.replace(var_name, similar),
                        confidence=0.75,
                        auto_applicable=False,
                    )
                )

        return suggestions

    def _handle_attribute_error(
        self, error: RuntimeError, code: str
    ) -> List[FixSuggestion]:
        """Handle attribute/property errors."""
        suggestions = []

        # Extract property name
        prop_name = self._extract_property_name(error.message)

        if prop_name:
            suggestions.append(
                FixSuggestion(
                    fix_type=FixType.SUGGESTION,
                    description=f"Check if property '{prop_name}' exists",
                    original_code=code,
                    fixed_code=self._add_property_check(code, error.line, prop_name),
                    confidence=0.75,
                    auto_applicable=False,
                )
            )

        return suggestions

    def _handle_division_error(
        self, error: RuntimeError, code: str
    ) -> List[FixSuggestion]:
        """Handle division by zero errors."""
        suggestions = []

        # Suggest zero check
        suggestions.append(
            FixSuggestion(
                fix_type=FixType.BOUNDS_CHECK,
                description="Add zero check before division",
                original_code=code,
                fixed_code=self._add_zero_check(code, error.line),
                confidence=0.9,
                auto_applicable=True,
            )
        )

        return suggestions

    def _add_null_check(self, code: str, line: int) -> str:
        """Add null check to code."""
        lines = code.split("\n")
        if line < len(lines):
            target_line = lines[line]
            indent = len(target_line) - len(target_line.lstrip())
            spaces = " " * indent
            lines.insert(line, f"{spaces}if {self._extract_variable(target_line)} != null {{")
            lines.append(f"{spaces}}}")
        return "\n".join(lines)

    def _add_optional_chaining(self, code: str, line: int) -> str:
        """Add optional chaining to code."""
        lines = code.split("\n")
        if line < len(lines):
            lines[line] = lines[line].replace(".", "?.")
        return "\n".join(lines)

    def _add_type_conversion(self, code: str, line: int, converter: str) -> str:
        """Add type conversion to code."""
        lines = code.split("\n")
        if line < len(lines):
            lines[line] = f"{lines[line].rstrip()}  # Use {converter}() for type conversion"
        return "\n".join(lines)

    def _add_bounds_check(self, code: str, line: int) -> str:
        """Add bounds check to code."""
        lines = code.split("\n")
        if line < len(lines):
            target_line = lines[line]
            indent = len(target_line) - len(target_line.lstrip())
            spaces = " " * indent
            lines.insert(
                line,
                f"{spaces}if 0 <= index < len(array) {{",
            )
            lines.append(f"{spaces}}}")
        return "\n".join(lines)

    def _wrap_in_try_catch(self, code: str, line: int) -> str:
        """Wrap code in try-catch."""
        lines = code.split("\n")
        if line < len(lines):
            target_line = lines[line]
            indent = len(target_line) - len(target_line.lstrip())
            spaces = " " * indent
            lines.insert(line, f"{spaces}try {{")
            lines.append(f"{spaces}}} catch (e) {{")
            lines.append(f"{spaces}    print \"Error: \" + e")
            lines.append(f"{spaces}}}")
        return "\n".join(lines)

    def _add_variable_declaration(self, code: str, line: int, var_name: str) -> str:
        """Add variable declaration."""
        lines = code.split("\n")
        if line < len(lines):
            target_line = lines[line]
            indent = len(target_line) - len(target_line.lstrip())
            spaces = " " * indent
            lines.insert(line, f"{spaces}let {var_name} = null  # TODO: Initialize")
        return "\n".join(lines)

    def _add_property_check(self, code: str, line: int, prop_name: str) -> str:
        """Add property existence check."""
        lines = code.split("\n")
        if line < len(lines):
            target_line = lines[line]
            indent = len(target_line) - len(target_line.lstrip())
            spaces = " " * indent
            lines.insert(
                line,
                f'{spaces}if object != null && "{prop_name}" in object {{',
            )
            lines.append(f"{spaces}}}")
        return "\n".join(lines)

    def _add_zero_check(self, code: str, line: int) -> str:
        """Add zero check before division."""
        lines = code.split("\n")
        if line < len(lines):
            target_line = lines[line]
            indent = len(target_line) - len(target_line.lstrip())
            spaces = " " * indent
            lines.insert(line, f"{spaces}if divisor != 0 {{")
            lines.append(f"{spaces}}}")
        return "\n".join(lines)

    def _extract_variable(self, line: str) -> str:
        """Extract variable name from a line."""
        # Simple extraction - would need proper parsing
        parts = line.split()
        for i, part in enumerate(parts):
            if part in ["=", "+=", "-=", "*=", "/="] and i > 0:
                return parts[i - 1]
        return "variable"

    def _extract_variable_name(self, error_msg: str) -> Optional[str]:
        """Extract variable name from error message."""
        # Simple extraction
        if "'" in error_msg:
            start = error_msg.index("'") + 1
            end = error_msg.index("'", start)
            return error_msg[start:end]
        return None

    def _extract_property_name(self, error_msg: str) -> Optional[str]:
        """Extract property name from error message."""
        if "'" in error_msg:
            start = error_msg.index("'") + 1
            end = error_msg.index("'", start)
            return error_msg[start:end]
        return None

    def _find_similar_variable(self, name: str, code: str) -> Optional[str]:
        """Find similar variable name in code."""
        import re

        # Extract all identifiers
        identifiers = set(re.findall(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b", code))

        # Find similar names
        for identifier in identifiers:
            if identifier != name and self._levenshtein_distance(name, identifier) <= 2:
                return identifier
        return None

    def _levenshtein_distance(self, s1: str, s2: str) -> int:
        """Calculate Levenshtein distance between two strings."""
        if len(s1) < len(s2):
            return self._levenshtein_distance(s2, s1)

        if len(s2) == 0:
            return len(s1)

        previous_row = range(len(s2) + 1)
        for i, c1 in enumerate(s1):
            current_row = [i + 1]
            for j, c2 in enumerate(s2):
                insertions = previous_row[j + 1] + 1
                deletions = current_row[j] + 1
                substitutions = previous_row[j] + (c1 != c2)
                current_row.append(min(insertions, deletions, substitutions))
            previous_row = current_row

        return previous_row[-1]

    def get_healing_stats(self) -> Dict[str, Any]:
        """Get statistics about self-healing."""
        return {
            "total_errors": len(self.error_history),
            "total_fixes": len(self.fix_history),
            "fix_rate": (
                len(self.fix_history) / len(self.error_history)
                if self.error_history
                else 0
            ),
            "error_types": self._count_error_types(),
            "fix_types": self._count_fix_types(),
        }

    def _count_error_types(self) -> Dict[str, int]:
        """Count error types."""
        counts: Dict[str, int] = {}
        for error in self.error_history:
            severity = error.severity.value
            counts[severity] = counts.get(severity, 0) + 1
        return counts

    def _count_fix_types(self) -> Dict[str, int]:
        """Count fix types."""
        counts: Dict[str, int] = {}
        for fix in self.fix_history:
            fix_type = fix.fix_type.value
            counts[fix_type] = counts.get(fix_type, 0) + 1
        return counts
