workspace MilMoveTasks "The ECS Scheduled Tasks for MilMove" {

    model {
      s = softwareSystem "MilMove ECS Scheduled Tasks" {
        s3 = container "Advana bucket" {
          tags "Amazon Web Services - Simple Storage Service S3 Bucket with Objects"
        }

        dms_export_user = container "RDS Super User" {
          tags "Amazon Web Services - Systems Manager Parameter Store"
        }

        dms_service = container "DMS Task" "This container is made up of multiple components.\n (Steps 5-8)" {
            tags "Amazon Web Services - Database Migration Service Database Migration Workflow"
            dms_source_endpoint = component "Source Endpoint" {
              tags "Amazon Web Services - VPC Endpoints"
            }
            dms_migration_task = component "DMS Migration Task" {
              tags "Amazon Web Services - Database Migration Service Database Migration Workflow"
            }
            dms_target_endpoint = component "Target Endpoint" {
              tags "Amazon Web Services - VPC Endpoints"
            }
          }

        database = container "Database" {
          tags "Amazon Web Services - RDS PostgreSQL instance"
        }

        dms_ecs_task = container "Custom DMS Task" "This container is made up of multiple components.\n (Steps 1-3, 9-10)" {
          tags "Amazon Web Services - Elastic Container Service Container1"
          dms_ecs_task_update_unique_superuser = component "MilMove Task - Update DMS User" {
            tags "Amazon Web Services - Elastic Container Service Container2"
          }
          dms_ecs_task_enable_superuser = component "MilMove Task - Enable DMS User in RDS" {
            tags "Amazon Web Services - Elastic Container Service Container2"
          }
          dms_ecs_task_disable_superuser = component "MilMove Task - Disable DMS User in RDS" {
            tags "Amazon Web Services - Elastic Container Service Container2"
          }
        }
      }

      dms_ecs_task_update_unique_superuser -> dms_export_user "1: Create the dms_export unique username and password"
      dms_export_user -> dms_ecs_task_enable_superuser "2: Retrieve the username and password"
      dms_ecs_task_enable_superuser -> database "3: Enable the rds_superuser"

      dms_ecs_task -> dms_service "4: Triggers the start of the DMS migration task"

      database -> dms_source_endpoint "5: Extracts data from RDS"
      dms_source_endpoint -> dms_migration_task "6: Sends RDS datasets to DMS from Source Endpoint"
      dms_migration_task -> dms_target_endpoint "7: Sends DMS transformed data to Target Endpoint"
      dms_target_endpoint -> s3 "8: Exports data as CSV"

      dms_ecs_task_disable_superuser -> database "9: Disable the rds_superuser"
      dms_ecs_task_update_unique_superuser -> dms_export_user "10: Delete the dms_export username and password"

      atoEnvironment = deploymentEnvironment "ATO Environment" {
        deploymentNode "AWS GovCloud - Truss" {
          tags "Amazon Web Services - Cloud"
          deploymentNode "US-Gov-West-1 " {
            tags "Amazon Web Services - Region"
            deploymentNode "AWS RDS" {
              tags "Amazon Web Services - RDS"
              containerInstance database
            }
            deploymentNode "AWS ECS" {
              tags "Amazon Web Services - Elastic Container Service"
              deploymentNode "Fargate Engine" {
                tags "Amazon Web Services - Fargate"
                deploymentNode "ECS Scheduled Tasks" {
                    tags "Amazon Web Services - Elastic Container Service Service"
                    containerInstance dms_ecs_task
                }
              }
            }
            deploymentNode "AWS DMS" {
              tags "Amazon Web Services - Database Migration Service"
              containerInstance dms_service
            }
            deploymentNode "AWS SSM" {
              tags "Amazon Web Services - Systems Manager"
              containerInstance dms_export_user
            }
          }
        }
        deploymentNode "AWS GovCloud - Advana" {
          tags "Amazon Web Services - Cloud"
          deploymentNode "US-Gov-West-1 " {
            deploymentNode "AWS S3" {
              tags "Amazon Web Services - Simple Storage Service S3"
              containerInstance s3
            }
          }
        }
      }
    }

    views {

      component dms_ecs_task {
        include *
        autoLayout lr
      }

      component dms_service {
        include *
        autoLayout lr
      }

      deployment s atoEnvironment {
        include *
        autoLayout lr
      }

      styles {
        element "Element" {
          shape RoundedBox
          background #ffffff
          color #000000
        }
      }

      theme https://static.structurizr.com/themes/amazon-web-services-2020.04.30/theme.json
    }
}
