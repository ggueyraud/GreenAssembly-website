const on_mount = () => {
    document
        .querySelectorAll('article header')
        .forEach(header => header.addEventListener('click', () => {
            header.setAttribute(
                'aria-expanded',
                header.getAttribute('aria-expanded') === 'false'
            );
        }));
}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => window.removeEventListener('onMount', on_mount))