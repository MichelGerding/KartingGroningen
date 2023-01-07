function createLapTimeViolinChart(data, ctx, dataType) {
    let typeIndex;
    if (dataType === "outlier") {
        typeIndex = "outlier_laps";
    } else if (dataType === "all") {
        typeIndex = "all_laps";
    } else if (dataType === "normal") {
        typeIndex = "normal_laps";
    }


    // const maxLaps = Math.max(...data.drivers.map(e => e.total_laps));

    const colorRangeInfo = {
        colorStart: 0.2,
        colorEnd: 1,
        useEndAsStart: false,
    }
    const COLOURS = interpolateColors(20, d3.interpolateTurbo, colorRangeInfo)

    const allLapTimes = []
    const normalLapTimes = []
    data.drivers.forEach((driver, index) => {
        driver.all_laps.forEach((lap, lapIndex) => {
            allLapTimes.push(lap.lap_time)
        })
        driver.normal_laps.forEach((lap, lapIndex) => {
            normalLapTimes.push(lap.lap_time)
        })
    })

    return new Chart(ctx, {
        type: 'violin',
        data: {
            labels: [""],
            datasets: [{
                label: "All Laps",
                backgroundColor: "rgba(182, 242, 65, 0.8",
                borderColor: "rgba(91, 121, 33, 1",
                borderWidth: 1,
                itemRadius: 2,
                data: [allLapTimes]
            }, {
                label: "Normal Laps",
                backgroundColor: "rgba(191, 36, 36, 0.8",
                borderColor: "rgba(91, 121, 33, 1",
                borderWidth: 1,
                itemRadius: 2,
                data: [normalLapTimes]
            }],
        },
        options: {
            indexAxis: "y",

            responsive: true,
            plugins: {
                legend: {
                    position: 'right'
                }
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