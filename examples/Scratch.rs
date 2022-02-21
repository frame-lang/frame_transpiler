// emitted from framec_v0.4.0


type FrameState = fn(&mut NexusModelWorkflow, &mut FrameEvent);

enum FrameEventParameter {
    None,
}

enum FrameEventReturn {
    None,
}

struct FrameEvent {
    message: FrameMessage,
    parameters:Option<FrameParameters>,
    ret:FrameEventReturn,
}

impl FrameEvent {
    fn new(message:FrameMessage, parameters:Option<FrameParameters>) -> FrameEvent {
        FrameEvent {
            message,
            parameters,
            ret:FrameEventReturn::None,
        }
    }
}

pub struct NexusModelWorkflow {
    state:FrameState,
    
    //===================== Domain Block ===================//
    
    taskId:<?>,
    sessionName:<?>,
    modelName:<?>,
    modelJson:<?>,
}

impl NexusModelWorkflow {
    fn new() -> NexusModelWorkflow {
        
        NexusModelWorkflow {
            state:NexusModelWorkflow::start_state,
            taskId:null,
            sessionName:null,
            modelName:null,
            modelJson:null,
        }
    }
    
    
    //===================== Machine Block ===================//
    
    fn start_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Create => {
                self.taskId = (e.parameters.as_ref().unwrap().get_create_taskId());
                self.modelName = (e.parameters.as_ref().unwrap().get_create_modelName());
                self.transition(NexusModelWorkflow::createnexusmodel_state);
                return;
            },
            FrameMessage::CreateAndLoadSessionModel => {
                self.taskId = (e.parameters.as_ref().unwrap().get_createAndLoadSessionModel_taskId());
                self.sessionName = (e.parameters.as_ref().unwrap().get_createAndLoadSessionModel_sessionName());
                self.modelName = (e.parameters.as_ref().unwrap().get_createAndLoadSessionModel_modelName());
                // Create and Load\nSession Model
                self.transition(NexusModelWorkflow::createnexusmodel_state);
                return;
            },
            FrameMessage::Load => {
                self.taskId = (e.parameters.as_ref().unwrap().get_load_taskId());
                self.sessionName = (e.parameters.as_ref().unwrap().get_load_sessionName());
                self.modelName = (e.parameters.as_ref().unwrap().get_load_modelName());
                self.transition(NexusModelWorkflow::getmodelfromneo4j_state);
                return;
            },
            FrameMessage::StartSession => {
                self.taskId = (e.parameters.as_ref().unwrap().get_startSession_taskId());
                self.sessionName = (e.parameters.as_ref().unwrap().get_startSession_sessionName());
                // Start Session
                self.transition(NexusModelWorkflow::startsession_state);
                return;
            },
            _ => {}
        }
    }
    
    fn startsession_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                if (startSession()) {
                    // Started
                    self.transition(NexusModelWorkflow::emptysession_state);
                    return;
                } else {
                    // Error
                    self.transition(NexusModelWorkflow::startsessionfailed_state);
                    return;
                }
                return;
            },
            _ => {}
        }
    }
    
    fn emptysession_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                startPollTimer();
                return;
            },
            FrameMessage::Exit => {
                stopPollTimer();
                return;
            },
            FrameMessage::Tick => {
                if (modelDetected()) {
                    self.transition(NexusModelWorkflow::editinginaurora_state);
                    return;
                }
                return;
            },
            _ => {}
        }
    }
    
    fn startsessionfailed_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                post_startSessionFailed();
                self.transition(NexusModelWorkflow::end_state);
                return;
            },
            _ => {}
        }
    }  //  create a new nexus model in neo4j
    
    
    fn createnexusmodel_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                post_createNewNexusModel();
                return;
            },
            FrameMessage::CreateNewNexusModel_Ack => {
                post_newModelCreated();
                if (haveSession()) {
                    // Have\nSession
                    self.transition(NexusModelWorkflow::getmodelfromneo4j_state);
                    return;
                } else {
                    // No\nSession
                    self.transition(NexusModelWorkflow::end_state);
                    return;
                }
                return;
            },
            _ => {}
        }
    }
    
    fn getmodelfromneo4j_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                post_getModelJsonFromNeo4j();
                return;
            },
            FrameMessage::GetModelJsonFromNeo4j_Ack => {
                self.modelJson = (e.parameters.as_ref().unwrap().get_getModelJsonFromNeo4j_Ack_modelJson());
                // Loaded\nModel
                self.transition(NexusModelWorkflow::loadmodelintoaurora_state);
                return;
            },
            _ => {}
        }
    }
    
    fn loadmodelintoaurora_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                if (!(loadAurora())) {
                    // Failure
                    self.transition(NexusModelWorkflow::loadaurorafailed_state);
                    return;
                }
                // Success
                self.transition(NexusModelWorkflow::editinginaurora_state);
                return;
            },
            _ => {}
        }
    }
    
    fn editinginaurora_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Save => {
                serializeModel();
                post_updateModel();
                return;
            },
            FrameMessage::UpdateSavedCheckedOutModel_Ack => {
                return;
            },
              //  todo
            FrameMessage::UpdateSavedCheckedOutModel_Nack => {
                return;
            },
              //  @todo. Cache JSON somewhere?
            FrameMessage::SaveAndCloseSession => {
                self.transition(NexusModelWorkflow::savemodel_state);
                return;
            },
            FrameMessage::CloseSession => {
                closeAuroraSession();
                self.transition(NexusModelWorkflow::closeaurorasession_state);
                return;
            },
            _ => {}
        }
    }
    
    fn savemodel_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                serializeModel();
                post_updateModel();
                return;
            },
            FrameMessage::UpdateModel_Ack => {
                post_updateModelSucceeded();
                // Model Saved
                self.transition(NexusModelWorkflow::closeaurorasession_state);
                return;
            },
            FrameMessage::UpdateModel_Nack => {
                post_updateModelFailed();
                // Model\nNot Saved
                self.transition(NexusModelWorkflow::closeaurorasession_state);
                return;
            },
            _ => {}
        }
    }  //  @todo. Cache JSON somewhere?
    
    
    fn loadaurorafailed_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                post_loadAuroraFailed();
                self.transition(NexusModelWorkflow::end_state);
                return;
            },
            _ => {}
        }
    }
    
    fn closeaurorasession_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                if (closeAuroraSession()) {
                    post_logCloseAuroraSessionSucceeded();
                } else {
                    post_logCloseAuroraSessionFailed();
                }
                self.transition(NexusModelWorkflow::end_state);
                return;
            },
            _ => {}
        }
    }
    
    fn end_state(&mut self, e:&mut FrameEvent) {
        match e.message {
            
            FrameMessage::Enter => {
                post_workflowComplete();
                terminateSelf();
                return;
            },
            _ => {}
        }
    }
    
    //=============== Machinery and Mechanisms ==============//
    
    
    
    fn transition(&mut self, newState:FrameState) {
        let mut exit_event = FrameEvent::new(FrameMessage::Exit,None);
        (self.state)(self,&mut exit_event);
        self.state = newState;
        let mut enter_event = FrameEvent::new(FrameMessage::Enter,None);
        (self.state)(self,&mut enter_event);
    }
    
}

