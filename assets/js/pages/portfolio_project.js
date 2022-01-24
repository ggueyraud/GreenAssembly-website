import Carousel, { CarouselPagination, CarouselTouch } from 'carousel';
import LightBox from '@js/components/lightbox';

let lightbox = null;

const on_mount = () => {
    lightbox = new LightBox('section img');
    const carousel_formules = new Carousel(document.querySelector('.carousel'), {
        breakpoints: {
            768: {
                slides_visible: 2
            },
            1280: {
                slides_visible: 3
            }
        }
    });
    // Prevent lightbox be active on carousel move
    carousel_formules.on('startDrag', () => {
        lightbox.disabled = true;
    });
    carousel_formules.on('endDrag', () => {
        setTimeout(() => lightbox.disabled = false, 250);
    });
    carousel_formules.use([CarouselPagination, CarouselTouch]);
}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => {
    lightbox.close();
    window.removeEventListener('onMount', on_mount);
});