// Function to parse the CPU data from /proc/stat
function parseCpuData(data) {
    const lines = data.trim().split("\n");
    const cpus = lines.filter(line => line.startsWith("cpu"));
    return cpus.map(cpuLine => {
        const parts = cpuLine.split(/\s+/).slice(1).map(Number);
        return parts;
    });
}

// Function to calculate the CPU usage
function calculateCpuUsage(oldData, newData) {
    const usage = [];
    for (let i = 0; i < oldData.length; i++) {
        const oldTimes = oldData[i];
        const newTimes = newData[i];
        const totalDiff = newTimes.reduce((a, b) => a + b, 0) - oldTimes.reduce((a, b) => a + b, 0);
        const idleDiff = newTimes[3] - oldData[i][3];
        const usagePercent = ((totalDiff - idleDiff) / totalDiff) * 100;
        usage.push(usagePercent.toFixed(2));
    }
    return usage;
}

// Main function to read data, calculate and display CPU usage
async function displayCpuUsage() {
    const firstRead = await Deno.readTextFile("/proc/stat");
    const firstData = parseCpuData(firstRead);
    // Wait for a short interval to get a second sample
    await new Promise(resolve => setTimeout(resolve, 100));
    const secondRead = await Deno.readTextFile("/proc/stat");
    const secondData = parseCpuData(secondRead);

    const usage = calculateCpuUsage(firstData, secondData);
    console.log("CPU Usage (%):", usage);
    let s_sep = '.';
    let s_cpu = '|'
    let s = usage.map(
        n_perc=>{
            return `${s_sep}${s_cpu.repeat(parseInt((n_perc/100)*5)).padEnd(5,' ')}`
        }
    ).join('')
    console.log(s)
}

// Run the main function
displayCpuUsage();
window.setInterval(
    displayCpuUsage, 100
)
