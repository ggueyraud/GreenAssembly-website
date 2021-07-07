export default class Router {
    constructor() {
        this.cache = {};
        this.onMountEvent = new Event('onMount');
        this.onDestroyEvent = new Event('onDestroy');
        this.parser = new DOMParser();

        const frames = {};
        for (const frame of document.querySelectorAll('ga-frame')) {
            frames[frame.id] = frame.innerHTML;
        }

        // Cache page on which the router has been initialized
        this.caching(
            location.href,
            document.title,
            frames,
            [...document.querySelectorAll('script')]
                .filter(script => script.getAttribute('o-no-load') === null)
                .map(script => script.attributes.src.nodeValue),
            [...document.querySelectorAll('head > link[rel=stylesheet]')]
                .filter(link => link.getAttribute('o-no-load') === null)
                .map(link => link.attributes.href.nodeValue)
        );

        console.log('dispatch onMount')
        window.dispatchEvent(this.onMountEvent);

        window.addEventListener('popstate', e => {
            window.dispatchEvent(new Event('router:loading', { bubbles: true, cancelable: false }))
            window.dispatchEvent(this.onDestroyEvent)
            this.clean(window.history.state ? window.history.state.prevUrl : document.referrer)
            this.handle_url(location.href)
        })

        document.body.addEventListener('click', e => {
            const link = e.target instanceof HTMLAnchorElement
                ? e.target
                : e.target.parentElement instanceof HTMLAnchorElement
                    ? e.target.parentElement
                    : null

            if (link && link.matches('[o-follow]')) {
                e.preventDefault()

                window.dispatchEvent(new Event('router:loading', { bubbles: true, cancelable: false }))
                window.dispatchEvent(this.onDestroyEvent)

                console.info('Remove old hooks', this.cache[location.href])
                window.removeEventListener('onMount', this.cache[location.href].onMount)
                window.removeEventListener('onDestroy', this.cache[location.href].onDestroy)

                history.pushState({ prevUrl: location.href }, null, link.href)
                
                this.clean(window.history.state.prevUrl)
                this.handle_url(link.href, true, true)
            }
        })

        const links_preload = document.querySelectorAll('[o-preload]')
        const links_preload_once = document.querySelectorAll('[o-preload-once]')

        // Warn developer an unintended behavior may occur
        for (const link_preload_once of links_preload_once) {
            for (const link_preload of links_preload) {
                if (link_preload_once == link_preload) {
                    console.warn('A link has [o-preload-once] and [o-preload] tags at the same time')
                }
            }
        }
        
        for (const link of links_preload) {
            link.addEventListener('mouseover', () => {
                this.handle_url(link.href, true, false)
            })
            link.addEventListener('mouseleave', e => {
                // delete pages[e.target.href]
            })
        }
        
        for (const link of links_preload_once) {
            link.addEventListener('mouseover', () => {
                if (!this.cache[link.href])
                    this.handle_url(link.href, true, false)
            })
        }
    }

    onMount(callback) {
        this.cache[location.href].onMount = callback
        console.log('CREATE ONMOUNT', this.cache[location.href])
        window.addEventListener('onMount', this.cache[location.href].onMount)
    }

    onDestroy(callback) {
        this.cache[location.href].onDestroy = callback
        window.addEventListener('onDestroy', this.cache[location.href].onDestroy)
    }

    caching(url, title, frames, scripts, links) {
        this.cache[url] = {
            frames,
            title,
            last_loaded_time: Date.now(),
            scripts,
            links
        }
    }

    async handle_url(url, add_in_cache = true, do_rendering = true) {
        let page = this.cache[url]

        // console.info(`${page ? 'Load cached fragment' : 'Load new fragment'}; URL: ${url}; Do rendering: ${do_rendering}`)

        if (!page) {
            let res = await fetch(url)

            if (res.ok) {
                res = await res.text()
                let parsed_fragment = this.parse(res)
        
                if (add_in_cache) {
                    this.caching(
                        url,
                        parsed_fragment.title,
                        parsed_fragment.frames,
                        parsed_fragment.scripts,
                        parsed_fragment.links
                    )
                }
        
                page = parsed_fragment
            }
        }

        page.last_loaded_time = Date.now()

        if (do_rendering === true)
            this.render(page)
    }

    parse(content) {
        const doc = this.parser.parseFromString(content, 'text/html')
        const frames = {};

        for (const frame of doc.querySelectorAll('ga-frame')) {
            frames[frame.id] = frame.innerHTML;
        }

        return {
            title: doc.title,
            frames,
            scripts: [...doc.querySelectorAll('script')]
                .filter(script => script.getAttribute('o-no-load') === null)
                .map(script => script.attributes.src.nodeValue),
            links: [...doc.querySelectorAll('head > link[rel=stylesheet]')]
                .filter(link => link.getAttribute('o-no-load') === null)
                .map(link => link.attributes.href.nodeValue)
        }
    }

    render(page) {
        console.log('render')
        let loadings = []

        // window.dispatchEvent(new Event('router:loading'))

        page.scripts.forEach(newScript => {
            loadings.push(
                new Promise((resolve, reject) => {
                    let script = document.createElement('script')
                    script.type = 'text/javascript'
                    script.src = newScript
                    script.onload = resolve
                    document.head.appendChild(script)
                })
            )
        })

        document.title = page.title
        window.scrollTo(0, 0);

        page.links.forEach(link => {
            loadings.push(
                new Promise((resolve, reject) => {
                    console.log('Link', link)
                    let link_el = document.createElement('link');
                    link_el.rel = 'stylesheet';
                    link_el.href = link;
                    link_el.onload = resolve;
                    document.head.appendChild(link_el);
                })
            )
        })

        Promise.all(loadings)
            .then(() => {
                for (let frame of document.querySelectorAll('ga-frame')) {
                    const page_frame = page.frames[frame.id];
        
                    if (page_frame !== undefined) {
                        frame.innerHTML = page_frame;
                    }
                }

                console.log('dispatch onMount2')
                window.dispatchEvent(this.onMountEvent)

                if (page.onMount) {
                    console.log('Create onMount listener')
                    window.addEventListener('onMount', page.onMount)
                    // window.dispatchEvent(this.onMountEvent)
                }
                if (page.onDestroy) window.addEventListener('onDestroy', page.onDestroy)
        
                window.dispatchEvent(new Event('router:change'))
            })

    }

    clean(url) {
        console.log('clean')
        const page_to_clear = this.cache[url]

        if (page_to_clear) {
            // Remove old css files
            page_to_clear.links.forEach(link => {
                console.log(link)
                const link_el = document.head.querySelector(`link[href="${link}"]`);
                console.log(link_el);
                link_el.remove()
                // document.head.removeChild(document.querySelector(`link[href="${link}"]`)
            })
    
            console.log(`Clean: ${url}`, page_to_clear)
        
            page_to_clear.scripts.forEach(script => {
                console.log(`remove ${script.src} js file`, script)
                const script_el = document.head.querySelector(`script[src="${script}"]`)
                console.log(script_el)
                script_el.remove()
            })
        }

    }
}