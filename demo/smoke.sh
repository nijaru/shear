#!/bin/sh
# Local screen-recording preflight; captures the current display, not a product demo.
set -eu
out=${1:-demo/smoke}
mkdir -p "$out"
if [ -e "$out/capture.mov" ] || [ -e "$out/capture.mp4" ]; then
    echo 'Use a new output directory; refusing to overwrite a recording.' >&2
    exit 1
fi
/usr/sbin/screencapture -x -v -V 10 "$out/capture.mov"
ffmpeg -v error -n -i "$out/capture.mov" -an \
    -vf 'scale=1920:-2' -c:v libx264 -crf 18 -pix_fmt yuv420p \
    -movflags +faststart "$out/capture.mp4"
ffprobe -v error -show_entries \
    format=duration,size:stream=codec_name,width,height,pix_fmt \
    -of json "$out/capture.mp4"
ffmpeg -v error -i "$out/capture.mp4" -f null -
printf '\nCapture and decode passed. Full-size playback/readability review is still required.\n'
