<script lang="ts">
    import {listen, TauriEvent} from '@tauri-apps/api/event';
    import {onDestroy, onMount} from 'svelte'

    /**
     * List of allowed file extensions. Disallowed files are filtered out.
     *
     * If it's null (default), all file extensions are allowed.
     */
    export let extensions: null | string[] = null
    /** Handle a file drop of one or more files */
    export let handleFiles: (files: string[]) => void = () => {
        // noop
    }
    /**
     * Handle a file drop of a single file.
     *
     * Note that `handleFile()` is also called, no matter what.
     *
     * This is not called if any disallowed files were filtered out.
     */
    export let handleOneFile: (file: string) => void = () => {
        // noop
    }

    function getValidPaths(paths: string[]) {
        if (extensions === null) {
            return paths
        }
        let validPaths = []
        for (const path of paths) {
            for (const ext of extensions) {
                if (path.endsWith('.' + ext)) {
                    validPaths.push(path)
                    break
                }
            }
        }
        return validPaths
    }

    let files: string[] = []

    let fileDrop;
    let fileDropCancelled;

    onMount(()=>{
        files = []

        fileDrop  = listen('tauri://drop', (e) => {
            //payload: {paths: ["/Users/zhouyi/Downloads/tauri-dev/core/tauri/src/manager.rs"]
            if(e.payload?.paths){
                files = getValidPaths(e.payload.paths)
            }
            if (files.length > 0) {
                handleFiles(files)
            }
            if (files.length === 1) {
                handleOneFile(files[0])
            }
            files = []
        })

        fileDropCancelled = listen('tauri://drop-cancelled', () => {
            files = []
        });
    });

    onDestroy(async () => {
        if(fileDrop){
            const unlisten = await fileDrop
            unlisten()
        }
    })

    onDestroy(async () => {
        if(fileDropCancelled){
            const unlisten = await fileDropCancelled
            unlisten()
        }
    })
</script>
