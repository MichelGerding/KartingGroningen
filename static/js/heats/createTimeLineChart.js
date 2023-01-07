function createTimeLineChart(data, ctx) {
    console.log(data)
    const medianLaps = Object.entries(data.median_day).map(([key, value]) => {
        return {
            date: key,
            lapTime: value
        }
    });
    const avgLaps = Object.entries(data.avg_day).map(([key, value]) => {
       return {
           date: key,
           lapTime: value
       }
    });
    const minLaps = Object.entries(data.minimum_day).map(([key, value]) => {
        return {
            date: key,
            lapTime: value
        }
    });

    const colorRangeInfo = {
        colorStart: 0.2,
        colorEnd: 1,
        useEndAsStart: false,
    }
    // get the amount of unique dirvers
    const datasets = [
        {
            label: "Average Lap Times",
            data: avgLaps.sort(
                (a, b) => new Date(a.date) - new Date(b.date)
            ).map(e => e.lapTime),
        },
        {
            label: "Median Lap Times",
            data: medianLaps.sort(
                (a, b) => new Date(a.date) - new Date(b.date)
            ).map(e => e.lapTime),
        }, {
            label: "Minimum Lap Times",
            data: minLaps.sort(
                (a, b) => new Date(a.date) - new Date(b.date)
            ).map(e => e.lapTime),
        }
    ]

    console.log(datasets)

    return new Chart(ctx, {
        type: 'line',
        data: {
            labels: medianLaps.sort((a, b) => {
                console.log(a, b)
                var aDate = new Date(a.date);
                var bDate = new Date(b.date);
                return aDate - bDate;
            }).map(e => e.date),
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
                    max: 100,
                    grid: {color: "rgb(200,200,200)"},
                },
                x: {
                    type: "time",
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