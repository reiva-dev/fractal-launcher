import { appWindow } from '@tauri-apps/api/window';
import { styled } from 'solid-styled-components';
import ReivaIcon from "../assets/reivatek.svg?component";

function AppIcon() {
  return ( <ReivaIcon style={{ fill:'#6d6d6d', width:'28', height:'28', }} /> )
}

function TitleBar() {
    const appExit = async () => { await appWindow.hide(); };
    const appMinimize = async () => { await appWindow.minimize(); };
    const appToggleMaximize = async () => { await appWindow.toggleMaximize(); };
    
    return (
      <Bar data-tauri-drag-region>
        <AppRightsContents data-tauri-drag-region>
          <Icon>{AppIcon()}</Icon>
          <AppTitle>Fractal</AppTitle>
        </AppRightsContents>
        <WindowedContents>
          <BarButton onClick={appMinimize}>
            <svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" width="1em" height="1em"
             preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
              <path fill="white" d="M19 12.998H5v-2h14z"/>
            </svg>
          </BarButton>
          <BarButton onClick={appToggleMaximize}>
           <svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" width="16" height="16"
             preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
              <path fill="white" d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/>
            </svg>
          </BarButton>
          <BarButtonRed onClick={appExit}>
            <svg xmlns="http://www.w3.org/2000/svg" aria-hidden="true" width="1em" height="1em"
             preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
              <path fill="white" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"/>
            </svg>
          </BarButtonRed>
        </WindowedContents>
      </Bar>
    )
}

const Bar = styled.div`
  height: 30px;
  background: #232526;
  user-select: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
  top: 0;
  left: 0;
  right: 0;
  border-color: #424242;
  border-bottom-style: solid;
  border-width: 0.5px;
`;

const WindowedContents = styled.div`
  display: flex;
`;

const BarButton = styled.div`
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 45px;
  height: 30px;
  &:hover {
    background: rgba(255, 255, 255, 0.5);
  }
`;

const BarButtonRed = styled.div`
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 45px;
  height: 30px;
  &:hover {
    background: rgba(244, 67, 54, 0.5);
  }
`;

const AppRightsContents = styled.div`
  display: flex;
`;

const Icon = styled.div`
  display: inline-flex;
  justify-content: center;
  align-items: center;
  height: 30px;
  width: 45px;
`;

const AppTitle = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  color: #6d6d6d;
`;

export default TitleBar;