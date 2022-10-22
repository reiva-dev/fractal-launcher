import { createSignal } from "solid-js";
import { styled } from "solid-styled-components";

function HeaderTools() {
    const [isSearchBarOpen, setSearchMenu] = createSignal<boolean>(false);

    const toggleHandler = () => setSearchMenu(prev => !prev);

    return (
      <Bar>
        <Button onClick={toggleHandler}
         xmlns="http://www.w3.org/2000/svg" aria-hidden="true" width="28" height="28"
         preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
            <path d="m19.6 21l-6.3-6.3q-.75.6-1.725.95Q10.6 16 9.5 16q-2.725 0-4.612-1.887Q3 12.225 3 9.5q0-2.725 1.888-4.613Q6.775 3 9.5 3t4.613 1.887Q16 6.775 16 9.5q0 1.1-.35 2.075q-.35.975-.95 1.725l6.3 6.3ZM9.5 14q1.875 0 3.188-1.312Q14 11.375 14 9.5q0-1.875-1.312-3.188Q11.375 5 9.5 5Q7.625 5 6.312 6.312Q5 7.625 5 9.5q0 1.875 1.312 3.188Q7.625 14 9.5 14Z"/>
        </Button>
        { isSearchBarOpen() ? <SearchBar placeholder="Search..." /> : <div></div> }
      </Bar>
    )
}

const Bar = styled.div`
  height: 30px;
  background: #232526;
  display: flex;
  position: relative;
  align-items: center;
  padding-top: 6px;
  padding-bottom: 6px;
  padding-left: 12px;
`;

const Button = styled.svg`
  fill: #7d7d7d;
  display: flex;
  align-items: center;
  max-width: 30px;
  max-height: 30px;

  &:hover {
    fill: white;
  }
`;

const SearchBar = styled.input`
  height: calc(1em + 3px);
  width: 30em;
  align-items: center;
  margin-top: 6px;
  margin-left: 12px;
  margin-bottom: 3px;
  padding-bottom: 3px;
  border: none;
  border-bottom: #424242;
  border-bottom-style: solid;
  border-width: 0.5px;
  background-color: transparent;
  color: #6d6d6d;

  &:focus {
    outline: none;
    color: #a8a8a8;
  }
`;



export default HeaderTools;