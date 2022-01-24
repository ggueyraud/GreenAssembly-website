import Form, { Required, Regex } from 'formvalidation';
import { get } from '@js/utils/http';

const on_mount = () => {
    get(`/api/metrics/views?start=2022-01-24`, {
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        validate_status: status => status === 200,
        body: new URLSearchParams(body)
    })
    .then(async res => {
        res = await res.json();
        
        console.log(res)
    })

    // const data = {
    //     labels: [
    //       'January',
    //       'February',
    //       'March',
    //       'April'
    //     ],
    //     datasets: [{
    //       type: 'bar',
    //       label: 'Bar Dataset',
    //       data: [10, 20, 30, 40],
    //       borderColor: 'rgb(255, 99, 132)',
    //       backgroundColor: 'rgba(255, 99, 132, 0.2)'
    //     }, {
    //       type: 'line',
    //       label: 'Line Dataset',
    //       data: [50, 50, 50, 50],
    //       fill: false,
    //       borderColor: 'rgb(54, 162, 235)'
    //     }]
    //   };

    // const config = {
    //     type: 'scatter',
    //     data: data,
    //     options: {
    //       scales: {
    //         y: {
    //           beginAtZero: true
    //         }
    //       }
    //     }
    //   };
    //   const config = {
    //     type: 'scatter',
    //     data: data,
    //     options: {
    //       scales: {
    //         y: {
    //           beginAtZero: true
    //         }
    //       }
    //     }
    //   };
}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => {
    window.removeEventListener('onMount', on_mount);
});