use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kandnw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDNW, operand1: Some(Direct(K3)), operand2: Some(Direct(K7)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 66, 223], OperandSize::Dword)
}

fn kandnw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDNW, operand1: Some(Direct(K4)), operand2: Some(Direct(K6)), operand3: Some(Direct(K2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 66, 226], OperandSize::Qword)
}

