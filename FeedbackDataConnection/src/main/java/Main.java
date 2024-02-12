import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.List;

public class Main {
	public static void main(String[] args) {
		System.out.println("Starting Server");

		try (ServerSocket server = new ServerSocket(8080)) {

			// noinspection InfiniteLoopStatement
			while(true) {
				Socket sock = server.accept();
				new Thread(() -> logic(sock)).start();
			}

		} catch(IOException e) {
			throw new RuntimeException(e);
		}
	}

	private static synchronized void logic(Socket sock) {
		try (BufferedReader reader = new BufferedReader(new InputStreamReader(sock.getInputStream()));
		     BufferedWriter writer = new BufferedWriter(
					 new FileWriter("/feedback/feedback.txt", true))) {

			writer.write("-".repeat(50));
			writer.newLine();

			DateTimeFormatter dtf = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
			LocalDateTime now = LocalDateTime.now();
			writer.write(dtf.format(now));

			writer.newLine();

			List<String> lines = new ArrayList<>();
			String read;
			while((read = reader.readLine()) != null) lines.add(read);

			for(String line : lines) {
				writer.write(line);
				writer.newLine();
			}
			writer.write("-".repeat(50));
			writer.newLine();
			writer.newLine();
        	writer.flush();

		} catch(IOException e) {
			throw new RuntimeException(e);
		}
	}
}