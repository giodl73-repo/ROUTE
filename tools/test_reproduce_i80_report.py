from __future__ import annotations

import unittest
import tempfile
from pathlib import Path

import reproduce_i80_report as reproduce


CURRENT = """## Key Facts

| Fact | Value | Source |
|---|---|---|
| Total miles | 10 | source |

## Dimension Scores

| Band | Dim | Name | Score | Quality | Confidence | Justification |
|---|---|---|---|---|---|---|
| A | A1 | Throughput | 1.0 | Low | 0.50 | current |

**Band totals**: 1
**Confidence**: 0.5
"""

REGENERATED = CURRENT.replace("| 10 |", "| 11 |").replace("current", "updated")


class ReproductionComparisonTests(unittest.TestCase):
    def test_comparison_detects_changed_fields(self) -> None:
        rows = reproduce.comparison_rows(CURRENT, REGENERATED)
        changed = {row["field"] for row in rows if row["changed"] == "true"}
        self.assertIn("fact:Total miles", changed)
        self.assertIn("dimension:A1:justification", changed)

    def test_comparison_keeps_unchanged_score(self) -> None:
        rows = reproduce.comparison_rows(CURRENT, REGENERATED)
        score = next(row for row in rows if row["field"] == "dimension:A1:score")
        self.assertEqual(score["changed"], "false")

    def test_failure_cleanup_removes_stale_outputs(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            original_regenerated = reproduce.REGENERATED
            original_comparison = reproduce.COMPARISON
            try:
                reproduce.REGENERATED = Path(directory) / "regenerated.md"
                reproduce.COMPARISON = Path(directory) / "comparison.csv"
                reproduce.REGENERATED.write_text("stale", encoding="utf-8")
                reproduce.COMPARISON.write_text("stale", encoding="utf-8")
                reproduce.clear_generated_outputs()
                self.assertFalse(reproduce.REGENERATED.exists())
                self.assertFalse(reproduce.COMPARISON.exists())
            finally:
                reproduce.REGENERATED = original_regenerated
                reproduce.COMPARISON = original_comparison

    def test_full_document_hash_detects_unparsed_changes(self) -> None:
        rows = reproduce.comparison_rows(CURRENT, REGENERATED + "\nExtra narrative.\n")
        digest = next(row for row in rows if row["field"] == "document:sha256")
        self.assertEqual(digest["changed"], "true")


if __name__ == "__main__":
    unittest.main()
