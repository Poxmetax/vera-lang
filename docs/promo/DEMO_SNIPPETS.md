# VERA — Demo snippets (authentic)

Short, captioned excerpts from real examples / FixPatch docs.  
Sources under `examples/` unless noted.

---

## 1. Prove clamp (Z3 VC)

**Caption:** Return refinement proved under `requires lo <= hi` — `vera --prove examples/prove_clamp.vera`  
**Source:** `examples/prove_clamp.vera`

```vera
fn clamp(x: Int, lo: Int, hi: Int) -> {r: Int | r >= lo && r <= hi}
    requires lo <= hi
    ensures result >= lo
    ensures result <= hi
{
    if x < lo {
        lo
    } else {
        if x > hi {
            hi
        } else {
            x
        }
    }
}
```

**Honest note:** Needs Z3 on `PATH`. Without `--prove`, refinements still run as runtime checks.

---

## 2. Refine `len` measure (bounds in the type)

**Caption:** Index refined by `len(xs)` — in-range literal typechecks; OOB literal is a compile-time error  
**Source:** `examples/refine_len_ok.vera`

```vera
fn nth(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    match xs.get(i) {
        Some(v) => v,
        None => -1,
    }
}

fn main(console: Console) -> Unit
    uses {console}
{
    console.print(nth([10, 20, 30], 1).show());
}
```

---

## 3. Ephemeral FixPatch (non-exhaustive match)

**Caption:** Structured diagnostics can carry a machine-applicable FixPatch — always `ephemeral: true`  
**Source:** documented live output in `docs/pilot/P2E_FIXPATCH_SLICE.md` (demo file not in `examples/` — committed examples must typecheck)

```json
{
  "source": "typecheck",
  "severity": "error",
  "code": "TYPE-ERROR",
  "message": "non-exhaustive match on Signal: missing Sell, Hold",
  "span": { "line": 11, "col": 5 },
  "fix": {
    "kind": "add-match-arms",
    "ephemeral": true,
    "span": { "line": 11, "col": 5 },
    "missing": [ "Signal::Sell(_)", "Signal::Hold" ]
  }
}
```

**Honest note:** Not a durable proof cache. Consumers must not persist/replay against drifted code without INV-2 keying (GAP-D2).

---

## Optional fourth (Option propagation)

**Caption:** Postfix `?` on `Option` — Phase 1 surface, no SMT required  
**Source:** `examples/propagate.vera`

```vera
fn dig(xs: List<Int>) -> Option<Int> {
    let h: Int = xs.head()?;
    Some(h)
}
```
