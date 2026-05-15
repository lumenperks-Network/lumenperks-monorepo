import { Router } from "express";

const router = Router();

// GET /api/customers - list all customers
router.get("/", (_req, res) => {
  res.json({ customers: [] });
});

// POST /api/customers - register a customer
router.post("/", (req, res) => {
  const { walletAddress, name } = req.body;
  if (!walletAddress || !name) {
    return res.status(400).json({ error: "walletAddress and name are required" });
  }
  // TODO: persist to DB and call Soroban contract
  res.status(201).json({ walletAddress, name, points: 0 });
});

export default router;
