from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import public_proof


class PublicProofTests(unittest.TestCase):
    def test_public_proof_executes_no_credential_sources_before_gating(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            packet = root / "docs" / "packets" / "i80-flagship-review-packet.md"
            packet.parent.mkdir(parents=True)
            packet.write_text("Decision: Hold and narrow\n", encoding="utf-8")

            with patch.object(public_proof, "ROOT", root), patch.object(
                public_proof, "run"
            ) as run:
                self.assertEqual(public_proof.main(), 0)

            self.assertEqual(run.call_count, 2)
            self.assertEqual(
                run.call_args_list[1].args,
                (
                    "I-80 no-credential source gate",
                    [
                        sys.executable,
                        "tools/prepare_i80_report_sources.py",
                        "--execute",
                        "--gate-no-credential",
                    ],
                ),
            )


if __name__ == "__main__":
    unittest.main()
