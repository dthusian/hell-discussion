import { spawn } from "child_process";
import { writeFile } from "fs/promises";

const SCORE_THRESHOLD = 2200;

function spawnUtil(cmd, args) {
  return new Promise((resolve, reject) => {
    let proc = spawn(cmd, args, {
      stdio: "pipe"
    });
    let data = "";
    proc.stdout.on("data", v => {
      data += v.toString("utf8");
    });
    proc.on("exit", code => {
      if(code !== 0) reject(code);
      else resolve(data);
    });
  });
}

async function filter(s, msg) {
  if(s && s.trim() !== "") return false;
  if(msg.attachments.size === 0) return false;
  let promises = await Promise.all(msg.attachments.map(async v => {
    if(!v.contentType.toString().startsWith("image")) return false;
    if(v.height !== 256 || v.width !== 296) return false;
    await fetch(v.url)
      .then(v => v.arrayBuffer())
      .then(v => Buffer.from(v))
      .then(v => writeFile("/tmp/testimage", v));
    let score = parseFloat(await spawnUtil("imgdiff/target/release/imgdiff", ["/tmp/testimage"]));
    console.log(`info: image ${v.url} score ${score}`);
    return score > SCORE_THRESHOLD;
  }));
  return promises.every(v => v);
}

export function sekaiFilter() {
  return {
    filter: filter,
    errorMsg: "must communicate only in project sekai stickers (generate one at st.ayaka.one)\nor your image looked too unlike a sticker",
    errorMsgInv: "must not contain a project sekai sticker"
  }
}