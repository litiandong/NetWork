#!/usr/bin/python3

import socket
import time

s = socket.socket()
host = socket.gethostname()
port = 13200
s.bind((host, port))

s.listen(5)
while True:
    c, addr = s.accept()
    c.send('hello'.encode("utf-8"))
    c.send('world'.encode("utf-8"))
    time.sleep(1)
    c.close()

