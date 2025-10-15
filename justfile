# use powershell on Windows instead of `sh`
set windows-powershell

default:
  @just --list
  @just init

init:
  cargo install cargo-generate cargo-watch cargo-nextest

create year:
  cargo generate --name aoc-{{ year }} --git https://github.com/renan-z/advent-of-code.git --branch template
  @echo "✅ AOC {{ year }} created. Creating first day..."
  @cd aoc-{{ year }} && just create 01
  @echo "ℹ️ Start develop solution by run 'just work 01 1'"