use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ktestd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KTESTD, operand1: Some(Direct(K5)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 153, 237], OperandSize::Dword)
}

fn ktestd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KTESTD, operand1: Some(Direct(K3)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 153, 217], OperandSize::Qword)
}

