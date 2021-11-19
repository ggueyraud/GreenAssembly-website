let mouseover_tick = false;

function init() {
    const video = document.getElementById('motion_home_video');
    const actions = document.getElementById('motion_home_actions');
    const play_button = document.getElementById('motion_home_video_button');
    
    if(!video) return

    play_button?.addEventListener('click', () => {
        if(video.currentTime > 0 && !video.ended && !video.paused) {
            video.pause();

            actions.classList.remove('video-actions--played');
            return
        }

        video.play();
        actions.classList.remove('video-actions--ended');
        actions.classList.add('video-actions--played');
        actions.classList.add('video-actions--hide');
        video.addEventListener('ended', () => {
            actions.classList.remove('video-actions--played');
            actions.classList.add('video-actions--ended');
        });
        actions.addEventListener('touchstart', () => {
            actions.classList.add('video-actions--hide');
        }, false);
        actions.addEventListener('touchend', () => {
            actions.classList.remove('video-actions--hide');
        }, false);
        actions.addEventListener('mouseover', () => {
            if(!mouseover_tick) {
                setTimeout(() => {
                    actions.classList.remove('video-actions--hide');
                    mouseover_tick = false;
                }, 250);
            }

            mouseover_tick = true;
        }, false);
    });
}

const on_mount = () => {
    init();
}
window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => window.removeEventListener('onMount', on_mount))