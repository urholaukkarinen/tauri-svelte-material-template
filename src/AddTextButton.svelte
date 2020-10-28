<script>
	import Button from '@smui/button';
	import { invoke } from "tauri/api/tauri"

	function addText() {
		invoke({
			cmd: "addText",
			text: "Test"
		})
	}
</script>

<Button on:click={addText}>Add text to list</Button>