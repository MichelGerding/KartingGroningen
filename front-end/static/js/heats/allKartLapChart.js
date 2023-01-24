function createAllKartLapChart(data, ctx) {
    // copy data
    data = JSON.parse(JSON.stringify(data));
    const baseData = data.datasets.find(d => d.label === 'All Laps');
    console.log(baseData)


    const colorRangeInfo = {
        colorStart: 0.2,
        colorEnd: 1,
        useEndAsStart: false,
    }

    // get the amount of unique dirvers
    const uniqueDrivers = [...new Set(baseData.data.map(e => e.driver.name))];
    const COLOURS = interpolateColors(uniqueDrivers.length, d3.interpolateTurbo, colorRangeInfo,)
    const getColour = (index) => {
        let amountColors = COLOURS.length;
        let halfIndex = Math.floor(amountColors / 2);


        if (index % 2 == 0) {
            return COLOURS[index];
        }

        return COLOURS[amountColors - index];

    }

    let min_lap = 99999;

    const pointColours = []
    let colorIndex = 0;
    let lastDriver = baseData.data[0].driver.name;
    const dataset = {
        label: "All Lap Times",
        fill: false,
        tension: 0.1,
        data: baseData.data.map(e => {
            if (e.driver.name !== lastDriver) {
                colorIndex++;
                lastDriver = e.driver.name;
            }
            pointColours.push(getColour(colorIndex))

            if (e.lap_time < min_lap) {
                min_lap = e.lap_time;
            }
            return e.lap_time;
        }),
        pointBackgroundColor: pointColours,
    }
    console.log(baseData.data[0])

    return new Chart(ctx, {
        type: 'line',
        data: {
            labels: Array.from(Array(baseData.data.length).keys()),
            datasets: [dataset]
        },
        options: {
            responsive: true,
            plugins: {
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
                legend: {
                    position: 'right'
                }
            },
            aspectRatio: 32 / 9,
            scales: {
                y: {
                    title: {
                        display: true,
                        text: 'Laptimes (s)',
                    },
                    beginAtZero: false,
                    grid: {color: "rgb(200,200,200)"},
                },
                x: {
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