import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
}

kotlin.compilerOptions.jvmTarget = JvmTarget.fromTarget(target = libs.versions.javaVersion.get())

val javaVersion = JavaVersion.values()[libs.versions.javaVersion.get().toInt() - 1]

android {
    compileSdk = libs.versions.compileSdk.get().toInt()
    defaultConfig {
        minSdk = libs.versions.minSdk.get().toInt()
        targetSdk = libs.versions.targetSdk.get().toInt()
    }
    compileOptions {
        sourceCompatibility = javaVersion
        targetCompatibility = javaVersion
    }

    namespace = "ellegard.hans.adornofilter"

    defaultConfig {
        applicationId = "ellegard.hans.adornofilter"
        versionCode = 1
        versionName = "1.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = true
        }
    }
}

dependencies {
}
