import type { Component } from 'solid-js';
import { styled } from 'solid-styled-components';
import HeaderTools from './app/HeaderTools';
import PlaceHolder from './app/PlaceHolder';
import SideBar from './app/SideBar';

import TitleBar from './windowed/TitleBar';

const App: Component = () => {
  return (
    <>
      <TitleBar />
      <HeaderTools />
      <AppWrap>
        <SideBar />
        <PlaceHolder />
      </AppWrap>
    </>
  );
};

const AppWrap = styled.div`
  display: flex;
  flex-direction: row;
  flex-grow: 0;
`;

export default App;