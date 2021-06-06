import com.sun.jna.Library;

public interface Jni extends Library {
    String hello_rust(String name);
    int sum_numbers(int num1, int num2);
}
