#!/usr/bin/env bash
# gate.sh — the Sentri CI security gate.
#
# Runs the deterministic scan, writes a JSON report and a SARIF file (for
# GitHub code scanning), prints a one-line summary, and exits with the
# scan's own code so the PR check fails when findings at/above the fail-on
# threshold exist. Zero LLM cost, identical result every run.
#
# Env / args:
#   SENTRI_BIN   path to the sentri binary        (default: sentri)
#   SCAN_PATH    file/dir to scan                 (default: .)
#   SCAN_CHAIN   evm|solana|move|soroban          (default: evm)
#   FAIL_ON      low|medium|high|critical         (default: critical)
#   OUT_JSON     report path                      (default: sentri-report.json)
#   OUT_SARIF    sarif path                        (default: sentri.sarif)
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SENTRI_BIN="${SENTRI_BIN:-sentri}"
SCAN_PATH="${SCAN_PATH:-.}"
SCAN_CHAIN="${SCAN_CHAIN:-evm}"
FAIL_ON="${FAIL_ON:-critical}"
OUT_JSON="${OUT_JSON:-sentri-report.json}"
OUT_SARIF="${OUT_SARIF:-sentri.sarif}"

if ! command -v "$SENTRI_BIN" >/dev/null 2>&1 && [ ! -x "$SENTRI_BIN" ]; then
  echo "::error::sentri binary not found ('$SENTRI_BIN'). Build it first (cargo build --release --bin sentri) or set SENTRI_BIN." >&2
  exit 127
fi

# Run the scan. --fail-on makes the exit code the gate signal; capture it
# without aborting so we still emit SARIF for the Security tab.
scan_rc=0
"$SENTRI_BIN" scan "$SCAN_PATH" --chain "$SCAN_CHAIN" --output json --fail-on "$FAIL_ON" \
  > "$OUT_JSON" 2>/dev/null || scan_rc=$?

# Always convert to SARIF (valid even on an empty/failed report).
python3 "$HERE/to_sarif.py" "$OUT_JSON" "$OUT_SARIF" || true

# One-line summary from the report.
python3 - "$OUT_JSON" <<'PY' || true
import json, sys
try:
    d = json.load(open(sys.argv[1]))
    s = d.get("summary", {})
    print("::notice::Sentri scan — %d violations (critical %d, high %d, medium %d, low %d) on %s"
          % (s.get("violations", 0), s.get("critical", 0), s.get("high", 0),
             s.get("medium", 0), s.get("low", 0), d.get("chain", "?")))
except Exception:
    print("::notice::Sentri scan — no parseable report (target may have no in-scope files)")
PY

if [ "$scan_rc" -ne 0 ]; then
  echo "::error::Sentri gate FAILED — findings at or above '$FAIL_ON'. See the Security tab / $OUT_SARIF." >&2
fi
exit "$scan_rc"
