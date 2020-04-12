package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"runtime"
	"strconv"

	"github.com/shirou/gopsutil/cpu"
	"github.com/shirou/gopsutil/host"
	"github.com/shirou/gopsutil/mem"
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

func dealwithErr(err error) {
	if err != nil {
		fmt.Println(err)
		//os.Exit(-1)
	}
}

// GetData info
func GetData(w http.ResponseWriter, r *http.Request) {
	runtimeOS := runtime.GOOS
	// memory
	vmStat, err := mem.VirtualMemory()
	dealwithErr(err)

	// cpu - get CPU number of cores and speed
	cpuStat, err := cpu.Info()
	dealwithErr(err)
	percentage, err := cpu.Percent(0, true)
	dealwithErr(err)

	hostStat, err := host.Info()
	dealwithErr(err)

	var ram = RAM{strconv.FormatUint(vmStat.Total, 10), vmStat.UsedPercent}
	var read float64
	for idx, cpupercent := range percentage {
		read = cpupercent
		println(idx)
	}
	var cpu = CPU{
		cpuStat[0].Cores,
		cpuStat[0].VendorID,
		cpuStat[0].Family,
		cpuStat[0].ModelName,
		strconv.FormatFloat(cpuStat[0].Mhz, 'f', 2, 64),
		read}

	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Access-Control-Allow-Origin", "*")
	d := Data{Os: runtimeOS, HostName: hostStat.Hostname, Platform: hostStat.Platform, Ram: ram, Cpu: cpu}
	json.NewEncoder(w).Encode(d)
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/", GetData)
	http.ListenAndServe(":8080", mux)
}
