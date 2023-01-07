
function createLapTimeChart(data, ctx, dataType) {
    let typeIndex;
    if (dataType === "outlier") {
        typeIndex = "outlier_laps";
    } else if (dataType === "all") {
        typeIndex = "all_laps";
    } else if (dataType === "normal") {
        typeIndex = "normal_laps";
    }




    const maxLaps = Math.max(...data.drivers.map(e => e.total_laps));

    const colorRangeInfo = {
        colorStart: 0.2,
        colorEnd: 1,
        useEndAsStart: false,
    }
    const COLOURS = interpolateColors(data.drivers.length, d3.interpolateTurbo, colorRangeInfo, )
    console.log(COLOURS)

    const datasets = data.drivers.map((driver, index) => {
        let laps = [];
        driver[typeIndex].forEach((lap, lapIndex) => {
            laps.push({
                x: lap.lap_in_heat ,
                y: lap.lap_time,
            })
        })

        return {
            label: driver.driver_name,
            data: laps,
            borderColor: COLOURS[index],
            backgroundColor: COLOURS[index],
            borderWidth: 3,
            tension: 0.3,
            fill: false,
        }
    });

    const avgLapTimes = []
    for (let i=0; i < maxLaps; i++) {
        let totalLaptime = 0;
        let driverCount = 0;
        datasets.forEach((dataset) => {
            if (dataset.data[i]) {
                driverCount++;
                totalLaptime += dataset.data[i];
            }
        });

        avgLapTimes.push(totalLaptime / driverCount);
    }

    datasets.push({
        label: 'All Driver Average',
        data: avgLapTimes,
        borderColor: 'rgba(255, 255, 255, 0.5)',
        backgroundColor: 'rgba(255, 255, 255, 0.5)',
        borderWidth: 3,
        tension: 0.3,
        fill: false,

    });


    return new Chart(ctx, {
        type: 'line',
        data: {
            labels: Array.from({length: maxLaps}, (_, i) => i + 1),
            datasets: datasets
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
                    grid: {color: "rgb(200,200,200)"},
                },
                x: {
                    title: {
                        display: true,
                        text: 'Lap in heat',
                    },
                    grid: {color: "rgb(200,200,200)"},
                }
            }
        }
    });

}