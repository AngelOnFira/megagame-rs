job "megagame-rs" {
  datacenters = ["scs"]

  group "megagame-rs" {
    restart {
      attempts = 3
      delay    = "30s"
    }

    task "megagame-rs-rust" {
      driver = "docker"

      config {
        image = "ghcr.io/angelonfira/megagame-rs/megagame-serenity-bot:latest"
      }

      resources {
        cpu    = 64
        memory = 64
      }

      template {
        data = <<EOH
DISCORD_TOKEN="{{ key "megagame-rs-discord-api-key" }}"
APPLICATION_ID="{{ key "megagame-rs-applicaiton-id" }}"
EOH

        destination = "secrets/file.env"
        env         = true
      }
    }
  }
}