<script>
  import Greet from './lib/Greet.svelte'
  import { getInsets } from 'tauri-plugin-safe-area-insets'

	let response = ''

	// function updateResponse(returnValue) {
	// 	response += `[${new Date().toLocaleTimeString()}] ` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	// }

	function _ping() {
    getInsets().then(res => {
      response += JSON.stringify(res);
    }).catch(err => {
      response += JSON.stringify(err);
    })
	}

	const MINIMUM_KEYBOARD_HEIGHT = 300;
	const set = async () => {
  	let deviceInsets = await getInsets();
  	document.documentElement.style.cssText = `--safe-area-inset-top: ${deviceInsets.top}px; --safe-area-inset-bottom: ${deviceInsets.bottom}px;`;
	  if (deviceInsets.bottom > MINIMUM_KEYBOARD_HEIGHT) {
	    // document.body.style.height = `${}`;
	    document.clientHeight;
	  }
  }

  set()
  document.addEventListener('focus', set);
  document.addEventListener('blur', set);

</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
    <input>
  </div>

  <div>
    <button on:click="{_ping}">Ping</button>
    <div>{@html response}</div>
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
