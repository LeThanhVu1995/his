#!/bin/bash
# Seed all services with initial data

set -e

echo "🌱 Seeding all services with initial data..."

# Array of services to seed
services=(
    "iam-service"
    "master-data-service"
    "patient-service"
    "scheduling-service"
    "emr-service"
    "lis-service"
    "ris-pacs-service"
    "pharmacy-service"
    "inventory-service"
    "billing-service"
    "insurance-service"
    "or-cssd-service"
    "blood-bank-service"
    "notify-service"
    "workflow-service"
    "reporting-service"
    "iot-service"
    "dms-service"
    "search-service"
    "audit-service"
)

# Function to seed a service
seed_service() {
    local service=$1
    local service_dir="services/$service"

    if [ -d "$service_dir" ]; then
        echo "📦 Seeding $service..."
        if [ -f "$service_dir/scripts/seed.sh" ]; then
            cd "$service_dir"
            ./scripts/seed.sh
            cd - > /dev/null
            echo "✅ $service seeded successfully"
        else
            echo "⚠️  No seed script found for $service"
        fi
    else
        echo "❌ Service directory not found: $service_dir"
    fi
}

# Seed all services
for service in "${services[@]}"; do
    seed_service "$service"
done

echo "🎉 All services seeded successfully!"
