const on_mount = () => {
    console.log('Lorem ipsum dolor sit amet')

}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => window.removeEventListener('onMount', on_mount))