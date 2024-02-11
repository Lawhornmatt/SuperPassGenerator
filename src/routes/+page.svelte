<script lang=ts>
	import { RangeSlider } from '@skeletonlabs/skeleton';
	import { invoke } from "@tauri-apps/api/tauri";

	let password: string = "";
	
	let value = 15;
	let max = 25;

	let lwrcase = false;
	let uprcase = false;
	let nmbrcase = false;
	let spclcase = false;

	async function genPass() {
		password = await invoke ("gen_pass", { 
			value: value,
			lwrcase: lwrcase, 
			uprcase: uprcase, 
			nmbrcase: nmbrcase, 
			spclcase: spclcase 
		});
	};
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<h1 class="h1 my-10 font-mono">💽 Password Generator! 🔒</h1>

		<div class="card p-4">
			<header class="text-2xl card-header font-semibold my-2">
				Generate a Password
			</header>

			<hr>
			
			<section class="p-4 flex justify-center">
				<textarea
					readonly
					id="password"
					name="password"
					placeholder="Your New Password"
					aria-label="Generated Password"
					class="w-full h-32 text-center border-dashed bordr-slate border-2 rounded-md text-black font-semibold font-mono"
					value={password}
			  	></textarea>
			</section>

			<hr>

			<div class="w-full flex justify-center">
				<button type="button" class="btn variant-filled-primary rounded-xl min-w-[25%] my-5" on:click={genPass} disabled={!lwrcase && !uprcase && !nmbrcase && !spclcase}>
					<span class="font-semibold">Generate</span>
				</button>
			</div>
		</div>

		<div class="card p-4">
			<header class="text-2xl card-header font-semibold my-2">
				Options
			</header>

			<hr>
			
			<RangeSlider name="range-slider" bind:value={value} max={max} step={1} ticked>
				<div class="flex justify-between items-center mt-4 mb-2">
					<div class="font-semibold">Password Character Length:</div>
					<div class="text-xs">{value} / {max}</div>
				</div>
			</RangeSlider>

			<p class="mt-4 mb-2 font-semibold">Choose atleast one character type:</p>

			<label class="mb-3">
				<input type="checkbox" bind:checked={lwrcase} />
				Use Lowercase Characters
			</label>

			<label class="my-3">
				<input type="checkbox" bind:checked={uprcase} />
				Use Uppercase Characters
			</label>

			<label class="my-3">
				<input type="checkbox" bind:checked={nmbrcase} />
				Use Numbers
			</label>

			<label class="my-3">
				<input type="checkbox" bind:checked={spclcase} />
				Use Special Characters
			</label>
		</div>


	</div>
</div>
