function timeLineChart(data, ctx, dataLabels = ["Median", "Average", "Minimum"]) {
    // depp copy data
    data = JSON.parse(JSON.stringify(data));

    let filteredSets =data.datasets
        .filter(e => dataLabels.includes(e.label) || dataLabels[0] == "All")


    let days = new Set();

    let rawDataSets = filteredSets.map(dataset => {
            let key = dataset.label
            let humanKey = key.charAt(0).toUpperCase() + key.slice(1);

            let label = "";
            if (dataLabels[0] == "All") {
                label = "Kart " + humanKey;
            } else {
                label = humanKey + " Lap Times";
            }

            return {
                label: label,
                data: dataset.data.map(e => {
                    days.add(e.date);
                    return {
                        date: e.date,
                        lapTime: e.lap_time
                    }

                }),
            }
        })

    const datasets = rawDataSets.map((dataset, i) => {
        return {
            label: dataset.label,
            data: dataset.data.sort(
                (a, b) => new Date(a.date) - new Date(b.date)
            ).map(e => e.lapTime),
            hidden: i > 3,
        }
    });

    console.log(rawDataSets)

    return new Chart(ctx, {
        type: 'line',
        data: {
            labels: Array.from(days).sort((a, b) => {
                var aDate = new Date(a);
                var bDate = new Date(b);
                return aDate - bDate;
            }).map(e => e),
            datasets: datasets
        },
        options: {
            responsive: true,
            plugins: {
                legend: {
                    position: datasets.length === 3 ? 'right' : 'top',
                },
                zoom: {
                    zoom: {

                        wheel: {
                            enabled: true,
                            modifierKey: 'ctrl',
                        },
                        pinch: {
                            enabled: true
                        },
                        mode: 'x',
                    },
                },
            },
            aspectRatio: datasets.length === 3 ? 32 / 9 : 32/18,
            scales: {
                y: {
                    title: {
                        display: true,
                        text: 'Laptimes (s)',
                    },
                    beginAtZero: false,
                    suggestedMin: 45,
                    suggestedMax: 70,
                    grid: {color: "rgb(200,200,200)"},
                },
                x: {
                    type: "timeseries",
                    ticks: {
                        source: 'labels',
                    },
                    time: {
                        parser: 'yyyy-MM-dd',
                        tooltipFormat: 'dd/MM/yyyy',
                        unit: 'day',
                        displayFormats: {
                            'day': 'dd/MM/yyyy'
                        }
                    },
                    title: {
                        display: true,
                        text: 'Lap',
                    },
                    grid: {color: "rgb(200,200,200)"},
                }
            }
        }
    });

}