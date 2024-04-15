#!/usr/bin/python3
import socket
import time

s = socket.socket()
host = socket.gethostname()
port = 13200

s.connect((host, port))
print(s.recv(1024))
time.sleep(2)
s.close()

