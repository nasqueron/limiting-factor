node('rust') {
    stage('Checkout') {
        git url: 'https://devcentral.nasqueron.org/source/limiting-factor.git', branch: 'main'
    }

    try {
        stage('Build')  {
            sh 'cargo build'
        }

        stage('Doc') {
            sh 'cargo doc --no-deps'
        }
    } finally {
         archiveArtifacts artifacts: 'target/doc/**', onlyIfSuccessful: true
    }
}
