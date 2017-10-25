use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kaddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDB, operand1: Some(Direct(K5)), operand2: Some(Direct(K5)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 74, 237], OperandSize::Dword)
}

fn kaddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDB, operand1: Some(Direct(K6)), operand2: Some(Direct(K5)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 74, 245], OperandSize::Qword)
}

