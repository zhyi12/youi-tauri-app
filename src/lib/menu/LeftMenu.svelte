<script lang="ts">
    import type {MenuInfo} from "../app-entity/base/menu";

    import {Icon, homeIcon, cogIcon, cubeIcon, hashtagIcon, hddIcon, globeIcon} from "../youi";
    import {APP_ICONS} from "../app-icon/icons";

    export let menus:Array<MenuInfo> = [];

    export let activeModule:string = undefined;

    const icons = {
        menu:hashtagIcon,
        home:homeIcon,
        res:hddIcon,
        geo:globeIcon,
        settings:cogIcon,
        dataproc:cubeIcon
    };

</script>

<div class="app-left-menu">
    <div class="app-logo">

    </div>
    {#each menus as menu}
        <div class="menu-bar" class:active={(menu.name==='home' && !activeModule) || activeModule === (menu.module||menu.name)}>
            <a href={menu.href} title={menu.text} on:click={()=>{activeModule = menu.name}}>
                <Icon scale={menu.scale||1.8} data={icons[menu.icon||menu.name]||APP_ICONS[menu.icon]||icons.menu}>

                </Icon>
            </a>
        </div>
    {/each}
</div>

<style lang="scss">
  .app-left-menu {
    width: 54px;
    background:  rgba(0, 117, 246, 1);

    .menu-bar{
      height: 48px;
      padding-top:6px;
      text-align: center;

      a{
        cursor: pointer;
        display: block;
        color: white;
        line-height: 32px;

        &:hover{
          color: black;
        }
      }

      &.active,&:hover{
        background: steelblue;
        a{
          color: yellow;
        }
      }

    }
  }
</style>