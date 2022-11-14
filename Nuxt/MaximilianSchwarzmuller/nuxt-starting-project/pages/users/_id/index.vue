<!--Start the file name with an underscore to make it dynamic-->
<template>
  <h1>A single user, with ID: {{ $route.params.id }}</h1>
</template>

<script>
export default {
  validate({ params }) {
    return /^\d+$/.test(params.id)
  },
  layout: 'users'
}
</script>
