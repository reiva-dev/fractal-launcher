import { appWindow } from '@tauri-apps/api/window';
import { titleBar } from './TitleBar.css';
import ReivaIcon from "../assets/reivatek.svg?component-solid";

const AppIcon = () => <ReivaIcon viewBox='0 0 135.47 135.47' style={{ fill:'#6d6d6d', width: '24px', height: '24px', }} />

function TitleBar() {
    const appExit = async () => { await appWindow.hide(); };
    const appMinimize = async () => { await appWindow.minimize(); };
    const appToggleMaximize = async () => { await appWindow.toggleMaximize(); };
    
    return (
      <div class={titleBar.mainContainer} data-tauri-drag-region>
        <div class={titleBar.rights} data-tauri-drag-region>
          <div class={titleBar.icon}>{AppIcon()}</div>
          <div class={titleBar.title}>Fractal</div>
        </div>
        <div class={titleBar.rootBox}>
          <div class={titleBar.button.Normal} onClick={appMinimize}>
            <svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" width="1em" height="1em"
             preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
              <path fill="#6d6d6d" d="M19 12.998H5v-2h14z"/>
            </svg>
          </div>
          <div class={titleBar.button.Normal} onClick={appToggleMaximize}>
           <svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" width="16" height="16"
             preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
              <path fill="#6d6d6d" d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/>
            </svg>
          </div>
          <div class={titleBar.button.Alart} onClick={appExit}>
            <svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" width="1em" height="1em"
             preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
              <path fill="#6d6d6d" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"/>
            </svg>
          </div>
        </div>
      </div>
    )
}

export default TitleBar;