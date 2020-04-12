from flask import Flask, render_template
import requests
import json
import redis
app = Flask(__name__)

@app.route('/')
def hello_world():
    r = redis.Redis(host='35.208.41.153', port=6379, db=0)
    p = r.lrange('cpu', 0, -1)
    cpuarray = []

    for i in p:
        tupla = str(i).split('|')
        print(tupla)
        valor = tupla[0].replace('b\'','')
        tiempo = tupla[1].replace('\'','')
        dataCpu = {"valor": valor,
                    "tiempo": tiempo}
        cpuarray.append(dataCpu)
    print(cpuarray)
    return render_template("index.html")