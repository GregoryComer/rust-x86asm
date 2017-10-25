use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kandw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDW, operand1: Some(Direct(K2)), operand2: Some(Direct(K3)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 65, 209], OperandSize::Dword)
}

fn kandw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDW, operand1: Some(Direct(K5)), operand2: Some(Direct(K1)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 65, 238], OperandSize::Qword)
}

