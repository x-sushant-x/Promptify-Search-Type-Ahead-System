package utils

import "errors"

var (
	ErrInvalidRedisClusterUrls = errors.New("invalid redis cluster urls in environment variables")
)
