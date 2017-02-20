# -*- coding: utf-8 -*-

import telebot
import expr
from os import path
from wrapper import RustWrap

token = ''
with open(path.join(path.dirname(path.abspath(__file__)), 'token')) as h:
    token = eval(h.read())

class Main:
    def __init__(self):
        self.bot   = telebot.TeleBot(token)
        self.state = None
        self.key   = None
        self.store = {}

        bot = self.bot

        def sendErr(foo):
            def bar(m):
                #stat = self.get(m.chat.id)
                try:
                    return foo(m)
                except Exception as e:
                    bot.send_message(m.chat.id, 'CONGRATULATIONS')
                    bot.send_message(m.chat.id, 'YOU CRASH ME')
                    bot.send_message(m.chat.id, 'THIS IS YOUR LOG:')
                    bot.send_message(m.chat.id, e.__repr__())
            return bar

        @bot.message_handler(func=lambda message: True, content_types=['text'])
        @sendErr
        def answer(mess):
            id  = mess.chat.id
            #self.bot.send_message(id, u'не понимаю команду')
            exprSrc = mess.text
            try:
               e = expr.parser.parse(exprSrc)
            except:
                self.bot.send_message(id, "некорректное выражение")
                return
            vrs = expr.getAllVars(e)
            tbl = expr.makeTbl(e,vrs)
            (used,unused) = expr.splitByUsing(e,tbl,vrs)
            if len(used) == 0:
                mess = "Тафтология. Всегда " % tbl[0]['!']
                self.bot.send_message(id, mess)
                return
#            if len(unused) > 0:
#               self.bot.send_message(id, 'неиспользуемые переменные: ' % unused)
            vrs = used
            envs = expr.makeTbl(e,vrs)
            wrap = RustWrap('./libsolver.so')
            analog = wrap.findAnalog([t['!'] for t in envs], len(vrs), 1000, int(len(vrs) * 2.5))
            if analog is None:
                self.bot.send_message(id, 'аналог не удалось найти')
            else:
                analog = expr.parser.parse(analog)
                analog.replace(dict(zip(expr.getAllVars(analog), vrs)))
                self.bot.send_message(id, str(analog))

        self.bot.polling(none_stop=True)

import os

def main():
    if os.fork() == 0:
        print('ok')
        os.setsid()
        Main()
    else:
        print('child')

Main()
