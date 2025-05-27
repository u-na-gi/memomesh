import { execSync } from "child_process";
import fs from "fs";

const settingsFile = ".vscode/settings.json";

if (!fs.existsSync(settingsFile)) {
  console.error("❌ settings.json not found");
  process.exit(1);
}

// which protoc
let protocPath;
try {
  protocPath = execSync("which protoc").toString().trim();
} catch (e) {
  console.error("❌ protoc not found in path");
  process.exit(1);
}

// 読み込み & パース
const raw = fs.readFileSync(settingsFile, "utf8");
let settings;
try {
  settings = JSON.parse(raw);
} catch (e) {
  console.error("❌ settings.json is not valid JSON");
  process.exit(1);
}

// 値の更新
settings["rust-analyzer.server.extraEnv"] = {
  ...(settings["rust-analyzer.server.extraEnv"] || {}),
  PROTOC: protocPath,
};

// 書き戻し（整形付き）
fs.writeFileSync(settingsFile, JSON.stringify(settings, null, 2));
console.log(`✅ Updated PROTOC to ${protocPath}`);
