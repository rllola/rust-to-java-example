class HelloWorld {
    // This declares that the static `hello_world` method will be provided
    // a native library.
    private static native void hello_world();

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("hello_world");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        HelloWorld.hello_world();
    }
}
