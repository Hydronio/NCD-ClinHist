// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{Serialize, Deserialize};
use near_sdk::{log, near_bindgen};
use near_sdk::collections::UnorderedMap

[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FichaClinica {
    motivo_de_consulta: String,
    sintomas: String,
    examen_fisico: String,
    examen_visual: String,
    motivo_de_ultima_consulta: String,
    enfermedades: String.
    tratamiento: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FichaClinicaView {
    id_ficha: u64,
    motivo_de_consulta: String,
    sintomas: String,
    examen_fisico: String,
    examen_visual: String,
    motivo_de_ultima_consulta: String,
    enfermedades: String.
    tratamiento: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Paciente {
    user_id: String,
    name: String,
    last_name: String,
    sexo: String,
    fecha_de_nacimiento: String,
    phone: String,
    email: String,
    country: String,
    ficha: Vec<FichaClinica>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Doctor {
    id: u64,
    user_id: string,
    name: String,
    last_name: String,
    sexo: String,
    fecha_de_nacimiento: String,
    phone: String,
    email: String,
    country: String,
    especialidad: string,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, PanicDefualt)]
#[serde(crate = "near_sdk::serde")]
pub struct ClinHist {
    pub pacientes: Vec<Paciente>,
    pub doctores: Vec<Doctor>,
    pub fichas: UnorderedMap<u64, FichaClinica>,
    pub id_ficha: u64
}

#[near_bindgen]
impl ClinHist {

    // toma como argumento un struct Paciente, asigna el user_id desde las variables de entorno de
    // NEAR, hace .push de esta información 
    pub fn create_paciente(&mut self, mut pacient: Paciente) -> Paciente {
        pacient.user_id = env::signer_account_id().to_string(); 
        
        self.pacientes.push(pacient);
        pacient
    }

    pub fn create_ficha_clinica(&mut self, mut ficha: FichaClinica) -> FichaClinica {
        self.id_ficha += 1;
        
        self.fichas.insert(&self.id_ficha, &ficha);
        ficha
    }

    pub fn get_paciente(self, 
        user_id: Option<AccountId>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Paciente> {
        if self.pacientes.len() > 0 {
            let start_index: u128 = from_index.map(From::from).unwrap_or_default();
            assert!(
                (self.paciente.len() as u128) > start_index,
                "Out of bounds, please use a smaller from_index."
            );
            let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
            assert_ne!(limit, 0, "Cannot provide limit of 0.");

            if user_id.is_some() {
                let user = user_id.unwrap().clone();
                self.paciente.iter().filter(|x| x.user_id == user.to_string())
                .skip(start_index as usize)
                .take(limit)
                .map(|x| x.clone()).collect()
            } else {
                self.paciente.iter()
                .skip(start_index as usize)
                .take(limit)
                .map(|x| x.clone()).collect()
            }
        } else {
            [].to_vec()
        }
    }

    pub get_ficha_clinica(self, 
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
    }

    pub get_fichas_clinica(self, 
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<FichaClinicaView> {
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            (self.paciente.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        self.fichas.iter()
            .skip(start_index)
            .take(limit)
            .map(|(k, v)| {
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
    }

}
