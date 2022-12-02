// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};

use near_sdk::collections::UnorderedMap;

//use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FichaClinica {
    motivo_de_consulta: String,
    sintomas: String,
    examen_fisico: String,
    examen_visual: String,
    motivo_de_ultima_consulta: String,
    enfermedades: String,
    tratamiento: String,
}


//#[derive(Serialize, Deserialize, Clone)]
//#[serde(crate = "near_sdk::serde")]
/*#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Paciente {
    user_id: AccountId, //NEARWallet
    //name: String,
    //last_name: String,
    //sexo: String,
    //fecha_de_nacimiento: String,
    //phone: String,
    //email: String,
    //country: String,
}*/

/*#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Doctor {
    user_id: AccountId, //nearWallet
    //name: String,
    //last_name: String,
    //sexo: String,
    //fecha_de_nacimiento: String,
    //phone: String,
    //email: String,
    //country: String,
    //especialidad: String,
}*/


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ClinHist {
    //pub pacientes: UnorderedSet<AccountId>,
    //pub doctores: UnorderedSet<AccountId>,
    pub fichas: UnorderedMap<AccountId, UnorderedMap<u64, FichaClinica>>,
}

//inicializamos nuestras variables
/*impl Default for ClinHist {
    fn default() -> self{
    pacientes: UnorderedMap::new(),
    doctores: UnorderedMap::new(),
    fichas: UnorderedMap::new(),//si no inicializa probar con: b'S'
    }
}*/


#[near_bindgen]
impl ClinHist {

    //inicializa el contrato
    #[init]
    pub fn new() -> Self {
        Self {
            //pacientes: UnorderedSet::new(b'S'),
            //doctores: UnorderedSet::new(b'S'),
            fichas: UnorderedMap::new(b'S'),//si no inicializa probar con: b'S'
        }
    }




    // toma como argumento un struct Paciente, asigna el user_id desde las variables de entorno de
    // NEAR, hace .push de esta información
    /*pub fn create_paciente(&mut self, user_id: AccountId) -> AccountId {
        self.pacientes.insert(&user_id);
        user_id
    }

    //toma como parametro un struc Doctor
    pub fn create_doctor(&mut self, user_id: AccountId) -> AccountId {    
        self.doctores.insert(&user_id);
        user_id
    }*/

    pub fn create_ficha_clinica(&mut self, 
        paciente_id: AccountId, 
        ficha: FichaClinica
    ) -> FichaClinica {
        let fichas = self.fichas.get(&paciente_id);
        let mut data_ficha: UnorderedMap<u64, FichaClinica> = UnorderedMap::new(b'S');

        if fichas.is_some() {
            data_ficha = fichas.unwrap();
        }

        data_ficha.insert(&(data_ficha.len()+1 as u64), &ficha);

        self.fichas.insert(&paciente_id, &data_ficha);
        ficha
    }

    pub fn get_fichas_clinica(self, paciente_id: AccountId) -> Vec<FichaClinica> {
        let fichas = self.fichas.get(&paciente_id).expect("No exist el paciente solicitado");
        
        fichas.iter()
            .map(|(_k, v)| { return v.clone() }).collect()
        
    }

    /*pub get_doctor(self, 
        id_ficha: u64
    ) -> Doctor {
        const respuesta = self.doctores.get(&id_ficha).expect("el doctor solicitado no existe");
        respuesta.map(|(k, v)| {
            return FichaClinicaView {
                id_doctor: *k,
                //name: v.name,
                //last_name: v.last_name,
                //sexo: v.sexo,
                //fecha_de_nacimiento: v.fecha_de_nacimiento,
                //phone: v.phone,
                //email: v.email,
                //country: v.country,
                //especialidad: v.especialidad,
            }
        }).collect()
    }*/

    //metodo para consultar una (1) ficha, toma como parametro el id de la ficha
    /*pub get_ficha_clinica(self, 
        id_ficha: u64
    ) -> FichaClinicaView {
        const respuesta = self.fichas.get(&id_ficha).expect("la ficha solicitada no existe");
        respuesta.map(|(k, v)| {
            return FichaClinicaView {
                id_ficha: *k,
                motivo_de_consulta: v.motivo_de_consulta,
                sintomas: v.sintomas,
                examen_fisico: v.examen_fisico,
                examen_visual: v.examen_visual,
                motivo_de_ultima_consulta: v.motivo_de_ultima_consulta,
                enfermedades: v.enfermedades.
                tratamiento: v.tratamiento,
            }
        }).collect()
    }*/

    //metodo para hacer una consulta paginada

}
