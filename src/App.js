import React from 'react';
import logo from './logo.svg';
import './App.css';

const p = window.preload;

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <ul>
          <li>p.is_dev = {p.is_dev.toString()}</li>
          <li>p.lib.example_add( Math.PI, 1.23 ) = {p.lib.example_add( Math.PI, 1.23 )}</li>
          <li>p.lib.example_add( -1.23, Math.E ) = {p.lib.example_add( -1.23, Math.E )}</li>
          <li>p.lib.example_concat_with_free( "🍣su", "shi🍣" ) = {p.lib.example_concat_with_free( "🍣su", "shi🍣" )}</li>
          <li>p.lib.example_concat_with_free( "🌶haba", "nero🌶" ) = {p.lib.example_concat_with_free( "🌶haba", "nero🌶" )}</li>
        </ul>
      </header>
    </div>
  );
}

export default App;
