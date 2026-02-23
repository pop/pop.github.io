+++
title = "Delete .beads/ directory"
priority = 3
status = "todo"
ticket_type = "task"
dependencies = []
+++
Check if .beads/ is tracked in git ('git ls-files .beads/'). If tracked, use 'git rm -r .beads/'. If gitignored, use 'rm -rf .beads/'. This is the final cleanup step after all tickets are migrated.