package redisClient

import (
	"context"

	"github.com/redis/go-redis/v9"
)

type Pinger interface {
	Ping(ctx context.Context) *redis.StatusCmd
}
