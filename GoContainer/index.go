package main

import (
	"encoding/json"
	"fmt"
	"html/template"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"
	"time"

	"github.com/go-redis/redis"
)

// Data struct
type Data struct {
	Os       string `json:"os"`
	HostName string `json:"hostname"`
	Platform string `json:"platform"`
	Ram      RAM    `json:"ram"`
	Cpu      CPU    `json:"cpu"`
}

// RAM struct
type RAM struct {
	Total string `json:"total"`
	Usage int64  `json:"usage"`
}

// CPU struct
type CPU struct {
	Cores  int32  `json:"cores"`
	Vendor string `json:"vendor"`
	Family string `json:"family"`
	Model  string `json:"model"`
	Speed  string `json:"speed"`
	Read   string `json:"read"`
}

// TodoPage struct
type TodoPage struct {
	Times   [100]string
	Values  [100]string
	Valores string
	Tiempos string
	Index   int
}

// Redis data struct
type RedisData struct {
	Valor  string
	Tiempo string
}

type redisClient struct {
	c *redis.Client
}

var (
	client = &redisClient{}
	data   TodoPage
	index  = 0
)

func showPage(w http.ResponseWriter, r *http.Request) {
	tmpl, _ := template.ParseFiles("layout.html")
	responseObject := getData()
	v1 := time.Now().Format("01-02-2006 15:04:05")
	v2 := strconv.FormatInt(responseObject.Ram.Usage, 10)
	data.Times[index] = v1
	data.Values[index] = v2
	index = index + 1
	data.Index = index
	data.Valores = strings.Join(data.Values[:], ",")
	data.Tiempos = strings.Join(data.Times[:], ",")

	redisClient := initialize()
	key1 := "ram"
	value1 := RedisData{Valor: v2, Tiempo: v1}
	err := redisClient.setKey(key1, value1, time.Minute*1)
	if err != nil {
		log.Fatalf("Error: %v", err.Error())
	}
	tmpl.Execute(w, data)
}

func initialize() *redisClient {
	c := redis.NewClient(&redis.Options{
		Addr: "35.208.41.153:6379",
	})

	if err := c.Ping().Err(); err != nil {
		panic("Unable to connect to redis " + err.Error())
	}
	client.c = c
	return client
}

func (client *redisClient) setKey(key string, value interface{}, expiration time.Duration) error {
	cacheEntry, err := json.Marshal(value)
	if err != nil {
		return err
	}
	err = client.c.LPush(key, cacheEntry).Err()
	if err != nil {
		return err
	}
	return nil
}

func getData() Data {
	response, err := http.Get("http://localhost:8080/")

	if err != nil {
		fmt.Print(err.Error())
		os.Exit(1)
	}

	responseData, err := ioutil.ReadAll(response.Body)
	if err != nil {
		log.Fatal(err)
	}

	var responseObject Data
	json.Unmarshal(responseData, &responseObject)
	return responseObject
}

func main() {
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))
	time.Sleep(2 * time.Second)
	http.HandleFunc("/", showPage)
	http.ListenAndServe(":8081", nil)
}
