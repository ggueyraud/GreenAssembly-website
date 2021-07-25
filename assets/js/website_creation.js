import SwiperCore, { Pagination } from 'swiper/core';
import Swiper from 'swiper';

const on_mount = () => {
    const swiper_options = {
        grabCursor: true,
        pagination: {
            el: '.swiper-pagination',
            clickable: true
        }
    };

    SwiperCore.use([Pagination]);
    new Swiper(
        '#formules .swiper-container',
        Object.assign({
            slidesPerView: 1,
            spaceBetween: 60
        }, swiper_options)
    );
    new Swiper(
        '#what_we_do .swiper-container',
        Object.assign({
            slidesPerView: 1,
            spaceBetween: 30,
            breakpoints: {
                768: {
                    slidesPerView: 2
                },
                1280: {
                    slidesPerView: 3,
                    spaceBetween: 60,
                }
            }
        }, swiper_options)
    );

    console.log(document.querySelectorAll('.stepper .stepper__wrapper__step')[0].getBoundingClientRect().height)
    
    import('../wasm/pkg/wasm_bg.js')
        .then(wasm => {
            console.log(wasm.Options.new(true))
            console.log(document.querySelectorAll('.stepper .stepper__wrapper__step')[0].getBoundingClientRect().height)
            const stepper = wasm.Stepper.new(document.querySelector('.stepper'), wasm.Options.new(true));

            stepper.on(wasm.Event.StepChange, (index) => {
                console.log('Change', index);
                const item = document.querySelector(`.stepper__nav a:nth-child(${index+1}n+1) div`);
                document.querySelector('.stepper .label').innerHTML = item.innerHTML
                console.log('Item', item);
            })
        })
}

window.addEventListener('onMount', on_mount)
window.addEventListener('router:change', on_mount)
window.addEventListener('onDestroy', () => {
    window.removeEventListener('onMount', on_mount);
    window.removeEventListener('router:change', on_mount)
})