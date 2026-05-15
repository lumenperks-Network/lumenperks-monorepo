import express from "express";
import cors from "cors";
import dotenv from "dotenv";
import customersRouter from "./routes/customers";

dotenv.config();

const app = express();
const PORT = process.env.PORT || 4000;

app.use(cors());
app.use(express.json());

app.get("/health", (_req, res) => res.json({ status: "ok" }));
app.use("/api/customers", customersRouter);

app.listen(PORT, () => console.log(`Backend running on port ${PORT}`));

export default app;
