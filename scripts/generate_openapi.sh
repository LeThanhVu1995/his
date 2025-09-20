#!/bin/bash
# Generate OpenAPI documentation for all services

set -e

echo "📚 Generating OpenAPI documentation for all services..."

# Array of services
services=(
    "gateway"
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

# Create docs directory
mkdir -p docs/openapi

# Function to generate OpenAPI for a service
generate_openapi() {
    local service=$1
    local service_dir="services/$service"

    if [ -d "$service_dir" ]; then
        echo "📖 Generating OpenAPI for $service..."

        # Check if service has OpenAPI generation capability
        if [ -f "$service_dir/Cargo.toml" ] && grep -q "utoipa" "$service_dir/Cargo.toml"; then
            cd "$service_dir"

            # Generate OpenAPI spec
            if cargo run --bin "$service" -- --openapi > "../../docs/openapi/${service}.yaml" 2>/dev/null; then
                echo "✅ OpenAPI generated for $service"
            else
                echo "⚠️  Failed to generate OpenAPI for $service"
            fi

            cd - > /dev/null
        else
            echo "⚠️  $service doesn't support OpenAPI generation"
        fi
    else
        echo "❌ Service directory not found: $service_dir"
    fi
}

# Generate OpenAPI for all services
for service in "${services[@]}"; do
    generate_openapi "$service"
done

# Generate combined OpenAPI spec
echo "🔗 Generating combined OpenAPI specification..."
if command -v yq &> /dev/null; then
    # Combine all OpenAPI specs into one
    yq eval-all '. as $item ireduce ({}; . * $item)' docs/openapi/*.yaml > docs/openapi/combined.yaml
    echo "✅ Combined OpenAPI specification generated"
else
    echo "⚠️  yq not found, skipping combined OpenAPI generation"
fi

echo "🎉 OpenAPI documentation generation completed!"
echo "📁 Documentation available in: docs/openapi/"
