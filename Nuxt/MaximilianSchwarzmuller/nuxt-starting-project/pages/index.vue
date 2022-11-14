<template>
  <section class="container">
    <Header />
    <nuxt-link to="/users">Users</nuxt-link>
    <nuxt-link to="/products">Products</nuxt-link>
  </section>
</template>

<script>
import Header from "../components/Header";

export default {
  components: {
    Header,
  }
}
</script>
