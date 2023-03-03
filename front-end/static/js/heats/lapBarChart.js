function createLapBarChart(data, canvas) {
    const colorRangeInfo = {
        colorStart: 0.2,
        colorEnd: 1,
        useEndAsStart: false,
    }
    const maxLaps = Math.max(...data.datasets.map(e => e.data.length));

    const COLOURS = interpolateColors(maxLaps, d3.interpolateTurbo, colorRangeInfo,)


    //
    data.datasets = data.datasets.map((dataset, i) => {
        return {
            label: dataset.label,
            data: dataset.data.map(e => e.lap_time),
            backgroundColor: COLOURS[i],
        }
    });

    const datasets = [];
    let lapIndex = 1
    for (let i = 0; i < maxLaps; i++) {
        datasets.push({
            label: "Lap " + lapIndex,
            data: data.datasets.map(e => e.data[i]),
            backgroundColor: COLOURS[i],
        })

        lapIndex++;
    }


    new Chart(canvas, {
        type: 'bar',
        data: {
            labels: data.labels,
            datasets: datasets
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
            aspectRatio: 32 / 9,
            scales: {
                x: {
                    stacked: true,
                    grid: {
                        display: false,
                    },
                    title: {
                        display: true,
                        text: 'Time in heat (s)',
                    },
                },
                y: {
                    stacked: true,
                    ticks: {
                        stepSize: 10
                    }
                },
            },
        },
    });

    return chart;
}