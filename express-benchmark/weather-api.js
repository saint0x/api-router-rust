const axios = require('axios');

// Direct API call with retries
async function getWeatherDirect(params, maxRetries = 3) {
    console.log('Making direct API call...');
    for (let i = 0; i < maxRetries; i++) {
        try {
            const response = await axios.get('https://api.open-meteo.com/v1/forecast', {
                params,
                timeout: 30000  // 30 second timeout
            });
            return response.data;
        } catch (error) {
            if (i === maxRetries - 1) throw error;
            console.log('Retrying direct call...');
            await new Promise(resolve => setTimeout(resolve, 2000));
        }
    }
}

// @zap decorated call
async function getWeatherWithZap(params) {
    console.log('Making @zap decorated call...');
    const response = await axios.get('http://localhost:3003/@zap/api.open-meteo.com/v1/forecast', {
        params,
        timeout: 30000
    });
    return response.data;
}

// Run benchmark
async function runBenchmark() {
    // Single location - Berlin
    const params = {
        latitude: 52.52,
        longitude: 13.41,
        current: 'temperature_2m'  // Minimal data for testing
    };

    console.log('\nMaking direct API call...');
    const directStart = Date.now();
    try {
        await getWeatherDirect(params);
        const directTime = Date.now() - directStart;
        console.log(`Direct API call took ${directTime}ms`);
    } catch (error) {
        console.error('Error in direct call:', error.message);
    }

    // Wait 2 seconds before next call
    await new Promise(resolve => setTimeout(resolve, 2000));

    console.log('\nMaking @zap decorated call...');
    const zapStart = Date.now();
    try {
        await getWeatherWithZap(params);
        const zapTime = Date.now() - zapStart;
        console.log(`@zap decorated call took ${zapTime}ms`);
    } catch (error) {
        console.error('Error in @zap call:', error.message);
    }
}

// Run the benchmark
runBenchmark().catch(console.error);
