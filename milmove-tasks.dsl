workspace MilMoveTasks "The ECS Scheduled Tasks for MilMove" {

    model {
        u = person "User"
        s = softwareSystem "Software System" {
          // MilMove Tasks taken from https://github.com/transcom/mymove/tree/master/cmd/milmove-tasks
          connect_to_gex_via_sftp = container "Connect to GEX sFTP"
          post_file_to_gex = container "Post file to GEX"
          process_edis = container "Process EDIs"
          save_ghc_fuel_price_data = container "Save GHC Fuel Price Data"
          send_payment_reminder_email = container "Send Payment Reminder Email"
          send_post_move_survey_email = container "Send Post Move Survey Email"

          database = container "MilMove Database"
        }

        u -> s "Uses"



        nonatoEnv = deploymentEnvironment "Non-ATO Environment" {
            deploymentNode "Amazon Web Services - GovCloud" {
                tags "Amazon Web Services - GovCloud"
                deploymentNode "US-Gov-West-1" {
                    tags "Amazon Web Services - Region"

                    deploymentNode "Amazon ECS" {
                        tags "Amazon Web Services - ECS"

                    ecs = infrastructureNode "Elastic Container Service" {
                        tags "Amazon Web Services - Elastic Container Service"
                    }


                        deploymentNode "MilMove Scheduled Tasks" {
                            tags "Amazon Web Services - ECS Scheduled Tasks"
                            containerInstance connect_to_gex_via_sftp
                            containerInstance post_file_to_gex
                            containerInstance process_edis
                            containerInstance save_ghc_fuel_price_data
                            containerInstance send_payment_reminder_email
                            containerInstance send_post_move_survey_email
                          }
                      }

                    deploymentNode "Amazon RDS" {
                        tags "Amazon Web Services - RDS"

                        deploymentNode "PostgreSQL" {
                            tags "Amazon Web Services - RDS PostgreSQL instance"

                            containerInstance database
                        }
                    }
                  }
              }
          }



    }

    views {
        /* systemContext s "milmove_ecs_tasks" { */
        /*     include * */
        /*     autoLayout lr */
        /* } */

        deployment s nonatoEnv {
            include *
            autoLayout lr
          }

        theme https://static.structurizr.com/themes/amazon-web-services-2020.04.30/theme.json
    }
    
}
