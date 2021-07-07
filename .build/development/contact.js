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

/***/ "./assets/js/components/form.js":
/*!**************************************!*\
  !*** ./assets/js/components/form.js ***!
  \**************************************/
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

eval("__webpack_require__.r(__webpack_exports__);\nconst validators = {\n    not_empty: value => value.length === 0 ? false : true,\n    string_length: (value, range = {}) => {\n        let valid = true\n        if (range.min !== -1 && value.length < range.min) valid = false\n        if (range.max !== -1 && value.length > range.max) valid = false\n        \n        return valid\n    },\n    less_than: (value, max) => value.length > max,\n    greater_than: (value, min) => value.length < min\n}\n\nclass Field {\n    constructor(elements, validators) {\n        this.validators = validators;\n        this.elements = elements;\n    }\n\n    get_value() {\n        let values = [];\n\n        this.elements.forEach(el => {\n            const { type } = el;\n\n            if (type === 'checkbox' || type === 'radio') {\n                if (el.checked) values.push(el.value);\n            } else {\n                values.push(el.value);\n            }\n        })\n\n        return values.length === 1 && this.elements[0].type !== 'checkbox' ? values[0] : values;\n    }\n\n    validate(display_error = true) {\n        if (Object.keys(this.validators).length === 0) return true;\n\n        let error = null;\n\n        for (const validator_name in this.validators) {\n            const type = this.elements[0].type;\n\n            if ((type === 'checkbox' || type === 'radio') && validator_name === 'notEmpty') {\n                let at_least_one_checked = false;\n\n                this.elements.forEach(el => {\n                    if (el.checked === true) {\n                        at_least_one_checked = true;\n                    }\n                });\n\n                if (!at_least_one_checked) {\n                    error = this.validators[validator_name];\n                }\n            } else if (type === 'text' || type === 'email' || type === 'textarea') {\n                this.elements.forEach(el => {\n                    const { value, minLength, maxLength, name, required } = el;\n\n                    if (value.length > 0 && validator_name === 'stringLength') {\n                        if (!validators.string_length(value, {\n                            min: minLength,\n                            max: maxLength\n                        })) {\n                            error = this.validators[validator_name];\n                        }\n                    } else if (required === true && validator_name === 'notEmpty') {\n                        if (!validators.not_empty(value)) {\n                            error = this.validators[validator_name];\n                        }\n                    }\n                })\n            }\n        }\n\n        if (display_error) {\n            this.update_error(error);\n        }\n\n        return error === null;\n    }\n\n    update_error(error) {\n        let last = this.elements.length === 0\n            ? this.elements[0]\n            : this.elements[this.elements.length - 1];\n        let next_el = this.validators.container\n            ? this.validators.container.nextSibling\n            : last.nextElementSibling;\n\n        if (error) {\n            if (!next_el || (next_el && !next_el.classList.contains('fv-error'))) {\n                next_el = document.createElement('div');\n                next_el.classList.add('fv-error');\n\n                if (this.validators.container) {\n                    this.validators.container.insertAdjacentElement('afterend', next_el);\n                } else {\n                    last.insertAdjacentElement('afterend', next_el);\n                }\n            }\n            \n            next_el.innerHTML = error;\n            next_el.classList.add('active');\n            last.classList.add('border-yellow-500');\n        } else {\n            if (next_el && next_el.classList && next_el.classList.contains('fv-error')) {\n                // next_el.remove();\n                next_el.classList.remove('active');\n                last.classList.remove('border-yellow-500');\n            }\n        }\n    }\n}\n\nconst input_event = (form, fields, current_field) => {\n    let is_form_valid = true;\n    let detail = {}\n\n    if (current_field.validate()) {\n        if (Object.keys(fields).length == 1) {\n            detail[current_field.elements[0].name] = current_field.get_value();\n        } else {\n            for (const f_name in fields) {\n                const f = fields[f_name];\n\n                if (f !== current_field) {\n                    if (!f.validate(false)) {\n                        is_form_valid = false;\n                        return;\n                    }\n\n                    detail[current_field.elements[0].name] = f.get_value();\n                }\n            }\n        }\n    } else {\n        is_form_valid = false;\n    }\n\n    if (is_form_valid) {\n        form.dispatchEvent(new CustomEvent('valid', {\n            detail\n        }));\n    } else {\n        form.dispatchEvent(new CustomEvent('invalid', {\n            detail\n        }));\n    }\n}\n\n/* harmony default export */ __webpack_exports__[\"default\"] = (class {\n    constructor(form, options = {}) {\n        this.el = form;\n        this.fields = {};\n\n        if (options.fields) {\n            for (const field_name in options.fields) {\n                let field = new Field(\n                    form.querySelectorAll(`[name=\"${field_name}\"]`),\n                    options.fields[field_name].validators || {}\n                );\n\n                this.fields[field_name] = field;\n            }\n        } else {\n            const fields = form.querySelectorAll('input, textarea, select');\n\n            fields.forEach(native_field => {\n                let validators = {};\n                let { minLength, maxLength, required, dataset } = native_field;\n\n                if ((minLength !== -1 || maxLength !== -1)) {\n                    validators.stringLength = dataset['fvString_lengthMessage'] || `Cannot be under ${minLength} characters`;\n                }\n\n                if (required === true) {\n                    validators.notEmpty = dataset['fvNot_emptyMessage'] || 'Cannot be empty';\n                }\n\n                let field = new Field(\n                    [native_field],\n                    validators\n                );\n\n                this.fields[native_field.name] = field;\n            })\n        }\n    \n        this.handle_fields()\n\n        const submit = form.querySelector('[type=\"submit\"]');\n\n        if (submit) {\n            submit.addEventListener('click', e => {\n                e.preventDefault();\n        \n                let valid = true;\n                let detail = {};\n\n                for (const field_name in this.fields) {\n                    const field = this.fields[field_name];\n\n                    if (!field.validate(false)) {\n                        valid = false;\n                        return\n                    }\n        \n                    detail[field.elements[0].name] = field.get_value() || null;\n                }\n        \n                if (valid) {\n                    form.dispatchEvent(new CustomEvent('send', {\n                        detail\n                    }))\n                }\n            })\n        }\n    }\n\n    handle_fields() {\n        for (const field_name in this.fields) {\n            const field = this.fields[field_name];\n\n            field.elements.forEach(el => {\n                el.addEventListener('input', () => {\n                    input_event(this.el, this.fields, field)\n                })\n            })\n        }\n\n        // this.fields.forEach(custom_field => {\n        //     custom_field.elements.forEach(el => {\n        //         el.addEventListener('input', () => {\n        //             let is_form_valid = true;\n        //             let detail = {}\n\n        //             if (custom_field.validate()) {\n        //                 if (this.fields.length == 1) {\n        //                     detail[custom_field.elements[0].name] = custom_field.get_value();\n        //                 } else {\n        //                     this.fields.forEach(f => {\n        //                         if (f !== custom_field) {\n        //                             if (!f.validate(false)) {\n        //                                 is_form_valid = false;\n        //                                 return;\n        //                             }\n    \n        //                             detail[custom_field.elements[0].name] = custom_field.get_value();\n        //                         }\n        //                     })\n        //                 }\n        //             } else {\n        //                 is_form_valid = false;\n        //             }\n\n        //             if (is_form_valid) {\n        //                 this.el.dispatchEvent(new CustomEvent('valid', {\n        //                     detail\n        //                 }));\n        //             } else {\n        //                 this.el.dispatchEvent(new CustomEvent('invalid', {\n        //                     detail\n        //                 }));\n        //             }\n        //         })\n        //     })\n        // })\n    }\n\n    on(event_name, callback) {\n        this.el.addEventListener(event_name, callback)\n\n        return this\n    }\n\n    add_field(field) {\n        const type = typeof field;\n        let native_field = null;\n\n        if (type === 'object' && field instanceof HTMLElement) {\n            native_field = field;\n        } else if (type === 'string') {\n            const f = this.el.querySelector(`[name=\"${field}\"]`);\n\n            if (f) {\n                native_field = f;\n            }\n        } else {\n            throw 'Incorrect field specified';\n        }\n\n        let validators = {};\n        let { minLength, maxLength, required, dataset } = native_field;\n\n        if ((minLength !== -1 || maxLength !== -1)) {\n            validators.stringLength = dataset['fvString_lengthMessage'] || `Cannot be under ${minLength} characters`;\n        }\n\n        if (required === true) {\n            validators.notEmpty = dataset['fvNot_emptyMessage'] || 'Cannot be empty';\n        }\n\n        this.fields[native_field.name] = new Field(\n            [native_field],\n            validators\n        );\n    }\n\n    remove_field(field_name) {\n        const field = this.fields[field_name];\n\n        if (field) {\n            field.elements.forEach(el => {\n                // TODO : make sure element is input or textarea\n                el.value = '';\n                el.removeEventListener('input', input_event);\n            });\n            delete this.fields[field_name];\n        }\n    //     this.fields.forEach((field, index) => {\n    //         if (field.name === field_name) {\n    //             field.removeEventListener('input', input_event);\n\n    //             let error = field.nextElementSibling;\n\n    //             if (error && error.classList.contains('fv-error')) {\n    //                 error.remove();\n    //                 field.classList.remove('border-yellow-500');\n    //             }\n\n    //             this.fields.splice(index, 1);\n    //             return;\n    //         }\n    //     })\n    }\n});\n\n//# sourceURL=webpack://gaf_tests/./assets/js/components/form.js?");

