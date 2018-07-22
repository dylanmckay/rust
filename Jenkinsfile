pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                mkdir -p build/
                cd build/

                echo 'Building..'
                ../x.py build
            }
        }
    }
}

