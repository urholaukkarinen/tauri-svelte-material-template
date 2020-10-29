<script>
    import Button from '@smui/button';
    import Snackbar, {Label} from '@smui/snackbar';
    import Textfield from '@smui/textfield';

    import { onMount } from 'svelte';

    import { invoke } from "tauri/api/tauri";
    import { listen } from "tauri/api/event";

    let name = '';
    let response = '';
    let snackbar;

    onMount(async => {
        listen("hello_from_rust", (msg) => {
            response = msg.payload;
            snackbar.open();
        })
    })

    function sayHelloToRust() {
        invoke({
            cmd: "hello",
            name: name
        })
    }
</script>

<div>
    <Textfield
        bind:value={name}
        label="Name"/>
</div>

<div>
    <Button variant="raised" disabled={!name} on:click={sayHelloToRust}>
        Say hello to Rust
    </Button>
</div>

<Snackbar bind:this={snackbar}>
    <Label>{response}</Label>
</Snackbar>