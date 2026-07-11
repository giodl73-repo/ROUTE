from __future__ import annotations

import csv
import tempfile
import unittest
import zipfile
from pathlib import Path

import prepare_i80_report_sources as sources


class SourceReadinessTests(unittest.TestCase):
    def test_artifact_evidence_counts_csv_records(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "source.csv"
            path.write_text("id,value\n1,a\n2,b\n", encoding="utf-8")
            self.assertEqual(sources.artifact_evidence(path), (2, "csv records=2"))

    def test_artifact_evidence_rejects_invalid_zip(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "source.zip"
            path.write_text("not a zip", encoding="utf-8")
            count, detail = sources.artifact_evidence(path)
            self.assertEqual(count, 0)
            self.assertIn("invalid zip", detail)

    def test_readiness_keeps_unwired_claim_blocked(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            original_root = sources.ROOT
            try:
                sources.ROOT = Path(directory)
                artifact = sources.ROOT / "data" / "cache" / "faf.csv"
                artifact.parent.mkdir(parents=True)
                with artifact.open("w", encoding="utf-8", newline="") as handle:
                    writer = csv.writer(handle)
                    writer.writerow(["id"])
                    writer.writerow(["1"])
                contract = [
                    {
                        "source_id": "SRC-I80-FAF5",
                        "artifact": "data/cache/faf.csv",
                        "current_source_year": "2022",
                        "acquisition_status": "claim-reference-unwired",
                        "blocking_gap": "join missing",
                        "next_action": "wire join",
                    }
                ]
                row = sources.readiness_rows(contract, {})[0]
                self.assertEqual(row["readiness_status"], "blocked")
            finally:
                sources.ROOT = original_root

    def test_readiness_keeps_unresolved_adapter_blocked(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            original_root = sources.ROOT
            try:
                sources.ROOT = Path(directory)
                artifact = sources.ROOT / "data" / "cache" / "adapter.csv"
                artifact.parent.mkdir(parents=True)
                artifact.write_text("id\n1\n", encoding="utf-8")
                contract = [
                    {
                        "source_id": "SRC-I80-ADAPTER",
                        "artifact": "data/cache/adapter.csv",
                        "current_source_year": "",
                        "acquisition_status": "adapter-missing",
                        "blocking_gap": "adapter missing",
                        "next_action": "implement adapter",
                    }
                ]
                row = sources.readiness_rows(contract, {})[0]
                self.assertEqual(row["readiness_status"], "blocked")
            finally:
                sources.ROOT = original_root

    def test_excluded_source_does_not_fail_gate(self) -> None:
        rows = [
            {
                "source_id": "SRC-I80-EXCLUDED",
                "readiness_status": "excluded",
            }
        ]
        self.assertEqual(sources.gate(rows, {"SRC-I80-EXCLUDED"}), [])

    def test_failed_current_attempt_blocks_stale_cache(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            original_root = sources.ROOT
            try:
                sources.ROOT = Path(directory)
                artifact = sources.ROOT / "data" / "cache" / "source.csv"
                artifact.parent.mkdir(parents=True)
                artifact.write_text("id\n1\n", encoding="utf-8")
                contract = [
                    {
                        "source_id": "SRC-I80-TEST",
                        "artifact": "data/cache/source.csv",
                        "current_source_year": "2022",
                        "acquisition_status": "automated",
                        "blocking_gap": "fetch failed",
                        "next_action": "retry",
                    }
                ]
                row = sources.readiness_rows(
                    contract, {"SRC-I80-TEST": sources.Attempt("failed", "boom")}
                )[0]
                self.assertEqual(row["readiness_status"], "blocked")
                self.assertIn("current attempt failed", row["evidence_detail"])
            finally:
                sources.ROOT = original_root

    def test_hpms_i80_gate_requires_all_corridor_states(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "hpms.csv"
            with path.open("w", encoding="utf-8", newline="") as handle:
                writer = csv.writer(handle)
                writer.writerow(["STATE", "ROUTE_ID", "AADT"])
                for state in sorted(sources.I80_STATE_SET):
                    writer.writerow([state, "I80", "10000"])
            ready, count, detail = sources.hpms_i80_evidence(path)
            self.assertTrue(ready)
            self.assertEqual(count, len(sources.I80_STATE_SET))
            self.assertIn("11/11", detail)

    def test_fema_gate_requires_i80_named_positive_tiles(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "fema.csv"
            with path.open("w", encoding="utf-8", newline="") as handle:
                writer = csv.writer(handle)
                writer.writerow(
                    ["tile", "xmin", "ymin", "xmax", "ymax", "sfha_count"]
                )
                writer.writerow(["NJ-coast", "-74.5", "39.5", "-73.5", "40.5", "4"])
            ready, count, detail = sources.fema_i80_evidence(path)
            self.assertFalse(ready)
            self.assertEqual(count, 0)
            self.assertIn("I80 tiles=0/49", detail)

    def test_rucc_normalization_selects_code_rows(self) -> None:
        content = """FIPS,State,County_Name,Attribute,Value
01001,AL,Autauga County,Population_2020,58805
01001,AL,Autauga County,RUCC_2023,2
01001,AL,Autauga County,Description,Metro
"""
        self.assertEqual(
            sources.normalize_rucc(content),
            [{"GEOID": "01001", "RUCC": "2", "POP": "58805", "DENSITY": ""}],
        )

    def test_console_output_replaces_unencodable_characters(self) -> None:
        original = sources.sys.stdout

        class AsciiStdout:
            encoding = "ascii"

        try:
            sources.sys.stdout = AsciiStdout()
            self.assertEqual(sources.console_safe("saved → file"), "saved ? file")
        finally:
            sources.sys.stdout = original


if __name__ == "__main__":
    unittest.main()
