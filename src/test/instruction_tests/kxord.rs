use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kxord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORD, operand1: Some(Direct(K5)), operand2: Some(Direct(K4)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 221, 71, 237], OperandSize::Dword)
}

fn kxord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORD, operand1: Some(Direct(K3)), operand2: Some(Direct(K4)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 221, 71, 217], OperandSize::Qword)
}

