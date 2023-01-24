function createLapTimeViolinChart(data, ctx) {
    const datasets = [
        data.datasets.find(e => e.label === "All Laps"),
        data.datasets.find(e => e.label === "All Laps (Normalized)"),
    ];



    const colorRangeInfo = {
        colorStart: 0.2,
        colorEnd: 1,
        useEndAsStart: false,
    }
    const COLOURS = interpolateColors(datasets.length, d3.interpolateTurbo, colorRangeInfo)

    return new Chart(ctx, {
        type: 'violin',
        data: {
            labels: [""],
            datasets: datasets.map((dataset, i) => {
                return {
                    label: datasets[i].label,
                    backgroundColor: COLOURS[i],
                    borderColor: COLOURS[i],
                    borderWidth: 1,
                    itemRadius: 2,
                    data: [datasets[i].data.map(e => e.lap_time)]
                }
            }),
        },
        options: {
            indexAxis: "y",

            responsive: true,
            plugins: {
                legend: {
                    position: 'right'
                },
                zoom: {
                    zoom: {
                        pan: {
enabled: true,
                        },
                        limits: {
                            x: {
                                min: 0,
                                max: 200,
                            },
                        },
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
            aspectRatio: 32 / 27,
            scales: {
                y: {
                    grid: {
                        color: "rgb(200,200,200)"
                    }
                    ,
                }
                ,
                x: {
                    beginAtZero: false,
                    suggestedMin: 46,
                    max: 100,
                    grid: {color: "rgb(200,200,200)"},
                    title: {display: true, text: 'Laptimes (s)',},
                    ticks: {stepSize: 0.5}
                }
            }
        }
    });
}