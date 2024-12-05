
public class Program {
	public static void main(String[] args) {
		float a = 1.2f, b = 1.0f, c = -1.0f, u = 2.6f, v = 2.0f;
		
		System.out.println("By float: " + String.valueOf(a + c * b) + "\t" + String.valueOf(u + c * v));
		
		double ad = 1.2, bd = 1.0;
		double cd = -1.0;
		double ud = 2.6, vd = 2.0;
		
		System.out.println("\tBy double: " + String.valueOf(ad + cd * bd) + "\t" + String.valueOf(ud + cd * vd));
	}
}
