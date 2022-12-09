import { createSignal } from "solid-js";
import { footerNavi } from "../FooterNavi.css";

import KeyIconSvg from "../../assets/key.svg?component-solid";
import { invoke } from "@tauri-apps/api";

function Signin() {
  const [loginStete, signupFunc] = createSignal(LoginState.NoLogin);
  const loginHandler = () => { 
    console.log("login_handler");
    invoke('login_msa');
  }

  return (
    <div class={footerNavi.footerButton}>
      <div class={footerNavi.account} onClick={loginHandler}>
        <KeyIconSvg width="16px" height="16px" viewBox="0 0 24 24" style={{"padding-right": '6px'}} />
        <p>Login with MS Account</p>
      </div>
    </div>
  );
}

enum LoginState {
  NoLogin,
  Validate,
  Login,
}

export default Signin;