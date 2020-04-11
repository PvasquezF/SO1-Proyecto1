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

var data TodoPage
var index = 0

func showPage(w http.ResponseWriter, r *http.Request) {
	tmpl, _ := template.ParseFiles("layout.html")
	responseObject := getData()

	data.Times[index] = time.Now().Format("01-02-2006 15:04:05")
	data.Values[index] = strconv.FormatInt(responseObject.Ram.Usage, 10)
	index = index + 1
	data.Index = index
	data.Valores = strings.Join(data.Values[:], ",")
	data.Tiempos = strings.Join(data.Times[:], ",")
	tmpl.Execute(w, data)
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
