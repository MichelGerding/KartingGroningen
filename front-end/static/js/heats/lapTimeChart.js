function lapTimeChart(data, ctx) {
    data = JSON.parse(JSON.stringify(data))

    const colorRangeInfo = {
        colorStart: 0.2,
        colorEnd: 1,
        useEndAsStart: false,
    }
    const COLOURS = interpolateColors(data.labels.length, d3.interpolateTurbo, colorRangeInfo, )


    const maxLaps = Math.max(...data.datasets.map(e => e.data.length));

    data.datasets = data.datasets.map((dataset, i) => {
        return {
            label: dataset.label,
            data: dataset.data.map(e => e.lap_time),
            fill: false,
            borderColor: COLOURS[i],
            backgroundColor: COLOURS[i],
            tension: 0.1,
        }
    });

    const avgLapTimes = []
    for (let i=0; i < maxLaps; i++) {
        let totalLaptime = 0;
        let driverCount = 0;
        data.datasets.forEach((dataset) => {
            if (dataset.data[i]) {
                driverCount++;
                totalLaptime += dataset.data[i];
            }
        });

        avgLapTimes.push(totalLaptime / driverCount);
    }

    let avgColour = 'rgb(0,0,0)';
    let textColour = 'rgb(0,0,0)';
    let linesColour = 'rgba(0,0,0, 0.5)';
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        avgColour = 'rgb(255, 255, 255)';
        textColour = 'rgb(255, 255, 255)';
        linesColour = 'rgba(255,255,255, 0.75)';
    }

    data.datasets.push({
        label: 'Average',
        data: avgLapTimes,
        borderColor: avgColour,
        backgroundColor: avgColour,
        borderWidth: 3,
        tension: 0.3,
        fill: false,
    });


    return new Chart(ctx, {
        type: 'line',
        data: {
            labels: Array.from(Array(maxLaps).keys()).map(e => e + 1),
            tooltipText: Array.from(Array(maxLaps).keys()).map(e => "Lap " + (e + 1)),
            datasets: data.datasets
        },
        options: {
            legend: {
                labels: {
                    fontColor: textColour,
                }
            },

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
                y: {
                    title: {
                        display: true,
                        text: 'Laptimes (s)',
                        color: textColour,
                    },
                    beginAtZero: false,
                    suggestedMin: 45,
                    grid: {color: linesColour},
                },
                x: {
                    title: {
                        display: true,
                        text: 'Lap in heat',
                        color: textColour,
                    },
                    grid: {color: linesColour},
                }
            },
            tooltips: {
                enabled: true,
                callbacks: {
                    title: function(tooltipItem, data) {
                        return data.tooltipText[tooltipItem[0].index];
                    },
                    label: function(tooltipItem, data) {
                        return tooltipItem.xLabel+' pages';
                    }
                },
            }
        }
    });

}