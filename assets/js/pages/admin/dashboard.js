import { get } from '@js/utils/http';
import Chart from 'chart.js/auto';

const DounghnutChart = (element, data, options = {}) => {
    options.plugins = Object.assign({
        tooltip: {
            titleColor: '#000',
            bodyColor: '#000',
            backgroundColor: '#fff',
            callbacks: {
                label: ctx => `${ctx.label} : ${ctx.parsed.toFixed(2)}%`
            }
        },
        legend: {
            position: 'bottom',
        },
        title: {
            display: true,
            color: '#fff',
            // text: 'Répartition par devices'
        }
    }, options.plugins);

    options = Object.assign({
        responsive: true,
        scaleFontColor: '#fff'
        
    }, options);

    console.log(
        options
        // Object.assign({
        //     tooltip: {
        //         titleColor: '#000',
        //         bodyColor: '#000',
        //         backgroundColor: '#fff',
        //         callbacks: {
        //             label: ctx => `${ctx.label} : ${ctx.parsed.toFixed(2)}%`
        //         }
        //     },
        //     legend: {
        //         position: 'bottom',
        //     },
        //     title: {
        //         display: true,
        //         color: '#fff',
        //         text: 'Répartition par devices'
        //     }
        // }, options.plugins)
    )

    return new Chart(element.getContext('2d'), {
        type: 'doughnut',
        data,
        options
    });
}


const on_mount = () => {
    const colors = [
        '13, 110, 253',
        '220, 53, 69',
        '25, 135, 84',
        '102, 16, 242',
        '214, 51, 132',
        '255, 193, 7',
        '32, 201, 151',
        '111, 66, 193',
        '13, 202, 240',
        '255, 255, 255',
        '108, 117, 125'
    ];
    const data = {};

    get(`/api/metrics/views-page?start=2022-01-22&end=2022-01-25`)
    .then(async res => {
        res = await res.json();

        const unique = (value, index, self) => self.indexOf(value) === index;

        data.labels = res
            .map(value => value.date)
            .filter(unique);

        const datasets = [];

        res.forEach(value => {
            const dataset = datasets.find(dataset => dataset.label === value.title);

            if (value.views > 0) {

                if (dataset) {
                    dataset.data.push(value.views);
                } else {
                    const new_dataset = {
                        type: 'bar',
                        label: value.title,
                        data: [value.views],
                        borderRadius: 6,
                        borderWidth: 2,
                        borderColor: `rgb(${colors[datasets.length]})`,
                        backgroundColor: `rgba(${colors[datasets.length]}, .45`
                    };
    
                    datasets.push(new_dataset);
                }
            }
        });

        const primary_datasets = datasets.filter(dataset => dataset.data.length > 0);
        data.datasets = [];
        data.datasets.push({
            type: 'line',
            label: 'Moyenne des vues sur la journée',
            data: primary_datasets.map(dataset => {
                let sum = dataset.data.reduce((prev, current) => prev + current);
                console.log(sum)
    
                return sum / dataset.data.length
            }),
            fill: false,
            borderColor: 'rgb(54, 162, 235)',
            borderDash: [5, 5],
            tension: 0.3
        });
        data.datasets.push({
            type: 'line',
            label: 'Total des vues',
            data: primary_datasets
                .map(dataset => dataset.data.reduce((previousValue, currentValue) => previousValue + currentValue)),
            fill: false,
            borderColor: '#ec980a',
            tension: 0.3
        });
        data.datasets = [...data.datasets, ...primary_datasets];

        console.log(data.datasets)

        new Chart(document.getElementById('myChart').getContext('2d'), {
            type: 'bar',
            data,
            options: {
                type: 'scatter',
                data: data,
                responsive: true,
                scaleFontColor: '#fff',
                plugins: {
                    tooltip: {
                        titleColor: '#000',
                        bodyColor: '#000',
                        backgroundColor: '#fff',
                        mode: 'index',
                        intersect: true
                    },
                    legend: {
                        position: 'bottom',
                    },
                    title: {
                        display: true,
                        color: '#fff',
                        text: 'Nombre de vues par page'
                    }
                },
                scales: {
                    x: {
                        title: {
                            text: 'Dates',
                            color: '#fff',
                            display: true
                        },
                        grid: {
                            color: 'rgba(255, 255, 255, .5)'
                        },
                        ticks: {
                            color: 'rgba(255, 255, 255, .5)'
                        }
                    },
                    y: {
                        type: 'logarithmic',
                        // max: 67,
                        // display: false,
                        title: {
                            text: 'Nombre de vues',
                            color: '#fff',
                            display: true
                        },
                        grid: {
                            color: 'rgba(255, 255, 255, .5)'
                        },
                        ticks: {
                            // stepSize: 25,
                            color: 'rgba(255, 255, 255, .5)'
                        },
                        beginAtZero: true
                    }
                }
            }
        });
    });

    get(`/api/metrics/devices?start=2022-01-22&end=2022-01-25`)
    .then(async res => {
        res = await res.json();
        let c = colors.sort(() => Math.random() - 0.5);

        DounghnutChart(
            document.getElementById('devices'),
            {
                labels: res.map(value => value.name),
                datasets: [
                    {
                        data: res.map(value => value.percent),
                        backgroundColor: res.map((_, index) => `rgba(${c[index]}, .45)`),
                        borderColor: res.map((_, index) => `rgb(${c[index]}, .45)`)
                    }
                ]
            },
            {
                plugins: {
                    title: {
                        display: true,
                        color: '#fff',
                        text: 'Répartition par devices'
                    }
                }
            }
        );
    })

    get(`/api/metrics/os?start=2022-01-22&end=2022-01-25`)
    .then(async res => {
        res = await res.json();
        let c = colors.sort(() => Math.random() - 0.5);

        DounghnutChart(
            document.getElementById('os'),
            {
                labels: res.map(value => value.name),
                datasets: [
                    {
                        data: res.map(value => value.percent),
                        backgroundColor: res.map((_, index) => `rgba(${c[index]}, .45)`),
                        borderColor: res.map((_, index) => `rgb(${c[index]}, .45)`)
                    }
                ]
            },
            {
                plugins: {
                    title: {
                        display: true,
                        color: '#fff',
                        text: 'Répartition par OS'
                    }
                }
            }
        );
    })

    get(`/api/metrics/browsers?start=2022-01-22&end=2022-01-25`)
    .then(async res => {
        res = await res.json();

        let c = colors.sort(() => Math.random() - 0.5);

        DounghnutChart(
            document.getElementById('browsers'),
            {
                labels: res.map(value => value.name),
                datasets: [
                    {
                        data: res.map(value => value.percent),
                        backgroundColor: res.map((_, index) => `rgba(${c[index]}, .45)`),
                        borderColor: res.map((_, index) => `rgb(${c[index]}, .45)`)
                    }
                ]
            },
            {
                plugins: {
                    title: {
                        display: true,
                        color: '#fff',
                        text: 'Répartition par navigateurs'
                    }
                }
            }
        );
    })

}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => {
    window.removeEventListener('onMount', on_mount);
});