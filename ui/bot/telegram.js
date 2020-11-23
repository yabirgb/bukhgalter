//const fetch = require("node-fetch");
const { Telegraf } = require('telegraf')

const bot = new Telegraf(process.env.TELEGRAM_TOKEN);
//const API = process.env.API;

bot.start((ctx) => ctx.reply(process.uptime()));

bot.hears(/\/hello/, async (ctx) => {
  bot.telegram.sendMessage(ctx.chat.id, "hello bro");
});

//bot.launch()
module.exports.handler = async event => {
  await bot.handleUpdate(JSON.parse(event.body));
  return { statusCode: 200, body: '' };
}
