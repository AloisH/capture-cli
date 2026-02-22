const https = require("https");
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

const REPO = "AloisH/capture-cli";
const VERSION = require("../package.json").version;

const PLATFORM_MAP = {
  linux: "unknown-linux-gnu",
  darwin: "apple-darwin",
};

const ARCH_MAP = {
  x64: "x86_64",
  arm64: "aarch64",
};

const platform = PLATFORM_MAP[process.platform];
const arch = ARCH_MAP[process.arch];

if (!platform) {
  console.error(`Unsupported platform: ${process.platform}`);
  process.exit(1);
}
if (!arch) {
  console.error(`Unsupported architecture: ${process.arch}`);
  process.exit(1);
}

const target = `${arch}-${platform}`;
const url = `https://github.com/${REPO}/releases/download/v${VERSION}/capture-${target}.tar.gz`;
const nativeDir = path.join(__dirname, "..", "native");
const tarball = path.join(nativeDir, "capture.tar.gz");

fs.mkdirSync(nativeDir, { recursive: true });

console.log(`Downloading capture ${VERSION} (${target})...`);

function download(url) {
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        return download(res.headers.location).then(resolve, reject);
      }
      if (res.statusCode !== 200) {
        return reject(new Error(`Download failed: HTTP ${res.statusCode}`));
      }
      const out = fs.createWriteStream(tarball);
      res.pipe(out);
      out.on("finish", () => out.close(resolve));
      out.on("error", reject);
    }).on("error", reject);
  });
}

download(url)
  .then(() => {
    execSync(`tar xzf capture.tar.gz`, { cwd: nativeDir });
    fs.unlinkSync(tarball);
    fs.chmodSync(path.join(nativeDir, "capture"), 0o755);
    console.log("capture installed successfully");
  })
  .catch((err) => {
    console.error(`Failed to install capture: ${err.message}`);
    process.exit(1);
  });
