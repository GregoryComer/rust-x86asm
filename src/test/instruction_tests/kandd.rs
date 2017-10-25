use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDD, operand1: Some(Direct(K5)), operand2: Some(Direct(K3)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 229, 65, 235], OperandSize::Dword)
}

fn kandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDD, operand1: Some(Direct(K3)), operand2: Some(Direct(K3)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 229, 65, 223], OperandSize::Qword)
}

