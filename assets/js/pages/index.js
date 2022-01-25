import Carousel, { CarouselPagination, CarouselTouch } from 'carousel';

const on_mount = () => {
    // Carousel
    const carousel = new Carousel(document.querySelector('.carousel'), {
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

    // Vidéo
    let mouseover_tick = false;
    let touch_event = false;
    const video = document.getElementById('motion_home_video');
    const actions = document.getElementById('motion_home_actions');
    const play_button = document.getElementById('motion_home_video_button');
    
    if(!video) return

    play_button?.addEventListener('click', () => {
        touch_event = false;
        if(video.currentTime > 0 && !video.ended && !video.paused) {
            video.pause();

            actions.classList.remove('video-actions--played');
            return
        }

        video.play();
        actions.classList.remove('video-actions--ended');
        actions.classList.add('video-actions--played');
        actions.classList.add('video-actions--hide');
    });
    video.addEventListener('ended', () => {
        actions.classList.remove('video-actions--played');
        actions.classList.add('video-actions--ended');
    });
    actions.addEventListener('touchstart', () => {
        actions.classList.toggle('video-actions--hide');
        touch_event = true;
    }, { passive: true });
    
    actions.addEventListener('mouseover', () => {
        if(touch_event) return
        
        if(!mouseover_tick) {
            setTimeout(() => {
                actions.classList.remove('video-actions--hide');
                mouseover_tick = false;
            }, 250);
        }

        mouseover_tick = true;
    }, false);
}
window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => window.removeEventListener('onMount', on_mount))