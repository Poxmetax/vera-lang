"""Surgical [P2-REFINE1-DEF] patcher for typecheck.rs."""
from __future__ import annotations
import argparse, hashlib, os, shutil, tempfile, time
from pathlib import Path

TARGET = Path("crates/vera/src/typecheck.rs")
EXPECTED = {"pre": "23845289d6cf21a239784575fccd6504"}
MARKER = "[P2-REFINE1-DEF]"
ARROW = "\u2192"

def md5_lf(b: bytes) -> str:
    return hashlib.md5(b.replace(b"\r\n", b"\n")).hexdigest()

def detect_eol(raw: bytes) -> str:
    return "\r\n" if b"\r\n" in raw[:4096] else "\n"

def atomic_write(path: Path, text: str, eol: str) -> None:
    payload = text.replace("\n", "\r\n").encode("utf-8") if eol == "\r\n" else text.encode("utf-8")
    fd, tmp = tempfile.mkstemp(dir=str(path.parent), prefix=".tmp_p2r1d_", suffix=".rs")
    try:
        with os.fdopen(fd, "wb") as f:
            f.write(payload)
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp, path)
    except Exception:
        if os.path.exists(tmp):
            os.remove(tmp)
        raise

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--rollback", action="store_true")
    ap.add_argument("--target", type=Path, default=TARGET)
    args = ap.parse_args()
    target = args.target

    if args.rollback:
        baks = sorted(target.parent.glob(target.name + ".bak_*_p2_refine1_def"))
        if not baks:
            print("FAIL: no backup")
            return 1
        shutil.copy2(baks[-1], target)
        print("restored", baks[-1])
        return 0

    raw = target.read_bytes()
    eol = detect_eol(raw)
    digest = md5_lf(raw)
    print("[*] Target:", target)
    print("[*] EOL:", "CRLF" if eol == "\r\n" else "LF")
    print("[*] Baseline MD5:", digest)

    text = raw.decode("utf-8")
    if eol == "\r\n":
        text = text.replace("\r\n", "\n")
    if "fn check_ret_refine_body" in text:
        print("[*] Already applied.")
        return 0
    if digest not in EXPECTED.values():
        print("FAIL: baseline mismatch", digest)
        return 1

    hook_old = (
        "    if !types_equal(&body_ty, &fn_decl.ret) {\n"
        "        return Err(TypeError::at(\n"
        "            fn_decl.span,\n"
        "            format!(\n"
        '                "function {}: body type {} != declared {}",\n'
        "                fn_decl.name,\n"
        "                body_ty.to_str(),\n"
        "                fn_decl.ret.to_str()\n"
        "            ),\n"
        "        ));\n"
        "    }\n"
        '    let ens_env = env.extend("result".into(), fn_decl.ret.clone());'
    )
    hook_new = (
        "    if !types_equal(&body_ty, &fn_decl.ret) {\n"
        "        return Err(TypeError::at(\n"
        "            fn_decl.span,\n"
        "            format!(\n"
        '                "function {}: body type {} != declared {}",\n'
        "                fn_decl.name,\n"
        "                body_ty.to_str(),\n"
        "                fn_decl.ret.to_str()\n"
        "            ),\n"
        "        ));\n"
        "    }\n"
        "    // [P2-REFINE1-DEF] hard reject when closed body falsifies return refine\n"
        "    check_ret_refine_body(fn_decl)?;\n"
        '    let ens_env = env.extend("result".into(), fn_decl.ret.clone());'
    )

    doc_old = (
        "/// [P2-REFINE1] REQ-REFINE-1 call-site slice: when an argument is an Int literal\n"
        "/// and the parameter refine pred is a closed QF-LIA comparison/&& tree over\n"
        "/// the binder + literals, evaluate it. Some(false) " + ARROW + " type error (zero exec).\n"
        "/// Unevaluable / non-literal args stay soft (prove / runtime). Definition-time\n"
        "/// return-refine body reject: see [P2-REFINE1-DEF] `check_ret_refine_body`."
    )
    doc_new = (
        "/// [P2-REFINE1] REQ-REFINE-1 call-site slice: when an argument is an Int literal\n"
        "/// and the parameter refine pred is a closed QF-LIA comparison/&& tree over\n"
        "/// the binder + literals, evaluate it. Some(false) " + ARROW + " type error (zero exec).\n"
        "/// Unevaluable / non-literal args stay soft (prove / runtime). Definition-time\n"
        "/// return-refine body reject: see [P2-REFINE1-DEF] `check_ret_refine_body`."
    )

    helpers_old = (
        "    Ok(())\n"
        "}\n"
        "\n"
        "fn pred_holds_for_lit(pred: &Expr, binder: &str, val: i64) -> Option<bool> {"
    )
    helpers_new = (
        "    Ok(())\n"
        "}\n"
        "\n"
        "/// [P2-REFINE1-DEF] REQ-REFINE-1 definition-time: `{r: Int | pred}` return type\n"
        "/// vs a *closed* body (Int literal / unary-minus / closed `if` tree). Decidable\n"
        "/// false " + ARROW + " type error (zero exec). Param-dependent bodies and requires-guided\n"
        "/// binds stay soft (prove / runtime).\n"
        "fn check_ret_refine_body(fn_decl: &FnDecl) -> Result<(), TypeError> {\n"
        "    let Type::Refine {\n"
        "        name: binder,\n"
        "        pred: Some(pred),\n"
        "    } = &fn_decl.ret\n"
        "    else {\n"
        "        return Ok(());\n"
        "    };\n"
        "    // Stmt-bearing bodies need dataflow; keep soft for this slice.\n"
        "    if !fn_decl.body.stmts.is_empty() {\n"
        "        return Ok(());\n"
        "    }\n"
        "    let Some(result) = &fn_decl.body.result else {\n"
        "        return Ok(());\n"
        "    };\n"
        "    let Some(value) = eval_closed_int_expr(result) else {\n"
        "        return Ok(());\n"
        "    };\n"
        "    if pred_holds_for_lit(pred, binder, value) == Some(false) {\n"
        "        return Err(TypeError::at(\n"
        "            fn_decl.span,\n"
        "            format!(\n"
        '                "[P2-REFINE1-DEF] body returns {value} which violates return refinement of {}",\n'
        "                fn_decl.name\n"
        "            ),\n"
        "        ));\n"
        "    }\n"
        "    Ok(())\n"
        "}\n"
        "\n"
        "fn eval_closed_int_expr(expr: &Expr) -> Option<i64> {\n"
        "    match expr {\n"
        "        Expr::LitInt { value, .. } => Some(*value),\n"
        '        Expr::UnaryOp { op, expr, .. } if op == "-" => eval_closed_int_expr(expr)?.checked_neg(),\n'
        "        Expr::IfExpr {\n"
        "            cond,\n"
        "            then_body,\n"
        "            else_body,\n"
        "            ..\n"
        "        } => {\n"
        "            // Empty binder => Names in cond fail closedness (soft).\n"
        '            let c = pred_holds_for_lit(cond, "", 0)?;\n'
        "            let branch = if c { then_body } else { else_body };\n"
        "            if !branch.stmts.is_empty() {\n"
        "                return None;\n"
        "            }\n"
        "            eval_closed_int_expr(branch.result.as_ref()?)\n"
        "        }\n"
        "        _ => None,\n"
        "    }\n"
        "}\n"
        "\n"
        "fn pred_holds_for_lit(pred: &Expr, binder: &str, val: i64) -> Option<bool> {"
    )

    tests_old = (
        "    fn refine1_rejects_negative_literal_call() {\n"
        "        // [P2-REFINE1] `-5` (unary minus over a literal) is a literal for reject purposes.\n"
        '        let src = r#"\n'
        "fn pos(x: {x: Int | x >= 1}) -> Int {\n"
        "    x\n"
        "}\n"
        "fn main(console: Console) -> Unit uses {console} {\n"
        "    console.print(pos(-5).show());\n"
        "}\n"
        '"#;\n'
        '        let prog = parse(src).expect("parse");\n'
        '        let err = check_program(&prog).expect_err("expected P2-REFINE1 reject");\n'
        '        assert!(err.0.contains("[P2-REFINE1]"), "{err}");\n'
        "    }\n"
        "\n"
        "    #[test]\n"
        "    fn propagate_into_plain_int_ret_is_rejected() {"
    )

    tests_new = (
        "    fn refine1_rejects_negative_literal_call() {\n"
        "        // [P2-REFINE1] `-5` (unary minus over a literal) is a literal for reject purposes.\n"
        '        let src = r#"\n'
        "fn pos(x: {x: Int | x >= 1}) -> Int {\n"
        "    x\n"
        "}\n"
        "fn main(console: Console) -> Unit uses {console} {\n"
        "    console.print(pos(-5).show());\n"
        "}\n"
        '"#;\n'
        '        let prog = parse(src).expect("parse");\n'
        '        let err = check_program(&prog).expect_err("expected P2-REFINE1 reject");\n'
        '        assert!(err.0.contains("[P2-REFINE1]"), "{err}");\n'
        "    }\n"
        "\n"
        "    #[test]\n"
        "    fn refine1_def_rejects_negative_literal_return() {\n"
        "        // [P2-REFINE1-DEF] SPEC section 4.4 definition-time negative return.\n"
        '        let src = r#"\n'
        "fn bad() -> {r: Int | r >= 0} {\n"
        "    -1\n"
        "}\n"
        "fn main(console: Console) -> Unit uses {console} {\n"
        "    console.print(bad().show());\n"
        "}\n"
        '"#;\n'
        '        let prog = parse(src).expect("parse");\n'
        '        let err = check_program(&prog).expect_err("expected P2-REFINE1-DEF reject");\n'
        "        assert!(\n"
        '            err.0.contains("[P2-REFINE1-DEF]"),\n'
        '            "expected [P2-REFINE1-DEF] in {err}"\n'
        "        );\n"
        "    }\n"
        "\n"
        "    #[test]\n"
        "    fn refine1_def_accepts_nonneg_literal_return() {\n"
        '        let src = r#"\n'
        "fn good() -> {r: Int | r >= 0} {\n"
        "    0\n"
        "}\n"
        "fn main(console: Console) -> Unit uses {console} {\n"
        "    console.print(good().show());\n"
        "}\n"
        '"#;\n'
        '        let prog = parse(src).expect("parse");\n'
        '        check_program(&prog).expect("nonneg literal return must typecheck");\n'
        "    }\n"
        "\n"
        "    #[test]\n"
        "    fn refine1_def_rejects_closed_ite_false_branch() {\n"
        "        // Closed ite: cond + branches are literals " + ARROW + " decidable without SMT.\n"
        '        let src = r#"\n'
        "fn bad() -> {r: Int | r >= 0} {\n"
        "    if 1 < 0 { 1 } else { -1 }\n"
        "}\n"
        "fn main(console: Console) -> Unit uses {console} {\n"
        "    console.print(bad().show());\n"
        "}\n"
        '"#;\n'
        '        let prog = parse(src).expect("parse");\n'
        '        let err = check_program(&prog).expect_err("expected P2-REFINE1-DEF reject");\n'
        '        assert!(err.0.contains("[P2-REFINE1-DEF]"), "{err}");\n'
        "    }\n"
        "\n"
        "    #[test]\n"
        "    fn refine1_def_soft_on_param_dependent_body() {\n"
        "        // Body mentions param - not closed; stay soft (prove/runtime).\n"
        '        let src = r#"\n'
        "fn id(x: Int) -> {r: Int | r >= 0} {\n"
        "    x\n"
        "}\n"
        "fn main(console: Console) -> Unit uses {console} {\n"
        "    console.print(id(1).show());\n"
        "}\n"
        '"#;\n'
        '        let prog = parse(src).expect("parse");\n'
        '        check_program(&prog).expect("param-dependent return refine stays soft");\n'
        "    }\n"
        "\n"
        "    #[test]\n"
        "    fn propagate_into_plain_int_ret_is_rejected() {"
    )

    for name, old, new in (
        ("hook", hook_old, hook_new),
        ("doc", doc_old, doc_new),
        ("helpers", helpers_old, helpers_new),
        ("tests", tests_old, tests_new),
    ):
        n = text.count(old)
        if n != 1:
            print("FAIL: anchor", name, "count=", n)
            return 1
        text = text.replace(old, new, 1)

    print("[*] Anchors OK; marker count=", text.count(MARKER))
    if not args.apply:
        print("[*] DRY-RUN MODE. Pass --apply to write changes.")
        return 0

    ts = time.strftime("%Y%m%d_%H%M%S")
    bak = target.with_name(target.name + ".bak_" + ts + "_p2_refine1_def")
    shutil.copy2(target, bak)
    print("[*] Backup:", bak)
    atomic_write(target, text, eol)
    post = target.read_bytes()
    if MARKER.encode() not in post:
        shutil.copy2(bak, target)
        print("FAIL: marker missing; rolled back")
        return 1
    print("[*] Read-back MD5:", md5_lf(post))
    print("[*] APPLY OK")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())