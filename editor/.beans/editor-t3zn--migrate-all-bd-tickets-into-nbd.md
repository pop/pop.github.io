---
# editor-t3zn
title: Migrate all bd tickets into nbd
status: completed
type: task
priority: critical
created_at: 2026-03-11T17:36:30Z
updated_at: 2026-03-11T17:36:30Z
---

Port all 36 bd tickets (open + closed) from .beads/issues.jsonl into nbd. Use a bash script with jq to parse the JSONL. Priority mapping: bd 1→9, bd 2→7, bd 3→5, bd 4→3. After creating tickets, run a second pass to wire up dependencies using the bd→nbd ID map, then archive closed tickets with 'nbd update <id> --status done'.
