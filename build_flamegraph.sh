# export FLAMEGRAPH_HOME=${HOME}/Documents/projects/rust/FlameGraph
# https://nanxiao.me/en/use-perf-and-flamegraph-to-profile-program-on-linux/
#
if [[ ! -v "${FLAMEGRAPH_HOME}" ]]; then
    echo "Building flamegraph from 'perf.data'..."
    perf script | ${FLAMEGRAPH_HOME}/stackcollapse-perf.pl | ${FLAMEGRAPH_HOME}/flamegraph.pl > perf.svg
    echo "Done, 'perf.svg' saved"
else
    echo "could not find flamegraph home, set 'FLAMEGRAPH_HOME' variable to the approprate folder."
fi