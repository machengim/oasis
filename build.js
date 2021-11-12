const fs = require('fs');
const process = require("process");
const child_process = require("child_process");
const path = require("path");

const copyRecursiveSync = (src, dest) => {
  const exists = fs.existsSync(src);
  const stats = exists && fs.statSync(src);
  const isDirectory = exists && stats.isDirectory();
  if (isDirectory) {
    fs.mkdirSync(dest);
    fs.readdirSync(src).forEach(function (childItemName) {
      copyRecursiveSync(path.join(src, childItemName),
        path.join(dest, childItemName));
    });
  } else {
    fs.copyFileSync(src, dest);
  }
};

const runCommand = (cmd) => {
  console.log("\n", cmd);
  child_process.execSync(cmd, { stdio: 'inherit' });
}

const createReleaseDir = () => {
  if (fs.existsSync("release")) {
    fs.rmSync("release", { recursive: true, force: true });
  }

  fs.mkdirSync("release/oasis", { recursive: true }, (e) => {
    if (e) {
      throw e;
    }
  });
}

const filename = process.platform.startsWith("win") ? "oasis.exe" : "oasis";

createReleaseDir();

process.chdir("frontend");
runCommand("npm i");
runCommand("npm run build");
copyRecursiveSync("public", "../release/oasis/public");

process.chdir("../backend");
runCommand("cargo build --release");
copyRecursiveSync("target/release/" + filename, "../release/oasis/" + filename);
copyRecursiveSync("assets/oasis.conf.sample", "../release/oasis/oasis.conf.sample");

process.chdir("../release/oasis");
fs.chmodSync(filename, 0o755);

console.log("\nBuild complete. Please check the 'release' directory.");