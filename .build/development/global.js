/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (function() { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./assets/js/global.js":
/*!*****************************!*\
  !*** ./assets/js/global.js ***!
  \*****************************/
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _utils_lazy_loader__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./utils/lazy_loader */ \"./assets/js/utils/lazy_loader.js\");\n/* harmony import */ var _utils_router__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./utils/router */ \"./assets/js/utils/router.js\");\n\n\n\nconst loader = document.querySelector('#loading')\n\nwindow.addEventListener('router:change', () => {\n    ;(0,_utils_lazy_loader__WEBPACK_IMPORTED_MODULE_0__.default)();\n    Object.assign(loader.style, { transition: 'visibility 100ms ease-out, opacity 100ms ease-out', visibility: 'hidden', opacity: 0 })\n    document.documentElement.style.overflow = 'auto'\n\n    setTimeout(() => {\n        loader.style.transition = null;\n    }, 100)\n\n    const scroll_down = document.querySelector('#scroll-down');\n        \n    if (scroll_down) {\n        scroll_down.addEventListener('click', () => {\n            window.scrollTo({\n                top: window.innerHeight,\n                behavior: 'smooth'\n            });\n        })\n    }\n})\n\ndocument.addEventListener('readystatechange', e => {\n    if (e.target.readyState === 'complete') {\n        (0,_utils_lazy_loader__WEBPACK_IMPORTED_MODULE_0__.default)();\n\n        loader.style.visibility = 'hidden';\n        loader.style.opacity = '0';\n        document.documentElement.style.overflow = 'auto'\n        \n        window.addEventListener('router:loading', () => {\n            Object.assign(loader.style, { opacity: 100, visibility: 'visible' })\n                \n            document.documentElement.style.overflow = 'hidden'\n        })\n\n        window.router = new _utils_router__WEBPACK_IMPORTED_MODULE_1__.default();\n        \n        const scroll_down = document.querySelector('#scroll-down');\n        \n        if (scroll_down) {\n            scroll_down.addEventListener('click', () => {\n                window.scrollTo({\n                    top: window.innerHeight,\n                    behavior: 'smooth'\n                });\n            })\n        }\n        \n        const navbar = document.querySelector('#navbar');\n        const html = document.querySelector('html');\n\n        const close_mobile_menu = () => {\n            navbar.classList.remove('show');\n            // html.classList.remove('overflow_hidden');\n            document.documentElement.style.overflow = null;\n            window.history.pushState(null, null, ' ');\n        }\n        \n        // Open menu\n        document\n            .querySelector('#open-mobile-menu')\n            .addEventListener('click', e => {\n                e.preventDefault();\n                window.history.pushState({ menu_opened: true }, null, '#menu-opened');\n                navbar.classList.add('show');\n                // html.classList.add('overflow_hidden');\n                document.documentElement.style.overflow = 'hidden';\n            })\n        \n        // Close menu\n        document\n            .querySelector('#close-mobile-menu')\n            .addEventListener('click', close_mobile_menu)\n\n        navbar\n            .querySelectorAll('#menu a')\n            .forEach(link => {\n                link\n                    .addEventListener('click', e => {\n                        e.preventDefault();\n        \n                        close_mobile_menu()\n                    })\n            })\n\n        // TODO : finir d'implémenter le nouveau comportement du menu\n        // window.addEventListener('popstate', (e) => {\n        //     console.log(e)\n        //     console.log(window.history)\n        // })\n        \n        // Listen window resizing\n        window.addEventListener('resize', (e) => {\n            if (e.target.innerWidth >= 640 && navbar.classList.contains('show')) {\n                navbar.classList.remove('show');\n                html.classList.remove('overflow_hidden');\n                window.history.pushState(null, null, ' ');\n            }\n        })\n    }\n})\n\n// if (document.readyState === 'interactive' || document.readyState === 'complete') {\n// }\n\n//# sourceURL=webpack://gaf_tests/./assets/js/global.js?");

/***/ }),

