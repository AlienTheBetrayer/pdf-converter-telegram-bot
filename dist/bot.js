import 'dotenv/config';
import { Bot } from "grammy";
// bot init
const bot = new Bot(process.env.TOKEN);
bot.on("message", (ctx) => ctx.reply(ctx.message.text ?? "What?"));
bot.start();
//# sourceMappingURL=bot.js.map