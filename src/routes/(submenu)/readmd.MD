
src-tauri/capabilities/default.json

配置 permissions，开放本地文件夹的exists 、mkdir、write-file 权限
```
"fs:allow-write",
    {
      "identifier": "fs:allow-exists",
      "allow": [{ "path": "/Volumes/D/project/workspace-youi7/youi-tauri-app/src/routes/(submenu)/**" }, { "path": "$APPDATA/**" }]
    },
    {
      "identifier": "fs:allow-create",
      "allow": [{ "path": "/Volumes/D/project/workspace-youi7/youi-tauri-app/src/routes/(submenu)/**" }, { "path": "$APPDATA/**" }]
    },
    {
      "identifier": "fs:allow-write-file",
      "allow": [{ "path": "/Volumes/D/project/workspace-youi7/youi-tauri-app/src/routes/(submenu)/**" }, { "path": "$APPDATA/**" }]
    },
    {
      "identifier": "fs:allow-mkdir",
      "allow": [{ "path": "/Volumes/D/project/workspace-youi7/youi-tauri-app/src/routes/(submenu)/**" }, { "path": "$APPDATA/**" }]
    }

```

./layout.ts

```
import {exists, mkdir, writeFile} from "@tauri-apps/plugin-fs";
/**
 * 初始化空白使用
 */
function _check_menu_page(menus) {
    let source_path = '/Volumes/D/project/workspace-youi7/youi-tauri-app/src/routes/(submenu)';
    menus.forEach(menu=>{
        if(menu.href){
            const dir = `${source_path}${menu.href}`;
            exists(`${dir}/+page.svelte`).then(r=>{
                if(!r){
                    _write_empty_page(menu,dir).then(s=>console.log(s))
                }
            })
        }

        if(menu.children){
            _check_menu_page(menu.children);
        }
    })
}

/**
 * 生成空白页面
 * @param menu
 * @param path
 */
async function _write_empty_page(menu,path) {
    const encoder = new TextEncoder();
    const data = encoder.encode(menu.text);
    await mkdir(path,{recursive:true});
    const file = await writeFile(`${path}/+page.svelte`, data,{});
    return file;
}

```