const fetch = require("node-fetch");
const Telegraf = require("telegraf");

const API = process.env.API;

const bot = new Telegraf(process.env.TELEGRAM_TOKEN);

bot.start((ctx) => ctx.reply(process.uptime()));

bot.hears(/\/status/, async (ctx) => {
  bot.telegram.sendMessage(ctx.chat.id, "Bot funcionando");
});

bot.hears(/\/collaborators/, async (ctx) => {
    const payload = await (await fetch(API + `/api/collaborators`)).json();

    let result = ""; 
    payload.forEach(function(coll){result+="Nombre: " + coll['name'] + "\nDescripción: " + coll['description'] + "\nNombre de usuario: "+ coll['user'] + "\n\n"}); 
    bot.telegram.sendMessage(ctx.chat.id, result);
});
  
bot.hears(/\/collaborator (.+)/, async (ctx) => {
    const payload = await (await fetch(API + `/api/collaborators`)).json();

    let result = ""; 
    let name = ctx.message.text.substr(14);
    console.log(name);
    payload.forEach(function(coll){
        if (coll['user'].includes(name)){
            result+="Nombre: " + coll['name'] + "\nDescripción: " + coll['description'] + "\nNombre de usuario: "+ coll['user']; 
            bot.telegram.sendMessage(ctx.chat.id, result);
        }
    })

    if (result.length == 0){
        bot.telegram.sendMessage(ctx.chat.id, "No hay ningún colaborador con ese nombre");
    }
    
});

//bot.launch()

exports.handler = async (event, ctx, callback) => {
  await bot.handleUpdate(JSON.parse(event.body));
  return callback(null, { statusCode: 200 });
};
