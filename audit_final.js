import fs from "node:fs";

const repo = "Riverbraid-Types";
const requiredFiles = ["README.md", "package.json"];
const missing = requiredFiles.filter((file) => !fs.existsSync(file));
let packageJsonParseable = false;
try {
  const packageText = fs.readFileSync("package.json", "utf8").replace(/^\uFEFF/, "");
  JSON.parse(packageText);
  packageJsonParseable = true;
} catch {
  packageJsonParseable = false;
}
const ok = missing.length === 0 && packageJsonParseable;
const output = {
  schema: "riverbraid.audit_final.output",
  version: "1.0.0",
  repo,
  status: ok ? "SCAFFOLD_CHECK_PASSED" : "SCAFFOLD_CHECK_FAILED",
  audit_scope: "workflow-target-presence-and-json-parse-check",
  claim_boundary: "presence-check-only-not-full-verification",
  required_files: requiredFiles,
  missing_files: missing,
  package_json_parseable_with_bom_strip: packageJsonParseable,
  non_claims: [
    "not certification",
    "not external audit",
    "not production readiness",
    "not security hardening",
    "not registry freshness",
    "not full protocol verification"
  ]
};

fs.writeFileSync("audit-final-output.json", JSON.stringify(output, null, 2) + "\n", "utf8");
if (!ok) {
  console.error(`${repo}_AUDIT_FINAL_SCAFFOLD_CHECK_FAILED`);
  process.exit(1);
}
console.log(`${repo}_AUDIT_FINAL_SCAFFOLD_CHECK_PASSED`);
