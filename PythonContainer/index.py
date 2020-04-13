from flask import Flask, render_template
import requests
import json
import redis
app = Flask(__name__)

@app.route('/data')
def hello_world():
    r = redis.Redis(host='35.208.41.153', port=6379, db=0)
    p = r.lrange('cpu', 0, -1)
    q = r.lrange('ram', 0, -1)
    cpuarray = []
    ramarray = []
    for i in p:
        tupla = str(i).split('|')
        valor = tupla[0].replace('b\'','')
        tiempo = tupla[1].replace('\'','')
        dataCpu = '{"valor": "'+valor+'", "tiempo": "'+tiempo+'"}'
        cpuarray.append(json.loads(dataCpu))

    for i in q:
        tupla = str(i).replace('b\'','')
        tupla = tupla.replace('\'', '')
        ramarray.append(json.loads(tupla))
    return {"ram": ramarray, "cpu": cpuarray}

@app.route('/')
def index():
    return render_template("index.html")

app.run(host ='0.0.0.0', port = 5000)