const open = (lightbox, e) => {
    if (lightbox.disabled === false) {
        const original = e.target.dataset.original;

        if (original) {
            lightbox.img.onload = () => lightbox.lightbox.classList.add('lightbox--active');

            const fallback = e.target.dataset.fallback;

            if (fallback && !document.documentElement.classList.contains('webp_supportted')) {
                lightbox.img.setAttribute('src', fallback);
            } else {
                lightbox.img.setAttribute('src', original);
            }
        } else {
            lightbox.lightbox.classList.add('lightbox--active');
            lightbox.img.setAttribute('src', e.target.getAttribute('src'));
        }

        document.documentElement.style.overflow = 'hidden';
    }
}

export default class LightBox {
    constructor(targets) {
        this.lightbox = document.querySelector('.lightbox');
        this.img = null;
        this.disabled = false;

        if (!this.lightbox) {
            this.lightbox = document.createElement('div');
            this.lightbox.classList.add('lightbox');
    
            this.img = document.createElement('img');
            this.lightbox.appendChild(this.img);
            this.img.addEventListener('click', this.close.bind(this));
    
            document.querySelector('body').insertAdjacentElement('beforeend', this.lightbox);
    
            this.lightbox.addEventListener('click', this.close.bind(this))
        }

        window.addEventListener('keydown', e => {
            const active_box = document.querySelector('.lightbox--active');
    
            if (active_box && e.key === 'Escape') {
                this.close();
                // hide(active_\box)
            }
        });

        document
            .querySelectorAll(targets)
            .forEach(element => {
                element.addEventListener('click', open.bind(element, this))
            });
    }

    close() {
        this.lightbox.classList.remove('lightbox--active');
        this.img.setAttribute('src', '');
        document.documentElement.style.overflow = 'auto';
    }
}