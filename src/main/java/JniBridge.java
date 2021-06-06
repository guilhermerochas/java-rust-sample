import com.sun.jna.Native;

public class JniBridge {
    private static final Jni jniInstance;

    static {
        jniInstance = Native.load("/libjava_native.so", Jni.class);
    }

    public String helloRust(String name) {
        return jniInstance.hello_rust(name);
    }

    public int sumNumbers(int num1, int num2) {
        return jniInstance.sum_numbers(num1, num2);
    }
}
