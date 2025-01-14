package redisClient

import (
	"context"
	"os"
	"strings"

	"github.com/redis/go-redis/v9"
	"github.com/rs/zerolog/log"
	"github.com/x-sushant-x/Promptify/backend/utils"
)

var (
	ctx = context.Background()
)

const (
	REDIS_CLUSTER_USER_ENV = "REDIS_CLUSTER_USER"
	REDIS_CLUSTER_PASS_ENV = "REDIS_CLUSTER_PASS"
	REDIS_CLUSTER_URLS_ENV = "REDIS_CLUSTER_URLS"
)

func createNewRedisConnection(addr, password string) *redis.Client {
	client := redis.NewClient(&redis.Options{
		Addr:     addr,
		Password: password,
	})

	ping(client)

	return client
}

func createRedisClusterConnection() *redis.ClusterClient {
	addrs, user, pass := getRedisClusterCredentials()

	client := redis.NewClusterClient(&redis.ClusterOptions{
		Addrs:    addrs,
		Username: user,
		Password: pass,
	})

	ping(client)

	return client
}

// Addrs, Username, Password
func getRedisClusterCredentials() ([]string, string, string) {
	clusterURLs := os.Getenv(REDIS_CLUSTER_URLS_ENV)

	addrs := strings.Split(clusterURLs, ",")

	if len(clusterURLs) == 0 {
		log.Fatal().Err(utils.ErrInvalidRedisClusterUrls).Msg("please check redis cluster urls")
	}

	user := os.Getenv(REDIS_CLUSTER_USER_ENV)
	pass := os.Getenv(REDIS_CLUSTER_PASS_ENV)

	if user == "" || pass == "" {
		log.Fatal().Err(utils.ErrNullRedisClusterCredentials).Msg("please specify REDIS_CLUSTER_USER and REDIS_CLUSTER_PASS in env")
	}

	return addrs, user, pass
}

func ping[C Pinger](client C) {
	result, err := client.Ping(ctx).Result()
	if err != nil || result == "" {
		log.Fatal().Err(err).Msg("unable to connect to redis cluster")
	}
}
