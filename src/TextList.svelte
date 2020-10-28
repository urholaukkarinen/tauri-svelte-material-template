<script>
    import List, {Item, Text} from '@smui/list';
    import { listen } from 'tauri/api/event'
    import { onMount } from 'svelte';

    let texts = ["test"];

    onMount(async () => {
        listen("texts", (received_texts) => {
            texts = received_texts.payload
        })
    })
</script>

<style>
    :global(.text-list) {
        max-width: 340px;
        border: 1px solid rgba(0,0,0,.1);
    }
</style>

<List class="text-list">
{#each texts as text}
    <Item>
        <Text>{text}</Text>
    </Item>
{/each}
</List>