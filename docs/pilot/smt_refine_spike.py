"""Phase -1b / REQ-REFINE spike: static SMT proofs for pilot buckets 1 and 6.

Discharges RESEARCH_REPORT risk R1 / SPEC.md §4.4 REQ-REFINE using z3-solver
(Python bindings). No program under test is executed — proofs are pure SMT.

Bucket 1 (CWE-20): apply_discount(price, pct) = price * (1 - pct/100)
  Prove:  price >= 0 /\\ 0 <= pct <= 100  ==>  result >= 0
  And:    without the pct bound, a counterexample exists (pct > 100).

Bucket 6 (CWE-787): read_at(buffer, offset)
  Prove:  0 <= offset < len  ==>  access is in-bounds
  And:    without the bound, a counterexample exists (negative / OOB offset).

Exit 0 iff all four checks hold (2 proves + 2 counterexamples found).
"""
from __future__ import annotations

import sys
from dataclasses import dataclass

from z3 import And, Ints, Not, Reals, Solver, sat, unsat


@dataclass(frozen=True)
class Check:
    name: str
    ok: bool
    detail: str


def check_bucket1_proved() -> Check:
    """Static proof: under contracts, result is always non-negative."""
    price, pct = Reals("price pct")
    result = price * (1 - pct / 100)
    pre = And(price >= 0, pct >= 0, pct <= 100)
    post = result >= 0
    s = Solver()
    s.add(pre)
    s.add(Not(post))
    status = s.check()
    if status == unsat:
        return Check(
            "b1_proved",
            True,
            "unsat: under price>=0 and 0<=pct<=100, result>=0 holds for all inputs",
        )
    return Check("b1_proved", False, f"expected unsat, got {status}: {s.model() if status == sat else ''}")


def check_bucket1_counterexample() -> Check:
    """Without the pct bound, SMT finds a silent-bug input (pct > 100)."""
    price, pct = Reals("price pct")
    result = price * (1 - pct / 100)
    s = Solver()
    s.add(price >= 0)
    s.add(Not(result >= 0))  # looking for a negative result
    status = s.check()
    if status == sat:
        m = s.model()
        return Check(
            "b1_counterexample",
            True,
            f"sat counterexample (no pct bound): price={m[price]}, pct={m[pct]}, "
            f"result would be negative — the silent CWE-20 bug",
        )
    return Check("b1_counterexample", False, f"expected sat, got {status}")


def check_bucket6_proved() -> Check:
    """Static proof: under 0 <= offset < len, the access is in-bounds."""
    length, offset = Ints("length offset")
    pre = And(length > 0, offset >= 0, offset < length)
    in_bounds = And(offset >= 0, offset < length)
    s = Solver()
    s.add(pre)
    s.add(Not(in_bounds))
    status = s.check()
    if status == unsat:
        return Check(
            "b6_proved",
            True,
            "unsat: under 0<=offset<length, in-bounds holds for all inputs",
        )
    return Check("b6_proved", False, f"expected unsat, got {status}")


def check_bucket6_counterexample() -> Check:
    """Without the bound, SMT finds an out-of-range offset."""
    length, offset = Ints("length offset")
    s = Solver()
    s.add(length > 0)
    s.add(Not(And(offset >= 0, offset < length)))
    status = s.check()
    if status == sat:
        m = s.model()
        return Check(
            "b6_counterexample",
            True,
            f"sat counterexample (no offset bound): length={m[length]}, "
            f"offset={m[offset]} — the silent CWE-787 class",
        )
    return Check("b6_counterexample", False, f"expected sat, got {status}")


def main() -> int:
    checks = [
        check_bucket1_proved(),
        check_bucket1_counterexample(),
        check_bucket6_proved(),
        check_bucket6_counterexample(),
    ]
    print(f"z3-solver in use; {len(checks)} checks")
    all_ok = True
    for c in checks:
        mark = "PASS" if c.ok else "FAIL"
        print(f"[{mark}] {c.name}: {c.detail}")
        all_ok = all_ok and c.ok
    print("VERDICT:", "PASS — REQ-REFINE discharged for buckets 1 and 6" if all_ok else "FAIL")
    return 0 if all_ok else 1


if __name__ == "__main__":
    sys.exit(main())
