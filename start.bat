@echo off
:loop
Quantum_Bot.exe
timeout 5
if NOT %QUANTUM_BOT_STOP% == 1 goto loop