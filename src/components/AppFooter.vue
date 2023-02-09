<template>


	<footer
		class="flex fixed bottom-0 left-0 z-20 w-full p-4 bg-white border-t border-gray-200 shadow md:flex md:items-center md:justify-between md:p-6 dark:bg-gray-800 dark:border-gray-600">
		<span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">© 2022 <a target="_blank"
				href="https://www.linkedin.com/in/husseindossoki/" class="hover:underline">Hussein Dossoki</a>. All
			Rights
			Reserved.
		</span>

		<div class="ml-auto">
			<input type="checkbox" class="checkbox" id="theme-checkbox" @change="toggleDark(); checkBackground()">
			<label for="theme-checkbox" class="checkbox-label">
				<font-awesome-icon icon="fa-solid fa-sun" />
				<font-awesome-icon icon="fa-solid fa-moon" />
				<span class="ball"></span>
			</label>
		</div>

	</footer>

</template>

<script setup>
import { useDark, useToggle } from "@vueuse/core";
const isDark = useDark();
const toggleDark = useToggle(isDark);
function checkBackground() {
	console.log(document.documentElement.classList.contains('dark'));
	if (!document.documentElement.classList.contains('dark')) {
		document.body.style.backgroundColor = '#1F2937'; // OR #1F2937, #0d1117
	} else {
		document.body.style.backgroundColor = '#fff';
	}
}

</script>

<style scoped>
.checkbox {
	opacity: 0;
	position: absolute;
}

.checkbox-label {
	background-color: #111;
	width: 50px;
	height: 26px;
	border-radius: 50px;
	position: relative;
	padding: 5px;
	cursor: pointer;
	display: flex;
	justify-content: space-between;
	align-items: center;
}

.fa-moon {
	color: #f1c40f;
}

.fa-sun {
	color: #f39c12;
}

.checkbox-label .ball {
	background-color: #fff;
	width: 22px;
	height: 22px;
	position: absolute;
	left: 2px;
	top: 2px;
	border-radius: 50%;
	transition: transform 0.2s linear;
}

.checkbox:checked+.checkbox-label .ball {
	transform: translateX(24px);
}
</style>
