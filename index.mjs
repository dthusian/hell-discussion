import { Client, Embed, EmbedBuilder, Events, GatewayIntentBits } from "discord.js";
import { readFile } from "fs/promises";
import { filters } from "./filters/all.mjs"
import { readFileSync } from "fs";

let bot = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent
  ]
});
let options = JSON.parse(await readFile("config.json"));

async function applyFilterSet(msg, ruleset) {
  if(!ruleset) return;
  let appliedFilters = ruleset["filters"];
  let responses = Promise.all(appliedFilters.map(async v => {
    let inverted = false;
    if(v.startsWith("!")) {
      inverted = true;
      v = v.slice(1);
    }
    let filterObj = filters[v];
    let passed = filterObj.filter(msg.content, msg);
    if(passed instanceof Promise) passed = await passed;
    if(inverted) passed = !passed;
    if(passed) {
      return null;
    } else {
      if(!inverted) {
        return filterObj.errorMsg;
      } else {
        return filterObj.errorMsgInv;
      }
    }
  }));
  return (await responses).filter(v => v);
}

async function inspectMessage(msg) {
  // filter extraneous things
  if(msg.author.id === "1153388878863024151") return;
  console.log(`info: message rcvd: ${msg.cleanContent}`);
  // apply filters
  const cid = msg.channelId.toString();
  const gid = msg.guildId.toString();
  let responses = [];
  if(options[cid]) responses = responses.concat(await applyFilterSet(msg, options[cid]));
  if(options[gid] && !options[gid]["except"].includes(cid)) responses = responses.concat(applyFilterSet(msg, options[gid]));
  responses = Array.from(new Set(responses).values());
  if(responses.length) {
    // delete the nonconformant message
    if(msg.deletable) {
      await msg.delete();
      console.log(`info: deleted message: ${msg.cleanContent} --- reason: ${responses}`)
      let embed = EmbedBuilder.from({
        title: "Message Deleted",
        fields: [{
          name: "Reason",
          value: responses.join("\n")
        }],
        color: 0xff3333
      });
      await msg.channel.send({
        embeds: [embed]
      });
    } else {
      console.log("warn: couldn't delete message (not enough perms?)")
    }
  }
}

bot.on(Events.MessageCreate, inspectMessage);
bot.on(Events.MessageUpdate, (oldM, newM) => inspectMessage(newM));

bot.on(Events.ClientReady, () => {
  console.log("info: discord connected")
});

bot.on(Events.Error, err => {
  console.log(`errr: ${err}`);
});

bot.login(readFileSync("token.txt").toString("utf8").trim());