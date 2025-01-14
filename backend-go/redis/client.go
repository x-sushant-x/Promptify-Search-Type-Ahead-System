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

func CreateNewRedisConnection(addr, password string) *redis.Client {
	client := redis.NewClient(&redis.Options{
		Addr:     addr,
		Password: password,
	})

	ping(client)

	return client
}

func ConnectToRedisCluster() *redis.ClusterClient {
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

	if len(clusterURLs) == 0 {
		log.Fatal().Err(utils.ErrInvalidRedisClusterUrls).Msg("please check redis cluster urls")
	}

	addrs := strings.Split(clusterURLs, ",")
	validateClusterAddrs(addrs)

	user := os.Getenv(REDIS_CLUSTER_USER_ENV)
	pass := os.Getenv(REDIS_CLUSTER_PASS_ENV)

	if user == "" || pass == "" {
		log.Fatal().Err(utils.ErrNullRedisClusterCredentials).Msg("please specify REDIS_CLUSTER_USER and REDIS_CLUSTER_PASS in env")
	}

	return addrs, user, pass
}

func validateClusterAddrs(addrs []string) error {
	for _, addr := range addrs {
		if !strings.Contains(addr, ":") {
			log.Fatal().Err(utils.ErrInvalidRedisClusterUrls).Msg("correct format for cluster address is IP:PORT")
		}
	}

	return nil
}

func ping[C Pinger](client C) {
	result, err := client.Ping(ctx).Result()
	if err != nil || result == "" {
		log.Fatal().Err(err).Msg("unable to connect to redis cluster")
	}
}
