<script>
  export let size = 16;
  export let direction = "right";
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points={direction === "down" ? "6 9 12 15 18 9" : "9 18 15 12 9 6"}
  ></polyline>
</svg>
