import { get } from '@js/utils/http';
import Chart from 'chart.js/auto';


const on_mount = () => {
    // Chart.register(BarController, CategoryScale, LinearScale, BarElement, LineController, PointElement, LineElement, Tooltip, Title, ScaleOptions);
    const data = {};

    get(
        `/api/metrics/views-page?start=2022-01-22&end=2022-01-24`,
        { validate_status: status => status === 200 }
    )
    .then(async res => {
        res = await res.json();

        const unique = (value, index, self) => self.indexOf(value) === index;

        data.labels = res
            .map(value => value.date)
            .filter(unique);

        const datasets = [];
        const colors = [
            '#0d6efd',
            '#6610f2',
            '#6f42c1',
            '#d63384',
            '#dc3545',
            '#fd7e14',
            '#ffc107',
            '#198754',
            '#20c997',
            '#0dcaf0',
            '#fff',
            '#6c757d',
        ];

        res.forEach((value, index) => {
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
                        backgroundColor: colors[datasets.length]
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
            borderColor: 'rgb(54, 162, 235)'
        });
        data.datasets.push({
            type: 'line',
            label: 'Total des vues',
            data: primary_datasets
                .map(dataset => dataset.data.reduce((previousValue, currentValue) => previousValue + currentValue)),
            fill: false,
            borderColor: '#ec980a'
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
                        backgroundColor: '#fff'
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
                        grid: {
                            color: 'rgba(255, 255, 255, .5)'
                        },
                        ticks: {
                            color: 'rgba(255, 255, 255, .5)'
                        }
                    },
                    y: {
                        grid: {
                            color: 'rgba(255, 255, 255, .5)'
                        },
                        ticks: {
                            color: 'rgba(255, 255, 255, .5)'
                        },
                        beginAtZero: true,
                        // max: 10
                    }
                }
            }
        });
    })

}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => {
    window.removeEventListener('onMount', on_mount);
});