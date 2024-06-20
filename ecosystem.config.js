module.exports = {
  apps: [
    {
      name: 'conexipro',
      script: './target/release/conexipro',
      exec_mode: 'cluster',
      autorestart: true,
      instances: 1,
      interpreter: 'none'
    },
  ],
}
