package main

import (
	"os"
	"time"

	"github.com/joho/godotenv"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	redisClient "github.com/x-sushant-x/Promptify/backend/redis"
)

func init() {
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr, TimeFormat: time.RFC3339})
	zerolog.SetGlobalLevel(zerolog.DebugLevel)

	loadEnv()
}

func loadEnv() {
	if err := godotenv.Load(); err != nil {
		log.Err(err).Msg("unable to read env file")
		os.Exit(-1)
	}
}

func main() {
	redisClient.ConnectToRedisCluster()
}
