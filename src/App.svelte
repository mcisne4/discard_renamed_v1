<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let err_info: any = "";

  interface ErrFromRust {
    error: string;
    info: string;
  }

  async function test_error() {
    invoke("err_test")
      .then((res) => console.log(`Succeded?: '${res}'`))
      .catch((err: ErrFromRust) => {
        err_info = `${err.error} | ${err.info}`;
        console.log("Errored:\n", err);
      });
  }
</script>

<main>
  <h1>Hello WOrld</h1>
  <p>ERROR: {err_info}</p>
  <button on:click={test_error}>Trigger</button>
</main>

<style lang="scss">
  main {
    background-color: orangered;
    width: 100%;
    height: 100%;
    min-height: 100vh;
    min-width: 100vw;
  }
</style>