/***/ "./assets/js/utils/lazy_loader.js":
/*!****************************************!*\
  !*** ./assets/js/utils/lazy_loader.js ***!
  \****************************************/
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (() => {\n    const lazy_images = [].slice.call(document.querySelectorAll(\".lazy\"));\n\n    console.log(lazy_images)\n\n    if (\"IntersectionObserver\" in window) {\n        let lazy_image_observe = new IntersectionObserver((entries, observer) => {\n            entries.forEach(entry => {\n                console.log(typeof entry, entry.target instanceof HTMLImageElement, entry.target instanceof HTMLPictureElement)\n                if (entry.isIntersecting) {\n                    const lazy_image = entry.target;\n\n                    if (lazy_image instanceof HTMLPictureElement) {\n                        lazy_image\n                            .querySelectorAll('source')\n                            .forEach(source => {\n                                source.srcset = source.dataset.srcset;\n                            })\n                    } else if (lazy_image instanceof HTMLImageElement) {\n                        lazy_image.src = lazy_image.dataset.src;\n                    }\n\n                    lazy_image.classList.remove(\"lazy\");\n                    lazy_image_observe.unobserve(lazy_image);\n                }\n            });\n        });\n\n        lazy_images.forEach(img => {\n            lazy_image_observe.observe(img);\n        });\n    } else {\n      // Possibly fall back to event handlers here\n      lazy_images\n        .forEach(img => {\n            if (lazy_image instanceof HTMLPictureElement) {\n                lazy_image\n                    .querySelectorAll('source')\n                    .forEach(source => {\n                        source.srcset = source.dataset.srcset;\n                    })\n            } else if (lazy_image instanceof HTMLImageElement) {\n                lazy_image.src = lazy_image.dataset.src;\n            }\n        })\n    }\n});\n\n//# sourceURL=webpack://gaf_tests/./assets/js/utils/lazy_loader.js?");

/***/ }),

/***/ "./assets/js/utils/router.js":
/*!***********************************!*\
  !*** ./assets/js/utils/router.js ***!
  \***********************************/
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"default\": function() { return /* binding */ Router; }\n/* harmony export */ });\nclass Router {\n    constructor() {\n        this.cache = {};\n        this.onMountEvent = new Event('onMount');\n        this.onDestroyEvent = new Event('onDestroy');\n        this.parser = new DOMParser();\n\n        const frames = {};\n        for (const frame of document.querySelectorAll('ga-frame')) {\n            frames[frame.id] = frame.innerHTML;\n        }\n\n        // Cache page on which the router has been initialized\n        this.caching(\n            location.href,\n            document.title,\n            frames,\n            [...document.querySelectorAll('script')]\n                .filter(script => script.getAttribute('o-no-load') === null)\n                .map(script => script.attributes.src.nodeValue),\n            [...document.querySelectorAll('head > link[rel=stylesheet]')]\n                .filter(link => link.getAttribute('o-no-load') === null)\n                .map(link => link.attributes.href.nodeValue)\n        );\n\n        console.log('dispatch onMount')\n        window.dispatchEvent(this.onMountEvent);\n\n        window.addEventListener('popstate', e => {\n            window.dispatchEvent(new Event('router:loading', { bubbles: true, cancelable: false }))\n            window.dispatchEvent(this.onDestroyEvent)\n            this.clean(window.history.state ? window.history.state.prevUrl : document.referrer)\n            this.handle_url(location.href)\n        })\n\n        document.body.addEventListener('click', e => {\n            const link = e.target instanceof HTMLAnchorElement\n                ? e.target\n                : e.target.parentElement instanceof HTMLAnchorElement\n                    ? e.target.parentElement\n                    : null\n\n            if (link && link.matches('[o-follow]')) {\n                e.preventDefault()\n\n                window.dispatchEvent(new Event('router:loading', { bubbles: true, cancelable: false }))\n                window.dispatchEvent(this.onDestroyEvent)\n\n                console.info('Remove old hooks', this.cache[location.href])\n                window.removeEventListener('onMount', this.cache[location.href].onMount)\n                window.removeEventListener('onDestroy', this.cache[location.href].onDestroy)\n\n                history.pushState({ prevUrl: location.href }, null, link.href)\n                \n                this.clean(window.history.state.prevUrl)\n                this.handle_url(link.href, true, true)\n            }\n        })\n\n        const links_preload = document.querySelectorAll('[o-preload]')\n        const links_preload_once = document.querySelectorAll('[o-preload-once]')\n\n        // Warn developer an unintended behavior may occur\n        for (const link_preload_once of links_preload_once) {\n            for (const link_preload of links_preload) {\n                if (link_preload_once == link_preload) {\n                    console.warn('A link has [o-preload-once] and [o-preload] tags at the same time')\n                }\n            }\n        }\n        \n        for (const link of links_preload) {\n            link.addEventListener('mouseover', () => {\n                this.handle_url(link.href, true, false)\n            })\n            link.addEventListener('mouseleave', e => {\n                // delete pages[e.target.href]\n            })\n        }\n        \n        for (const link of links_preload_once) {\n            link.addEventListener('mouseover', () => {\n                if (!this.cache[link.href])\n                    this.handle_url(link.href, true, false)\n            })\n        }\n    }\n\n    onMount(callback) {\n        this.cache[location.href].onMount = callback\n        console.log('CREATE ONMOUNT', this.cache[location.href])\n        window.addEventListener('onMount', this.cache[location.href].onMount)\n    }\n\n    onDestroy(callback) {\n        this.cache[location.href].onDestroy = callback\n        window.addEventListener('onDestroy', this.cache[location.href].onDestroy)\n    }\n\n    caching(url, title, frames, scripts, links) {\n        this.cache[url] = {\n            frames,\n            title,\n            last_loaded_time: Date.now(),\n            scripts,\n            links\n        }\n    }\n\n    async handle_url(url, add_in_cache = true, do_rendering = true) {\n        let page = this.cache[url]\n\n        // console.info(`${page ? 'Load cached fragment' : 'Load new fragment'}; URL: ${url}; Do rendering: ${do_rendering}`)\n\n        if (!page) {\n            let res = await fetch(url)\n\n            if (res.ok) {\n                res = await res.text()\n                let parsed_fragment = this.parse(res)\n        \n                if (add_in_cache) {\n                    this.caching(\n                        url,\n                        parsed_fragment.title,\n                        parsed_fragment.frames,\n                        parsed_fragment.scripts,\n                        parsed_fragment.links\n                    )\n                }\n        \n                page = parsed_fragment\n            }\n        }\n\n        page.last_loaded_time = Date.now()\n\n        if (do_rendering === true)\n            this.render(page)\n    }\n\n    parse(content) {\n        const doc = this.parser.parseFromString(content, 'text/html')\n        const frames = {};\n\n        for (const frame of doc.querySelectorAll('ga-frame')) {\n            frames[frame.id] = frame.innerHTML;\n        }\n\n        return {\n            title: doc.title,\n            frames,\n            scripts: [...doc.querySelectorAll('script')]\n                .filter(script => script.getAttribute('o-no-load') === null)\n                .map(script => script.attributes.src.nodeValue),\n            links: [...doc.querySelectorAll('head > link[rel=stylesheet]')]\n                .filter(link => link.getAttribute('o-no-load') === null)\n                .map(link => link.attributes.href.nodeValue)\n        }\n    }\n\n    render(page) {\n        console.log('render')\n        let loadings = []\n\n        // window.dispatchEvent(new Event('router:loading'))\n\n        page.scripts.forEach(newScript => {\n            loadings.push(\n                new Promise((resolve, reject) => {\n                    let script = document.createElement('script')\n                    script.type = 'text/javascript'\n                    script.src = newScript\n                    script.onload = resolve\n                    document.head.appendChild(script)\n                })\n            )\n        })\n\n        document.title = page.title\n        window.scrollTo(0, 0);\n\n        page.links.forEach(link => {\n            loadings.push(\n                new Promise((resolve, reject) => {\n                    console.log('Link', link)\n                    let link_el = document.createElement('link');\n                    link_el.rel = 'stylesheet';\n                    link_el.href = link;\n                    link_el.onload = resolve;\n                    document.head.appendChild(link_el);\n                })\n            )\n        })\n\n        Promise.all(loadings)\n            .then(() => {\n                for (let frame of document.querySelectorAll('ga-frame')) {\n                    const page_frame = page.frames[frame.id];\n        \n                    if (page_frame !== undefined) {\n                        frame.innerHTML = page_frame;\n                    }\n                }\n\n                console.log('dispatch onMount2')\n                window.dispatchEvent(this.onMountEvent)\n\n                if (page.onMount) {\n                    console.log('Create onMount listener')\n                    window.addEventListener('onMount', page.onMount)\n                    // window.dispatchEvent(this.onMountEvent)\n                }\n                if (page.onDestroy) window.addEventListener('onDestroy', page.onDestroy)\n        \n                window.dispatchEvent(new Event('router:change'))\n            })\n\n    }\n\n    clean(url) {\n        console.log('clean')\n        const page_to_clear = this.cache[url]\n\n        if (page_to_clear) {\n            // Remove old css files\n            page_to_clear.links.forEach(link => {\n                console.log(link)\n                const link_el = document.head.querySelector(`link[href=\"${link}\"]`);\n                console.log(link_el);\n                link_el.remove()\n                // document.head.removeChild(document.querySelector(`link[href=\"${link}\"]`)\n            })\n    \n            console.log(`Clean: ${url}`, page_to_clear)\n        \n            page_to_clear.scripts.forEach(script => {\n                console.log(`remove ${script.src} js file`, script)\n                const script_el = document.head.querySelector(`script[src=\"${script}\"]`)\n                console.log(script_el)\n                script_el.remove()\n            })\n        }\n\n    }\n}\n\n//# sourceURL=webpack://gaf_tests/./assets/js/utils/router.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	!function() {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = function(exports, definition) {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	!function() {
/******/ 		__webpack_require__.o = function(obj, prop) { return Object.prototype.hasOwnProperty.call(obj, prop); }
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	!function() {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = function(exports) {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	}();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./assets/js/global.js");
/******/ 	
/******/ })()
;