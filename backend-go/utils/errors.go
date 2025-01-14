package utils

import "errors"

var (
	ErrInvalidRedisClusterUrls     = errors.New("invalid redis cluster address in environment variables")
	ErrNullRedisClusterCredentials = errors.New("null redis cluster credentials in environment variables")
)
