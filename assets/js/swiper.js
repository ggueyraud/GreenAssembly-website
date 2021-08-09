// import SwiperCore, { Pagination } from 'swiper/core';
// import Swiper from 'swiper';

import Carousel, { CarouselPagination, CarouselTouch } from './components/carousel';

const on_mount = () => {
    const carousel = new Carousel(document.querySelector('.carousel'), {
        // slides_visible: 3,
        // auto_height: true,
        breakpoints: {
            768: {
                slides_visible: 2
            },
            1280: {
                slides_visible: 3
            }
        }
    });
    carousel.use([CarouselTouch]);
    carousel.use(CarouselPagination);
    // SwiperCore.use([Pagination]);
    // new Swiper('.swiper-container', {
    //     grabCursor: true,
    //     spaceBetween: 30,
    //     pagination: {
    //         el: '.swiper-pagination',
    //         clickable: true
    //     },
    //     breakpoints: {
    //         768: {
    //             slidesPerView: 2,
    //             spaceBetween: 60
    //         },
    //         1280: {
    //             slidesPerView: 3,
    //             watchOverflow: true,
    //             grabCursor: false,
    //             spaceBetween: 60,
    //         },
    //         1536: {
    //             slidesPerView: 3,
    //             watchOverflow: true,
    //             grabCursor: false,
    //             spaceBetween: 60,
    //         }
    //     }
    // });
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