/***/ }),

/***/ "./assets/js/contact.js":
/*!******************************!*\
  !*** ./assets/js/contact.js ***!
  \******************************/
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _components_form__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./components/form */ \"./assets/js/components/form.js\");\n/* harmony import */ var _utils_http__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./utils/http */ \"./assets/js/utils/http.js\");\n\n\n\nconst on_mount = () => {\n    console.log('ok')\n    const budget_label = document.querySelector('[for=\"budget\"]');\n        const budget_input = document.querySelector('[name=\"budget\"]');\n        const services_label = document.querySelector('[for=services]');\n        const services = document.querySelector('#services');\n        const company_label = document.querySelector('[for=\"company\"]');\n        const company_input = document.querySelector('[name=\"company\"]');\n        \n        let stepper = null;\n\n        __webpack_require__.e(/*! import() */ \"assets_wasm_pkg_wasm_bg_js\").then(__webpack_require__.bind(__webpack_require__, /*! ../wasm/pkg/wasm_bg.js */ \"./assets/wasm/pkg/wasm_bg.js\"))\n        .then(wasm => {\n            console.log(wasm)\n            stepper = wasm.Stepper.new(document.querySelector('.stepper'));\n            stepper.on(wasm.Event.StepChange, (index) => {\n                if (index === 0 && body.new_project === false) {\n                    \n                    personal_informations_form.add_field(company_input);\n                    project_form.add_field('services');\n        \n                    company_label.classList.remove('hidden');\n                    company_input.classList.remove('hidden');\n                    budget_label.classList.remove('hidden');\n                    budget_input.classList.remove('hidden');\n                    services.classList.remove('hidden');\n                    services_label.classList.remove('hidden');\n                    \n                }\n            })\n        })\n        .catch(console.error)\n        \n        const body = {};\n        \n        document\n            .querySelector('[name=\"message\"]')\n            .addEventListener('input', e => {\n                document.querySelector('#message_caracters_counter').innerHTML = e.target.value.length\n            })\n        \n        const personal_informations_form = new _components_form__WEBPACK_IMPORTED_MODULE_0__.default(document.querySelector('[name=\"informations\"]'))\n            .on('valid', e => {\n                e.target.querySelector('[type=submit]').classList.add('opacity_100');\n            })\n            .on('invalid', e => {\n                e.target.querySelector('[type=submit]').classList.remove('opacity_100');\n            })\n            .on('send', e => {\n                e.preventDefault();\n        \n                Object.assign(body, e.detail);\n        \n                // stepper.next();\n            });\n        const project_form = new _components_form__WEBPACK_IMPORTED_MODULE_0__.default(document.querySelector('[name=\"project\"]'), {\n            fields: {\n                services: {\n                    validators: {\n                        notEmpty: 'Sélectionnez au moins une prestation',\n                        container: document.querySelector('#services')\n                    }\n                },\n                message: {\n                    validators: {\n                        notEmpty: 'Ce champ est obligatoire',\n                        stringLength: 'Le message doit faire entre 30 et 500 caractères',\n                    }\n                },\n                budget: {}\n            }\n        })\n            .on('valid', e => {\n                e.target.querySelector('[type=submit]').classList.add('opacity_100');\n            })\n            .on('invalid', e => {\n                e.target.querySelector('[type=submit]').classList.remove('opacity_100');\n            })\n            .on('send', e => {\n                e.preventDefault();\n        \n                Object.assign(body, e.detail);\n                \n                if (body.budget) body.budget = parseFloat(body.budget) || null\n        \n                ;(0,_utils_http__WEBPACK_IMPORTED_MODULE_1__.post)('/contact', {\n                    headers: {\n                        'Content-Type': 'application/json'\n                    },\n                    validate_status: status => status === 200,\n                    body\n                })\n                    .then(res => {\n                        if (res.ok) {\n                            document.querySelector(\"#success\").classList.remove('hidden');\n                            document.querySelector('.steps').classList.add('hidden');\n                        }\n                    })\n            });\n        \n        // Contact-for forms\n        new _components_form__WEBPACK_IMPORTED_MODULE_0__.default(document.querySelector('[name=\"why-for\"]'), {\n            fields: {\n                why_for: {\n                    validators: {\n                        notEmpty: 'Sélectionnez une __'\n                    }\n                }\n            }\n        })\n            .on('valid', e => {\n                const message_label = document.querySelector('label[for=\"message\"]');\n                document.querySelector('[name=\"lastname\"]').focus({ preventScroll: true });\n        \n                if (e.detail.why_for === 'new_project') {\n                    body.new_project = true;\n        \n                    message_label.innerHTML = 'Description';\n                } else {\n                    body.new_project = false;\n        \n                    personal_informations_form.remove_field('company');\n                    project_form.remove_field('services');\n        \n                    company_label.classList.add('hidden');\n                    company_input.classList.add('hidden');\n                    message_label.innerHTML = 'Message';\n                    budget_label.classList.add('hidden');\n                    budget_input.classList.add('hidden');\n                    services.classList.add('hidden');\n                    services_label.classList.add('hidden');\n                }\n        \n                stepper.next()\n            });\n}\n\nwindow.addEventListener('onMount', on_mount)\nwindow.addEventListener('onDestroy', () => window.removeEventListener('onMount', on_mount))\n\n//# sourceURL=webpack://gaf_tests/./assets/js/contact.js?");

