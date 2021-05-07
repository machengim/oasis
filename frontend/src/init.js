let server = 'http://localhost:3000/';
fetch(server + 'config/')
    .then(res => {
        if (res.status !== 200) {
            window.location.href = '/pages/500';
        }
        res.json().then(
            v => {
                console.log(v);
                if (v.firstRun) {
                    window.location.href = '/setup';
                }
            }
        )
    })
    .catch(e => {
        console.error('Cannot read configuration from server: ' + e);
        window.location.href = '/pages/500';
    }); 