<template>
  <div class="not-found">
    <div class="not-found-content">
      <h1>404</h1>
      <h2>Page Not Found</h2>
      <p>The page you're looking for doesn't exist.</p>
      <router-link to="/home" class="btn-home"> Go to Home </router-link>
    </div>
  </div>
</template>

<style scoped>
.not-found {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
}

.not-found-content {
  max-width: 500px;
  padding: 2rem;
}

.not-found h1 {
  font-size: 6rem;
  margin: 0;
  color: #dc3545;
}

.not-found h2 {
  font-size: 2rem;
  margin: 1rem 0;
  color: #333;
}

.not-found p {
  color: #666;
  margin-bottom: 2rem;
}

.btn-home {
  display: inline-block;
  padding: 0.75rem 2rem;
  background: #007bff;
  color: white;
  text-decoration: none;
  border-radius: 4px;
  transition: background 0.2s;
}

.btn-home:hover {
  background: #0056b3;
}
</style>
