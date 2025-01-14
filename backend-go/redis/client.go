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

func createNewRedisConnection(addr, password string) (*redis.Client, error) {
	conn := redis.NewClient(&redis.Options{
		Addr:     addr,
		Password: password,
	})

	result, err := conn.Ping(ctx).Result()
	if err != nil || result == "" {
		log.Fatal().Err(err).Msg("unable to connect to redis instance: " + addr)
	}

	return conn, nil
}

/*
This function require REDIS_CLUSTER_URLS to be set in enviroment config.

	REDIS_CLUSTER_URLS=IP:PORT,IP:PORT
*/
func createRedisClusterConnection() (*redis.ClusterClient, error) {
	clusterURLs := os.Getenv("REDIS_CLUSTER_URLS")

	addrs := strings.Split(clusterURLs, ",")

	if len(clusterURLs) == 0 {
		log.Err(utils.ErrInvalidRedisClusterUrls).Msg("please check redis cluster urls")
	}

	conn := redis.NewClusterClient(&redis.ClusterOptions{
		Addrs: addrs,
	})

	result, err := conn.Ping(ctx).Result()
	if err != nil || result == "" {
		log.Fatal().Err(err).Msg("unable to connect to redis cluster")
	}

	return conn, nil
}
