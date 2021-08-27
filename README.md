# wasm-sha1

A WebAssembly sha1 digest lib.

## usage

The browser should support ES module.

```html
<!DOCTYPE html>
<html>

<head>
	<meta charset="UTF-8">
</head>

<body>
	<!-- enable ES module -->
	<script type="module"> 
		import init, { Sha1Digest } from '//cdn.jsdelivr.net/gh/yin1999/wasm-sha1@release/sha1_wasm.js'
		init()
			.then(() => {
				const hash = new Sha1Digest()
				const enc = new TextEncoder()
				const data = enc.encode("hello world")
				hash.update(data)

				console.log(`sha1 digest: ${hash.finalize()}`)
			})
	</script>
</body>

</html>
```
