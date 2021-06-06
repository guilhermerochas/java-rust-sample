public class App {
    public static void main(String[] args) {
        System.out.println("Calling from rust: ");
        JniBridge bridge = new JniBridge();

        System.out.println(bridge.helloRust("Guilherme"));
        System.out.println(bridge.sumNumbers(4, 5));
    }
}
