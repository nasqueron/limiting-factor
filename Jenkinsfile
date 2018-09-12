node('rust') {
    stage('Checkout') {
        git 'https://devcentral.nasqueron.org/source/limiting-factor.git'
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
