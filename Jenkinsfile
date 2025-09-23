node('rust') {
    stage('Checkout') {
        git url: 'https://devcentral.nasqueron.org/source/limiting-factor.git', branch: 'main'
    }

    try {
        stage('Toolchain') {
            sh 'rustup override set nightly-2024-12-15'
        }

        stage('Build')  {
            sh 'cd rocket-legacy && cargo build'
        }

        stage('Doc') {
            sh 'cd rocket-legacy && cargo doc --no-deps'
        }
    } finally {
         archiveArtifacts artifacts: 'rocket-legacy/target/doc/**', onlyIfSuccessful: true
    }
}
