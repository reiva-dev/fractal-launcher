import type { Component } from 'solid-js';
import HeaderTools from './app/HeaderTools';
import PlaceHolder from './app/PlaceHolder';
import { app } from './App.css'

import TitleBar from './windowed/TitleBar';
import FooterNavi from './app/FooterNavi';

const App: Component = () => {
  return (
    <div class={app.main}>
      <TitleBar />
      <HeaderTools />
      <div class={app.wrap}>
        { /* Todo: Impl Need Mod Launcher Components */ }
        <PlaceHolder />
      </div>
      <FooterNavi />
    </div>
  );
};

export default App;