
function createAllKartLapChart(data, ctx, dataType) {
    let typeIndex;
    if (dataType === "outlier") {
        typeIndex = "outlier_laps";
    } else if (dataType === "all") {
        typeIndex = "all_laps";
    } else if (dataType === "normal") {
        typeIndex = "normal_laps";
    }





    const colorRangeInfo = {
        colorStart: 0.2,
        colorEnd: 1,
        useEndAsStart: false,
    }
    // get the amount of unique dirvers
    const uniqueDrivers = [...new Set(data.map(e => e.driver.name))];
    console.log(uniqueDrivers)
    const COLOURS = interpolateColors(uniqueDrivers.length, d3.interpolateTurbo, colorRangeInfo, )

    const pointColours = []
    let colorIndex = 0;
    let lastDriver = data[0].driver.name;
    const dataset = {
        label: "All Lap Times",
        data: data.map(e => {
            if (e.driver.name !== lastDriver) {
                colorIndex++;
                lastDriver = e.driver.name;
            }
            pointColours.push(COLOURS[colorIndex])

            return e.lap.lap_time;
        }),
        pointBackgroundColor: pointColours,
    }

    console.log(dataset)
    return new Chart(ctx, {
        type: 'line',
        data: {
            labels: Array.from({length: data.length}, (_, i) => i + 1),
            datasets: [dataset]
        },
        options: {
            responsive: true,
            plugins: {
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
                    suggestedMin: 45,
                    max: 100,
                    grid: {color: "rgb(200,200,200)"},
                },
                x: {
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