/***/ }),

/***/ "./assets/js/utils/http.js":
/*!*********************************!*\
  !*** ./assets/js/utils/http.js ***!
  \*********************************/
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"get\": function() { return /* binding */ get; },\n/* harmony export */   \"post\": function() { return /* binding */ post; },\n/* harmony export */   \"patch\": function() { return /* binding */ patch; },\n/* harmony export */   \"put\": function() { return /* binding */ put; },\n/* harmony export */   \"del\": function() { return /* binding */ del; }\n/* harmony export */ });\nconst make_request = (method, url, options) => {\n    const init = {\n        method,\n        headers: {}\n    }\n\n    if (options.headers) Object.assign(init.headers, options.headers)\n\n    if (method === 'POST' || method === 'PATCH' || method === 'PUT')\n        init.body =\n            init.headers['Content-Type'] === 'application/json'\n                ? JSON.stringify(options.body)\n                : options.body\n\n    // if (options.base_url !== undefined) base_url = options.base_url\n\n    return fetch(url, init)\n}\n\nconst get = (url, options = {}) =>\n    new Promise((resolve, reject) => {\n        make_request('GET', url, options).then(response => {\n            if (!options.validate_status)\n                options.validate_status = status => status === 200\n\n            if (options.validate_status && !options.validate_status(response.status))\n                reject(response)\n\n            resolve(response)\n        })\n        .catch(e => reject(e))\n    })\n\nconst post = (url, options = {}) =>\n    new Promise((resolve, reject) => {\n        make_request('POST', url, options).then(response => {\n            if (!options.validate_status)\n                options.validate_status = status => status === 201\n\n            if (options.validate_status && !options.validate_status(response.status))\n                reject(response)\n\n            resolve(response)\n        })\n        .catch(e => reject(e))\n    })\n\nconst patch = (url, options = {}) =>\n    new Promise((resolve, reject) => {\n        make_request('PATCH', url, options).then(response => {\n            if (!options.validate_status)\n                options.validate_status = status => status === 200\n\n            if (options.validate_status && !options.validate_status(response.status))\n                reject(response)\n\n            resolve(response)\n        })\n        .catch(e => reject(e))\n    })\n\nconst put = (url, options = {}) =>\n    new Promise((resolve, reject) => {\n        make_request('PUT', url, options).then(response => {\n            if (!options.validate_status)\n                options.validate_status = status => status === 200\n\n            if (options.validate_status && !options.validate_status(response.status))\n                reject(response)\n\n            resolve(response)\n        })\n        .catch(e => reject(e))\n    })\n\nconst del = (url, options = {}) =>\n    new Promise((resolve, reject) => {\n        make_request('DELETE', url, options).then(response => {\n            if (!options.validate_status)\n                options.validate_status = status => status === 200\n\n            if (options.validate_status && !options.validate_status(response.status))\n                reject(response)\n\n            resolve(response)\n        })\n        .catch(e => reject(e))\n    })\n  \n\n//# sourceURL=webpack://gaf_tests/./assets/js/utils/http.js?");

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
/******/ 			id: moduleId,
/******/ 			loaded: false,
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Flag the module as loaded
/******/ 		module.loaded = true;
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = __webpack_module_cache__;
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
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	!function() {
/******/ 		__webpack_require__.f = {};
/******/ 		// This file contains only the entry chunk.
/******/ 		// The chunk loading function for additional chunks
/******/ 		__webpack_require__.e = function(chunkId) {
/******/ 			return Promise.all(Object.keys(__webpack_require__.f).reduce(function(promises, key) {
/******/ 				__webpack_require__.f[key](chunkId, promises);
/******/ 				return promises;
/******/ 			}, []));
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	!function() {
/******/ 		// This function allow to reference async chunks
/******/ 		__webpack_require__.u = function(chunkId) {
/******/ 			// return url for filenames based on template
/******/ 			return "" + chunkId + ".js";
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	!function() {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/harmony module decorator */
/******/ 	!function() {
/******/ 		__webpack_require__.hmd = function(module) {
/******/ 			module = Object.create(module);
/******/ 			if (!module.children) module.children = [];
/******/ 			Object.defineProperty(module, 'exports', {
/******/ 				enumerable: true,
/******/ 				set: function() {
/******/ 					throw new Error('ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: ' + module.id);
/******/ 				}
/******/ 			});
/******/ 			return module;
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	!function() {
/******/ 		__webpack_require__.o = function(obj, prop) { return Object.prototype.hasOwnProperty.call(obj, prop); }
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/load script */
/******/ 	!function() {
/******/ 		var inProgress = {};
/******/ 		var dataWebpackPrefix = "gaf_tests:";
/******/ 		// loadScript function to load a script via script tag
/******/ 		__webpack_require__.l = function(url, done, key, chunkId) {
/******/ 			if(inProgress[url]) { inProgress[url].push(done); return; }
/******/ 			var script, needAttach;
/******/ 			if(key !== undefined) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				for(var i = 0; i < scripts.length; i++) {
/******/ 					var s = scripts[i];
/******/ 					if(s.getAttribute("src") == url || s.getAttribute("data-webpack") == dataWebpackPrefix + key) { script = s; break; }
/******/ 				}
/******/ 			}
/******/ 			if(!script) {
/******/ 				needAttach = true;
/******/ 				script = document.createElement('script');
/******/ 		
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.setAttribute("data-webpack", dataWebpackPrefix + key);
/******/ 				script.src = url;
/******/ 			}
/******/ 			inProgress[url] = [done];
/******/ 			var onScriptComplete = function(prev, event) {
/******/ 				// avoid mem leaks in IE.
/******/ 				script.onerror = script.onload = null;
/******/ 				clearTimeout(timeout);
/******/ 				var doneFns = inProgress[url];
/******/ 				delete inProgress[url];
/******/ 				script.parentNode && script.parentNode.removeChild(script);
/******/ 				doneFns && doneFns.forEach(function(fn) { return fn(event); });
/******/ 				if(prev) return prev(event);
/******/ 			}
/******/ 			;
/******/ 			var timeout = setTimeout(onScriptComplete.bind(null, undefined, { type: 'timeout', target: script }), 120000);
/******/ 			script.onerror = onScriptComplete.bind(null, script.onerror);
/******/ 			script.onload = onScriptComplete.bind(null, script.onload);
/******/ 			needAttach && document.head.appendChild(script);
/******/ 		};
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
/******/ 	/* webpack/runtime/publicPath */
/******/ 	!function() {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript)
/******/ 				scriptUrl = document.currentScript.src
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) scriptUrl = scripts[scripts.length - 1].src
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/jsonp chunk loading */
/******/ 	!function() {
/******/ 		// no baseURI
/******/ 		
/******/ 		// object to store loaded and loading chunks
/******/ 		// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 		// [resolve, reject, Promise] = chunk loading, 0 = chunk loaded
/******/ 		var installedChunks = {
/******/ 			"contact": 0
/******/ 		};
/******/ 		
/******/ 		__webpack_require__.f.j = function(chunkId, promises) {
/******/ 				// JSONP chunk loading for javascript
/******/ 				var installedChunkData = __webpack_require__.o(installedChunks, chunkId) ? installedChunks[chunkId] : undefined;
/******/ 				if(installedChunkData !== 0) { // 0 means "already installed".
/******/ 		
/******/ 					// a Promise means "currently loading".
/******/ 					if(installedChunkData) {
/******/ 						promises.push(installedChunkData[2]);
/******/ 					} else {
/******/ 						if(true) { // all chunks have JS
/******/ 							// setup Promise in chunk cache
/******/ 							var promise = new Promise(function(resolve, reject) { installedChunkData = installedChunks[chunkId] = [resolve, reject]; });
/******/ 							promises.push(installedChunkData[2] = promise);
/******/ 		
/******/ 							// start chunk loading
/******/ 							var url = __webpack_require__.p + __webpack_require__.u(chunkId);
/******/ 							// create error before stack unwound to get useful stacktrace later
/******/ 							var error = new Error();
/******/ 							var loadingEnded = function(event) {
/******/ 								if(__webpack_require__.o(installedChunks, chunkId)) {
/******/ 									installedChunkData = installedChunks[chunkId];
/******/ 									if(installedChunkData !== 0) installedChunks[chunkId] = undefined;
/******/ 									if(installedChunkData) {
/******/ 										var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 										var realSrc = event && event.target && event.target.src;
/******/ 										error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 										error.name = 'ChunkLoadError';
/******/ 										error.type = errorType;
/******/ 										error.request = realSrc;
/******/ 										installedChunkData[1](error);
/******/ 									}
/******/ 								}
/******/ 							};
/******/ 							__webpack_require__.l(url, loadingEnded, "chunk-" + chunkId, chunkId);
/******/ 						} else installedChunks[chunkId] = 0;
/******/ 					}
/******/ 				}
/******/ 		};
/******/ 		
/******/ 		// no prefetching
/******/ 		
/******/ 		// no preloaded
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		// install a JSONP callback for chunk loading
/******/ 		var webpackJsonpCallback = function(parentChunkLoadingFunction, data) {
/******/ 			var chunkIds = data[0];
/******/ 			var moreModules = data[1];
/******/ 			var runtime = data[2];
/******/ 			// add "moreModules" to the modules object,
/******/ 			// then flag all "chunkIds" as loaded and fire callback
/******/ 			var moduleId, chunkId, i = 0;
/******/ 			for(moduleId in moreModules) {
/******/ 				if(__webpack_require__.o(moreModules, moduleId)) {
/******/ 					__webpack_require__.m[moduleId] = moreModules[moduleId];
/******/ 				}
/******/ 			}
/******/ 			if(runtime) var result = runtime(__webpack_require__);
/******/ 			if(parentChunkLoadingFunction) parentChunkLoadingFunction(data);
/******/ 			for(;i < chunkIds.length; i++) {
/******/ 				chunkId = chunkIds[i];
/******/ 				if(__webpack_require__.o(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 					installedChunks[chunkId][0]();
/******/ 				}
/******/ 				installedChunks[chunkIds[i]] = 0;
/******/ 			}
/******/ 		
/******/ 		}
/******/ 		
/******/ 		var chunkLoadingGlobal = self["webpackChunkgaf_tests"] = self["webpackChunkgaf_tests"] || [];
/******/ 		chunkLoadingGlobal.forEach(webpackJsonpCallback.bind(null, 0));
/******/ 		chunkLoadingGlobal.push = webpackJsonpCallback.bind(null, chunkLoadingGlobal.push.bind(chunkLoadingGlobal));
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/wasm chunk loading */
/******/ 	!function() {
/******/ 		// object to store loaded and loading wasm modules
/******/ 		var installedWasmModules = {};
/******/ 		
/******/ 		function promiseResolve() { return Promise.resolve(); }
/******/ 		
/******/ 		var wasmImportedFuncCache0;
/******/ 		var wasmImportedFuncCache1;
/******/ 		var wasmImportedFuncCache2;
/******/ 		var wasmImportedFuncCache3;
/******/ 		var wasmImportedFuncCache4;
/******/ 		var wasmImportedFuncCache5;
/******/ 		var wasmImportedFuncCache6;
/******/ 		var wasmImportedFuncCache7;
/******/ 		var wasmImportedFuncCache8;
/******/ 		var wasmImportedFuncCache9;
/******/ 		var wasmImportedFuncCache10;
/******/ 		var wasmImportedFuncCache11;
/******/ 		var wasmImportedFuncCache12;
/******/ 		var wasmImportedFuncCache13;
/******/ 		var wasmImportedFuncCache14;
/******/ 		var wasmImportedFuncCache15;
/******/ 		var wasmImportedFuncCache16;
/******/ 		var wasmImportedFuncCache17;
/******/ 		var wasmImportedFuncCache18;
/******/ 		var wasmImportedFuncCache19;
/******/ 		var wasmImportedFuncCache20;
/******/ 		var wasmImportedFuncCache21;
/******/ 		var wasmImportedFuncCache22;
/******/ 		var wasmImportedFuncCache23;
/******/ 		var wasmImportedFuncCache24;
/******/ 		var wasmImportedFuncCache25;
/******/ 		var wasmImportedFuncCache26;
/******/ 		var wasmImportedFuncCache27;
/******/ 		var wasmImportedFuncCache28;
/******/ 		var wasmImportedFuncCache29;
/******/ 		var wasmImportedFuncCache30;
/******/ 		var wasmImportedFuncCache31;
/******/ 		var wasmImportedFuncCache32;
/******/ 		var wasmImportedFuncCache33;
/******/ 		var wasmImportedFuncCache34;
/******/ 		var wasmImportedFuncCache35;
/******/ 		var wasmImportedFuncCache36;
/******/ 		var wasmImportedFuncCache37;
/******/ 		var wasmImportedFuncCache38;
/******/ 		var wasmImportedFuncCache39;
/******/ 		var wasmImportedFuncCache40;
/******/ 		var wasmImportedFuncCache41;
/******/ 		var wasmImportedFuncCache42;
/******/ 		var wasmImportedFuncCache43;
/******/ 		var wasmImportObjects = {
/******/ 			"./assets/wasm/pkg/wasm_bg.wasm": function() {
/******/ 				return {
/******/ 					"./wasm_bg.js": {
/******/ 						"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 							if(wasmImportedFuncCache0 === undefined) wasmImportedFuncCache0 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache0["__wbindgen_object_drop_ref"](p0i32);
/******/ 						},
/******/ 						"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache1 === undefined) wasmImportedFuncCache1 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache1["__wbindgen_string_new"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_log_682923c8ea4d4d53": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache2 === undefined) wasmImportedFuncCache2 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache2["__wbg_log_682923c8ea4d4d53"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 							if(wasmImportedFuncCache3 === undefined) wasmImportedFuncCache3 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache3["__wbindgen_object_clone_ref"](p0i32);
/******/ 						},
/******/ 						"__wbindgen_number_new": function(p0f64) {
/******/ 							if(wasmImportedFuncCache4 === undefined) wasmImportedFuncCache4 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache4["__wbindgen_number_new"](p0f64);
/******/ 						},
/******/ 						"__wbg_pageX_902206703620e827": function(p0i32) {
/******/ 							if(wasmImportedFuncCache5 === undefined) wasmImportedFuncCache5 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache5["__wbg_pageX_902206703620e827"](p0i32);
/******/ 						},
/******/ 						"__wbg_pageY_e4ea57875008005e": function(p0i32) {
/******/ 							if(wasmImportedFuncCache6 === undefined) wasmImportedFuncCache6 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache6["__wbg_pageY_e4ea57875008005e"](p0i32);
/******/ 						},
/******/ 						"__wbg_target_2dfa485f32a6d005": function(p0i32) {
/******/ 							if(wasmImportedFuncCache7 === undefined) wasmImportedFuncCache7 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache7["__wbg_target_2dfa485f32a6d005"](p0i32);
/******/ 						},
/******/ 						"__wbg_preventDefault_7c4d18eb2bb1a26a": function(p0i32) {
/******/ 							if(wasmImportedFuncCache8 === undefined) wasmImportedFuncCache8 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache8["__wbg_preventDefault_7c4d18eb2bb1a26a"](p0i32);
/******/ 						},
/******/ 						"__wbg_length_5dff05fb57a644be": function(p0i32) {
/******/ 							if(wasmImportedFuncCache9 === undefined) wasmImportedFuncCache9 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache9["__wbg_length_5dff05fb57a644be"](p0i32);
/******/ 						},
/******/ 						"__wbg_item_5385cba946ede1be": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache10 === undefined) wasmImportedFuncCache10 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache10["__wbg_item_5385cba946ede1be"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_get_4356d77a5acc3122": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache11 === undefined) wasmImportedFuncCache11 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache11["__wbg_get_4356d77a5acc3122"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_width_a04d387a0e0ffe94": function(p0i32) {
/******/ 							if(wasmImportedFuncCache12 === undefined) wasmImportedFuncCache12 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache12["__wbg_width_a04d387a0e0ffe94"](p0i32);
/******/ 						},
/******/ 						"__wbg_height_f7c5c956730ab37a": function(p0i32) {
/******/ 							if(wasmImportedFuncCache13 === undefined) wasmImportedFuncCache13 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache13["__wbg_height_f7c5c956730ab37a"](p0i32);
/******/ 						},
/******/ 						"__wbg_classList_bbb57a7d3cc23c85": function(p0i32) {
/******/ 							if(wasmImportedFuncCache14 === undefined) wasmImportedFuncCache14 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache14["__wbg_classList_bbb57a7d3cc23c85"](p0i32);
/******/ 						},
/******/ 						"__wbg_getBoundingClientRect_dbd899b7c945c55d": function(p0i32) {
/******/ 							if(wasmImportedFuncCache15 === undefined) wasmImportedFuncCache15 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache15["__wbg_getBoundingClientRect_dbd899b7c945c55d"](p0i32);
/******/ 						},
/******/ 						"__wbg_insertAdjacentHTML_a064ed8defe40b97": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 							if(wasmImportedFuncCache16 === undefined) wasmImportedFuncCache16 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache16["__wbg_insertAdjacentHTML_a064ed8defe40b97"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 						},
/******/ 						"__wbg_querySelector_f6958d956fc99dd5": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache17 === undefined) wasmImportedFuncCache17 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache17["__wbg_querySelector_f6958d956fc99dd5"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_querySelectorAll_4d091466df6d0bb7": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache18 === undefined) wasmImportedFuncCache18 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache18["__wbg_querySelectorAll_4d091466df6d0bb7"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_remove_5cd9814fbc0988da": function(p0i32) {
/******/ 							if(wasmImportedFuncCache19 === undefined) wasmImportedFuncCache19 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache19["__wbg_remove_5cd9814fbc0988da"](p0i32);
/******/ 						},
/******/ 						"__wbg_instanceof_HtmlElement_835072e813858ac0": function(p0i32) {
/******/ 							if(wasmImportedFuncCache20 === undefined) wasmImportedFuncCache20 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache20["__wbg_instanceof_HtmlElement_835072e813858ac0"](p0i32);
/******/ 						},
/******/ 						"__wbg_style_25309daade79abb3": function(p0i32) {
/******/ 							if(wasmImportedFuncCache21 === undefined) wasmImportedFuncCache21 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache21["__wbg_style_25309daade79abb3"](p0i32);
/******/ 						},
/******/ 						"__wbg_setonclick_020a4ab155fe4406": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache22 === undefined) wasmImportedFuncCache22 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache22["__wbg_setonclick_020a4ab155fe4406"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setontouchstart_bb83c005752b2a5f": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache23 === undefined) wasmImportedFuncCache23 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache23["__wbg_setontouchstart_bb83c005752b2a5f"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setontouchend_a5d45b44d40c1f6a": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache24 === undefined) wasmImportedFuncCache24 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache24["__wbg_setontouchend_a5d45b44d40c1f6a"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setontouchmove_c16f6766566c609e": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache25 === undefined) wasmImportedFuncCache25 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache25["__wbg_setontouchmove_c16f6766566c609e"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_add_3b4cecc512643e9f": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache26 === undefined) wasmImportedFuncCache26 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache26["__wbg_add_3b4cecc512643e9f"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_contains_c5b36ec90d80e696": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache27 === undefined) wasmImportedFuncCache27 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache27["__wbg_contains_c5b36ec90d80e696"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_remove_c15603553c81dc31": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache28 === undefined) wasmImportedFuncCache28 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache28["__wbg_remove_c15603553c81dc31"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_remove_36d83eabfd6b9d6e": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 							if(wasmImportedFuncCache29 === undefined) wasmImportedFuncCache29 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache29["__wbg_remove_36d83eabfd6b9d6e"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 						},
/******/ 						"__wbg_touches_976280ecfdead054": function(p0i32) {
/******/ 							if(wasmImportedFuncCache30 === undefined) wasmImportedFuncCache30 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache30["__wbg_touches_976280ecfdead054"](p0i32);
/******/ 						},
/******/ 						"__wbg_getPropertyValue_937a708feb88202f": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 							if(wasmImportedFuncCache31 === undefined) wasmImportedFuncCache31 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache31["__wbg_getPropertyValue_937a708feb88202f"](p0i32,p1i32,p2i32,p3i32);
/******/ 						},
/******/ 						"__wbg_removeProperty_e194113f699d27c9": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 							if(wasmImportedFuncCache32 === undefined) wasmImportedFuncCache32 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache32["__wbg_removeProperty_e194113f699d27c9"](p0i32,p1i32,p2i32,p3i32);
/******/ 						},
/******/ 						"__wbg_setProperty_dccccce3a52c26db": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 							if(wasmImportedFuncCache33 === undefined) wasmImportedFuncCache33 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache33["__wbg_setProperty_dccccce3a52c26db"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 						},
/******/ 						"__wbg_setProperty_9d2f969b8cc0de9c": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 							if(wasmImportedFuncCache34 === undefined) wasmImportedFuncCache34 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache34["__wbg_setProperty_9d2f969b8cc0de9c"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 						},
/******/ 						"__wbg_get_73dc28e00ffaad3c": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache35 === undefined) wasmImportedFuncCache35 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache35["__wbg_get_73dc28e00ffaad3c"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_call_3fc07b7d5fc9022d": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache36 === undefined) wasmImportedFuncCache36 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache36["__wbg_call_3fc07b7d5fc9022d"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_get_ade2390fbd606c54": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache37 === undefined) wasmImportedFuncCache37 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache37["__wbg_get_ade2390fbd606c54"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_new_76b8cb9dbeed052b": function() {
/******/ 							if(wasmImportedFuncCache38 === undefined) wasmImportedFuncCache38 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache38["__wbg_new_76b8cb9dbeed052b"]();
/******/ 						},
/******/ 						"__wbg_set_a68472f6e1a33f43": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache39 === undefined) wasmImportedFuncCache39 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache39["__wbg_set_a68472f6e1a33f43"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache40 === undefined) wasmImportedFuncCache40 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache40["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache41 === undefined) wasmImportedFuncCache41 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache41["__wbindgen_throw"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_closure_wrapper98": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache42 === undefined) wasmImportedFuncCache42 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache42["__wbindgen_closure_wrapper98"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbindgen_closure_wrapper100": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache43 === undefined) wasmImportedFuncCache43 = __webpack_require__.c["./assets/wasm/pkg/wasm_bg.js"].exports;
/******/ 							return wasmImportedFuncCache43["__wbindgen_closure_wrapper100"](p0i32,p1i32,p2i32);
/******/ 						}
/******/ 					}
/******/ 				};
/******/ 			},
/******/ 		};
/******/ 		
/******/ 		var wasmModuleMap = {
/******/ 			"assets_wasm_pkg_wasm_bg_js": [
/******/ 				"./assets/wasm/pkg/wasm_bg.wasm"
/******/ 			]
/******/ 		};
/******/ 		
/******/ 		// object with all WebAssembly.instance exports
/******/ 		__webpack_require__.w = {};
/******/ 		
/******/ 		// Fetch + compile chunk loading for webassembly
/******/ 		__webpack_require__.f.wasm = function(chunkId, promises) {
/******/ 		
/******/ 			var wasmModules = wasmModuleMap[chunkId] || [];
/******/ 		
/******/ 			wasmModules.forEach(function(wasmModuleId, idx) {
/******/ 				var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/ 		
/******/ 				// a Promise means "currently loading" or "already loaded".
/******/ 				if(installedWasmModuleData)
/******/ 					promises.push(installedWasmModuleData);
/******/ 				else {
/******/ 					var importObject = wasmImportObjects[wasmModuleId]();
/******/ 					var req = fetch(__webpack_require__.p + "" + {"assets_wasm_pkg_wasm_bg_js":{"./assets/wasm/pkg/wasm_bg.wasm":"15a9449dbe4869c34761"}}[chunkId][wasmModuleId] + ".module.wasm");
/******/ 					var promise;
/******/ 					if(importObject && typeof importObject.then === 'function' && typeof WebAssembly.compileStreaming === 'function') {
/******/ 						promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 							return WebAssembly.instantiate(items[0], items[1]);
/******/ 						});
/******/ 					} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 						promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 					} else {
/******/ 						var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 						promise = bytesPromise.then(function(bytes) {
/******/ 							return WebAssembly.instantiate(bytes, importObject);
/******/ 						});
/******/ 					}
/******/ 					promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 						return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 					}));
/******/ 				}
/******/ 			});
/******/ 		};
/******/ 	}();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// module cache are used so entry inlining is disabled
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	var __webpack_exports__ = __webpack_require__("./assets/js/contact.js");
/******/ 	
/******/ })()
;