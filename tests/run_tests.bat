@echo off
REM Nexora Test Runner Script

echo Running Nexora Compiler Tests...
cargo test --package nexora-compiler

echo.
echo Running Nexora Runtime Tests...
cargo test --package nexora-runtime

echo.
echo All tests completed!
