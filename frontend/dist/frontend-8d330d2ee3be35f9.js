
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_26(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he37c60f3425f6b38(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_29(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h678db4db5c381fc5(arg0, arg1, addHeapObject(arg2));
}

let cachedUint32Memory0 = new Uint32Array();

function getUint32Memory0() {
    if (cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
function __wbg_adapter_210(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__hb8168e4f468a71cb(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

/**
*/
export class IntoUnderlyingByteSource {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingbytesource_free(ptr);
    }
    /**
    * @returns {string}
    */
    get type() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.intounderlyingbytesource_type(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {number}
    */
    get autoAllocateChunkSize() {
        const ret = wasm.intounderlyingbytesource_autoAllocateChunkSize(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {any} controller
    */
    start(controller) {
        wasm.intounderlyingbytesource_start(this.ptr, addHeapObject(controller));
    }
    /**
    * @param {any} controller
    * @returns {Promise<any>}
    */
    pull(controller) {
        const ret = wasm.intounderlyingbytesource_pull(this.ptr, addHeapObject(controller));
        return takeObject(ret);
    }
    /**
    */
    cancel() {
        const ptr = this.__destroy_into_raw();
        wasm.intounderlyingbytesource_cancel(ptr);
    }
}
/**
*/
export class IntoUnderlyingSink {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingsink_free(ptr);
    }
    /**
    * @param {any} chunk
    * @returns {Promise<any>}
    */
    write(chunk) {
        const ret = wasm.intounderlyingsink_write(this.ptr, addHeapObject(chunk));
        return takeObject(ret);
    }
    /**
    * @returns {Promise<any>}
    */
    close() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.intounderlyingsink_close(ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} reason
    * @returns {Promise<any>}
    */
    abort(reason) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.intounderlyingsink_abort(ptr, addHeapObject(reason));
        return takeObject(ret);
    }
}
/**
*/
export class IntoUnderlyingSource {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingsource_free(ptr);
    }
    /**
    * @param {any} controller
    * @returns {Promise<any>}
    */
    pull(controller) {
        const ret = wasm.intounderlyingsource_pull(this.ptr, addHeapObject(controller));
        return takeObject(ret);
    }
    /**
    */
    cancel() {
        const ptr = this.__destroy_into_raw();
        wasm.intounderlyingsource_cancel(ptr);
    }
}
/**
* Raw options for [`pipeTo()`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo).
*/
export class PipeOptions {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pipeoptions_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    get preventClose() {
        const ret = wasm.pipeoptions_preventClose(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    get preventCancel() {
        const ret = wasm.pipeoptions_preventCancel(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    get preventAbort() {
        const ret = wasm.pipeoptions_preventAbort(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {AbortSignal | undefined}
    */
    get signal() {
        const ret = wasm.pipeoptions_signal(this.ptr);
        return takeObject(ret);
    }
}
/**
*/
export class QueuingStrategy {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_queuingstrategy_free(ptr);
    }
    /**
    * @returns {number}
    */
    get highWaterMark() {
        const ret = wasm.queuingstrategy_highWaterMark(this.ptr);
        return ret;
    }
}
/**
* Raw options for [`getReader()`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader).
*/
export class ReadableStreamGetReaderOptions {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_readablestreamgetreaderoptions_free(ptr);
    }
    /**
    * @returns {any}
    */
    get mode() {
        const ret = wasm.readablestreamgetreaderoptions_mode(this.ptr);
        return takeObject(ret);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_listenerid_12315eee21527820 = function(arg0, arg1) {
        const ret = getObject(arg1).__yew_listener_id;
        getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbg_setlistenerid_3183aae8fa5840fb = function(arg0, arg1) {
        getObject(arg0).__yew_listener_id = arg1 >>> 0;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbg_subtreeid_e348577f7ef777e3 = function(arg0, arg1) {
        const ret = getObject(arg1).__yew_subtree_id;
        getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbg_setsubtreeid_d32e6327eef1f7fc = function(arg0, arg1) {
        getObject(arg0).__yew_subtree_id = arg1 >>> 0;
    };
    imports.wbg.__wbg_cachekey_b61393159c57fd7b = function(arg0, arg1) {
        const ret = getObject(arg1).__yew_subtree_cache_key;
        getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbg_setcachekey_80183b7cfc421143 = function(arg0, arg1) {
        getObject(arg0).__yew_subtree_cache_key = arg1 >>> 0;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_error_71d6845bf00a930f = function(arg0, arg1) {
        var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
        wasm.__wbindgen_free(arg0, arg1 * 4);
        console.error(...v0);
    };
    imports.wbg.__wbg_fetch_2a45e344e57266ef = function(arg0) {
        const ret = fetch(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getReader_8ecba87d8003e950 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).getReader();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_close_e9110ca16e2567db = function(arg0) {
        getObject(arg0).close();
    };
    imports.wbg.__wbg_enqueue_d71a1a518e21f5c3 = function(arg0, arg1) {
        getObject(arg0).enqueue(getObject(arg1));
    };
    imports.wbg.__wbg_byobRequest_08c18cee35def1f4 = function(arg0) {
        const ret = getObject(arg0).byobRequest;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_close_da7e6fb9d9851e5a = function(arg0) {
        getObject(arg0).close();
    };
    imports.wbg.__wbg_view_231340b0dd8a2484 = function(arg0) {
        const ret = getObject(arg0).view;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_respond_8fadc5f5c9d95422 = function(arg0, arg1) {
        getObject(arg0).respond(arg1 >>> 0);
    };
    imports.wbg.__wbg_buffer_4e79326814bdd393 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_byteOffset_b69b0a07afccce19 = function(arg0) {
        const ret = getObject(arg0).byteOffset;
        return ret;
    };
    imports.wbg.__wbg_byteLength_5299848ed3264181 = function(arg0) {
        const ret = getObject(arg0).byteLength;
        return ret;
    };
    imports.wbg.__wbg_cancel_7f202496da02cd45 = function(arg0) {
        const ret = getObject(arg0).cancel();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_releaseLock_9ae075576f54bf0b = function() { return handleError(function (arg0) {
        getObject(arg0).releaseLock();
    }, arguments) };
    imports.wbg.__wbg_read_88c96573fc8b3b01 = function(arg0) {
        const ret = getObject(arg0).read();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_done_76252d32deca186b = function(arg0) {
        const ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_ff3741eb46856618 = function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Window_acc97ff9f5d2c7b4 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_document_3ead31dbcad65886 = function(arg0) {
        const ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_body_3cb4b4042b9a632b = function(arg0) {
        const ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_976dbb84fe1661b5 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createElementNS_1561aca8ee3693c0 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTextNode_300f845fab76642f = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_parentNode_e397bbbe28be7b28 = function(arg0) {
        const ret = getObject(arg0).parentNode;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_parentElement_0cffb3ceb0f107bd = function(arg0) {
        const ret = getObject(arg0).parentElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_lastChild_a2f5ed739809bb31 = function(arg0) {
        const ret = getObject(arg0).lastChild;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_nextSibling_62338ec2a05607b4 = function(arg0) {
        const ret = getObject(arg0).nextSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_setnodeValue_4077cafeefd0725e = function(arg0, arg1, arg2) {
        getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_textContent_77bd294928962f93 = function(arg0, arg1) {
        const ret = getObject(arg1).textContent;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_appendChild_e513ef0e5098dfdd = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_insertBefore_9f2d2defb9471006 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_removeChild_6751e9ca5d9aaf00 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).removeChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_status_c4ef3dd591e63435 = function(arg0) {
        const ret = getObject(arg0).status;
        return ret;
    };
    imports.wbg.__wbg_headers_fd64ad685cf22e5d = function(arg0) {
        const ret = getObject(arg0).headers;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_body_7bf1a45a7ee13f62 = function(arg0) {
        const ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_new_2d0053ee81e4dd2a = function() { return handleError(function () {
        const ret = new Headers();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_append_de37df908812970d = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).append(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_newwithstrandinit_05d7180788420c40 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_ShadowRoot_76b32ccdae10a710 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ShadowRoot;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_host_57eec05a2624bc1b = function(arg0) {
        const ret = getObject(arg0).host;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_value_ccb32485ee1b3928 = function(arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setvalue_df64bc6794c098f2 = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_instanceof_Element_33bd126d58f2021b = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Element;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_namespaceURI_e19c7be2c60e5b5c = function(arg0, arg1) {
        const ret = getObject(arg1).namespaceURI;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setinnerHTML_32081d8a164e6dc4 = function(arg0, arg1, arg2) {
        getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_outerHTML_bf662bdff92e5910 = function(arg0, arg1) {
        const ret = getObject(arg1).outerHTML;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_children_67776b4810f38b6a = function(arg0) {
        const ret = getObject(arg0).children;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_removeAttribute_beaed7727852af78 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_setAttribute_d8436c14a59ab1af = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_setchecked_f1e1f3e62cdca8e7 = function(arg0, arg1) {
        getObject(arg0).checked = arg1 !== 0;
    };
    imports.wbg.__wbg_value_b2a620d34c663701 = function(arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setvalue_e5b519cca37d82a7 = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_bubbles_03eed164b4feeaf1 = function(arg0) {
        const ret = getObject(arg0).bubbles;
        return ret;
    };
    imports.wbg.__wbg_cancelBubble_8c0bdf21c08f1717 = function(arg0) {
        const ret = getObject(arg0).cancelBubble;
        return ret;
    };
    imports.wbg.__wbg_composedPath_160ed014dc4d787f = function(arg0) {
        const ret = getObject(arg0).composedPath();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_addEventListener_1fc744729ac6dc27 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_b10f1a66647f3aa0 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
    }, arguments) };
    imports.wbg.__wbg_fetch_749a56934f95c96c = function(arg0, arg1) {
        const ret = getObject(arg0).fetch(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_57245cc7d7c7619d = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_from_7ce3cb27cb258569 = function(arg0) {
        const ret = Array.from(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_6e3bbe7c8bd4dbd8 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_8d2af00bc1e329ee = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_b5b063fc6c2f0376 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_97ae9d8645dc388b = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_168da88779e35f61 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_next_aaef7c8aa5e212ac = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_next_579e583d33566a86 = function(arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_done_1b73b0672e15f234 = function(arg0) {
        const ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_1ccc36bc03462d71 = function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_is_40a66842732708e7 = function(arg0, arg1) {
        const ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_0b9bfdd97583284e = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_toString_7be108a12ef03bc2 = function(arg0) {
        const ret = getObject(arg0).toString();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_6f9d4f28845f426c = function() {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_9962f939219f1820 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_210(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_99fe17964f31ffc0 = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_catch_4eaf75e3e2d27d00 = function(arg0, arg1) {
        const ret = getObject(arg0).catch(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_11f7a54d67b4bfad = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_cedad20fbbd9418a = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_globalThis_7f206bda628d5286 = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_6d479506f72c6a71 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_f2557cc78490aceb = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_ba75c50d1cf384f4 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_8c3f0052272a457a = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_d9aa266703cb98be = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_9e1ae1900cb0fbd5 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_set_83db9690f9353e79 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_buffer_3f3d764d4747d564 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_765201544a2b6869 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_has_8359f114ce042f5a = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.has(getObject(arg0), getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_bf3f89b92d5a34bf = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper2707 = function(arg0, arg1, arg2) {
        const ret = makeClosure(arg0, arg1, 164, __wbg_adapter_26);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper7721 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 338, __wbg_adapter_29);
        return addHeapObject(ret);
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = new Int32Array();
    cachedUint32Memory0 = new Uint32Array();
    cachedUint8Memory0 = new Uint8Array();

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('frontend-8d330d2ee3be35f9_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
