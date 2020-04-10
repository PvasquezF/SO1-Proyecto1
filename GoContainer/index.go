package main

import (
	"fmt"
	"html/template"
	"net/http"
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
	Total string  `json:"total"`
	Usage float64 `json:"usage"`
}

// CPU struct
type CPU struct {
	Cores  int32   `json:"cores"`
	Vendor string  `json:"vendor"`
	Family string  `json:"family"`
	Model  string  `json:"model"`
	Speed  string  `json:"speed"`
	Read   float64 `json:"read"`
}

type Todo struct {
	Title string
	Done  bool
}

type TodoPageData struct {
	PageTitle string
	Todos     []Todo
}

func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hi there, I love %s!", r.URL.Path[1:])
}

func main() {
	//response, err := http.Get("http://localhost:8080/")
	//
	//if err != nil {
	//	fmt.Print(err.Error())
	//	os.Exit(1)
	//}
	//
	//responseData, err := ioutil.ReadAll(response.Body)
	//if err != nil {
	//	log.Fatal(err)
	//}
	//
	//var responseObject Data
	//json.Unmarshal(responseData, &responseObject)
	//fmt.Println(responseObject.Os)
	//http.HandleFunc("/", handler)
	//log.Fatal(http.ListenAndServe(":8081", nil))

	tmpl := template.Must(template.ParseFiles("layout.html"))
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		data := TodoPageData{
			PageTitle: "My TODO list",
			Todos: []Todo{
				{Title: "Task 1", Done: false},
				{Title: "Task 2", Done: true},
				{Title: "Task 3", Done: true},
			},
		}
		tmpl.Execute(w, data)
	})
	http.ListenAndServe(":8081", nil)
}
