<script lang="ts">

	import '../app.css';

	import {onMount, setContext} from 'svelte';
	import {writable} from "svelte/store";
	import {afterNavigate} from "$app/navigation";
	import {page} from "$app/stores";
	import Header from "$lib/app-page/Header.svelte";
	import LeftMenu from "$lib/app-page/LeftMenu.svelte";
	import {parseMenuPaths} from "$lib/app-page/page.menu";
	import {configStore} from "$lib/app-stores/base/configStore";
	export let data;

	/**
	 * 页面导航路径
	 */
	let navPaths = writable([]);

	/**
	 * 可后退
	 */
	let canBack = writable(false);
	/**
	 * 可前进
	 */
	let canForward = writable(false);

	setContext('AppContext',{
		navPaths,
		canBack,
		canForward,
	});

	afterNavigate(({from})=>{
		$canBack = !!from && !!from.url;
		if($page.data){
			if($page.data.pathMenuPaths){
				$navPaths = [].concat($page.data.pathMenuPaths);
			}else {
				let menuPaths = parseMenuPaths($page);
				if(menuPaths && menuPaths.length){
					$navPaths = menuPaths;
				}
			}
		}
	});

	onMount(()=>{
		configStore.fetch();
	});

</script>


{#if $page.params._windowType === 'window'}
	<div class="bg-white h-full">
		<slot/>
	</div>
{:else}
	<div class="bg-white h-dvh  border-t border-gray-50">
		<!--  左侧窄菜单	-->
		<div class="lg:w-20 pb-4 overflow-auto hidden lg:block z-50 fixed inset-y-0 bg-gray-900">
			<div class="justify-center items-center h-16 flex shrink-0">

			</div>
			<LeftMenu menus={data.menus} activeModule={data.activeModule}/>
		</div>
		<main class="lg:pl-20 h-dvh flex flex-col">
			<Header navPaths={$navPaths}/>
			<div class="flex flex-1 h-0">
				<slot/>
			</div>
		</main>
	</div>
{/if}

