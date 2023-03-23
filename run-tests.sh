#!/usr/bin/bash

set -x

mkdir reports 

LOCAL_IP=$(curl -s http://169.254.169.254/latest/meta-data/local-ipv4) 

taskset -c 1-4 bin/receive -c ${LOCAL_IP}:3000 -n 1000000  > reports/1_blocking.csv &
sleep 10
taskset -c 5-7 bin/send -c ${LOCAL_IP}:3000 -s ${LOCAL_IP}:3001 -n 1000000 --core 1
fg

taskset -c 1-4 bin/receive -c ${LOCAL_IP}:3000 -n 1000000 --core 1 > reports/1_blocking_affinity.csv &
sleep 10
taskset -c 5-7 bin/send -c ${LOCAL_IP}:3000 -s ${LOCAL_IP}:3001 -n 1000000 --core 1
fg

taskset -c 1-4 bin/receive-coop -c ${LOCAL_IP}:3000 -n 1000000 --non-blocking > reports/2_cooperative.csv &
sleep 10
taskset -c 5-7 bin/send -c ${LOCAL_IP}:3000 -s ${LOCAL_IP}:3001 -n 1000000 --core 1
fg

taskset -c 1-4 bin/receive-coop -c ${LOCAL_IP}:3000 -n 1000000 --non-blocking --core 1 > reports/2_cooperative_affinity.csv &
sleep 10
taskset -c 5-7 bin/send -c ${LOCAL_IP}:3000 -s ${LOCAL_IP}:3001 -n 1000000 --core 1
fg

taskset -c 1-4 bin/receive -c ${LOCAL_IP}:3000 -n 1000000 --non-blocking > reports/3_busy_loop.csv &
sleep 10
taskset -c 5-7 bin/send -c ${LOCAL_IP}:3000 -s ${LOCAL_IP}:3001 -n 1000000 --core 1
fg

taskset -c 1-4 bin/receive -c ${LOCAL_IP}:3000 -n 1000000 --non-blocking --core 1 > reports/3_busy_loop_affinity.csv &
sleep 10
taskset -c 5-7 bin/send -c ${LOCAL_IP}:3000 -s ${LOCAL_IP}:3001 -n 1000000 --core 1
fg

taskset -c 1-4 chrt -f 99 bin/receive -c ${LOCAL_IP}:3000 -n 1000000 --non-blocking --core 1 > reports/3_busy_loop_affinity_rt.csv &
sleep 10
taskset -c 5-7 bin/send -c ${LOCAL_IP}:3000 -s ${LOCAL_IP}:3001 -n 1000000 --core 1
fg

tar zcvf reports.tgz reports/*
