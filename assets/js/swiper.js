import SwiperCore, { Pagination } from 'swiper/core';
import Swiper from 'swiper';

const on_mount = () => {
    console.log('onmount')
    SwiperCore.use([Pagination]);
    new Swiper('.swiper-container', {
        grabCursor: true,
        spaceBetween: 30,
        pagination: {
            el: '.swiper-pagination',
            clickable: true
        },
        breakpoints: {
            768: {
                slidesPerView: 2,
                spaceBetween: 60
            },
            1280: {
                slidesPerView: 3,
                watchOverflow: true,
                grabCursor: false,
                spaceBetween: 60,
            },
            1536: {
                slidesPerView: 3,
                watchOverflow: true,
                grabCursor: false,
                spaceBetween: 60,
            }
        }
    });
}

const on_destroy = () => {
    console.log('onDestroy')
    window.removeEventListener('onMount', on_mount)
    window.removeEventListener('router:change', on_mount)
    window.removeEventListener('onDestroy', on_destroy)
}

window.addEventListener('onMount', on_mount)
// window.addEventListener('router:change', on_mount)
window.addEventListener('onDestroy', on_destroy)

// document.addEventListener('readystatechange', e => {
//     const ready_state = e.target.readyState;

//     if (ready_state === 'interactive' || ready_state === 'complete') {
//     }
// });